//! Shared SVG tessellation logic.
//!
//! Used by both the `build` module (compile-time codegen) and the `runtime` module
//! (runtime SVG parsing).

use lyon_tessellation::{
    path::Path as TessPath, BuffersBuilder, FillOptions, FillTessellator, FillVertex,
    StrokeOptions, StrokeTessellator, StrokeVertex, VertexBuffers,
};
use usvg::tiny_skia_path::PathSegment;

/// Tessellated icon geometry.
pub struct TessellatedIcon {
    pub vertices: Vec<(f32, f32)>,
    pub indices: Vec<u32>,
    pub viewbox_width: f32,
    pub viewbox_height: f32,
}

/// Error type for tessellation operations.
#[derive(Debug)]
pub enum TessError {
    /// SVG parsing failed.
    SvgParse(String),
    /// Tessellation failed.
    Tessellation(String),
}

impl std::fmt::Display for TessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SvgParse(e) => write!(f, "SVG parse error: {e}"),
            Self::Tessellation(e) => write!(f, "tessellation error: {e}"),
        }
    }
}

impl std::error::Error for TessError {}

impl From<lyon_tessellation::TessellationError> for TessError {
    fn from(e: lyon_tessellation::TessellationError) -> Self {
        Self::Tessellation(e.to_string())
    }
}


impl From<usvg::Error> for TessError {
    fn from(e: usvg::Error) -> Self {
        Self::SvgParse(e.to_string())
    }
}

/// Tessellate an SVG string into vertex/index geometry.
pub fn tessellate_svg_data(svg_str: &str) -> Result<TessellatedIcon, TessError> {
    let (viewbox_width, viewbox_height) = extract_viewbox(svg_str).unwrap_or((24.0, 24.0));

    let options = usvg::Options::default();
    let tree = usvg::Tree::from_str(svg_str, &options)?;

    let mut geometry: VertexBuffers<[f32; 2], u32> = VertexBuffers::new();
    let mut fill_tessellator = FillTessellator::new();
    let mut stroke_tessellator = StrokeTessellator::new();

    tessellate_group(
        tree.root(),
        &mut fill_tessellator,
        &mut stroke_tessellator,
        &mut geometry,
    )?;

    let vertices = geometry.vertices.iter().map(|v| (v[0], v[1])).collect();

    Ok(TessellatedIcon {
        vertices,
        indices: geometry.indices,
        viewbox_width,
        viewbox_height,
    })
}

fn extract_viewbox(svg_data: &str) -> Option<(f32, f32)> {
    if let Some(viewbox_start) = svg_data.find("viewBox=\"") {
        let viewbox_str = &svg_data[viewbox_start + 9..];
        if let Some(viewbox_end) = viewbox_str.find('"') {
            let viewbox = &viewbox_str[..viewbox_end];
            let parts: Vec<&str> = viewbox.split_whitespace().collect();
            if parts.len() == 4 {
                if let (Ok(_), Ok(_), Ok(width), Ok(height)) = (
                    parts[0].parse::<f32>(),
                    parts[1].parse::<f32>(),
                    parts[2].parse::<f32>(),
                    parts[3].parse::<f32>(),
                ) {
                    return Some((width, height));
                }
            }
        }
    }

    let width = extract_dimension(svg_data, "width");
    let height = extract_dimension(svg_data, "height");

    if let (Some(w), Some(h)) = (width, height) {
        return Some((w, h));
    }

    None
}

fn extract_dimension(svg_data: &str, attr: &str) -> Option<f32> {
    let pattern = format!("{attr}=\"");
    if let Some(start) = svg_data.find(&pattern) {
        let value_str = &svg_data[start + pattern.len()..];
        if let Some(end) = value_str.find('"') {
            let value = &value_str[..end];
            let value = value.trim_end_matches("px");
            return value.parse::<f32>().ok();
        }
    }
    None
}

fn tessellate_group(
    group: &usvg::Group,
    fill_tessellator: &mut FillTessellator,
    stroke_tessellator: &mut StrokeTessellator,
    geometry: &mut VertexBuffers<[f32; 2], u32>,
) -> Result<(), TessError> {
    for node in group.children() {
        tessellate_node(node, fill_tessellator, stroke_tessellator, geometry)?;
    }
    Ok(())
}

fn tessellate_node(
    node: &usvg::Node,
    fill_tessellator: &mut FillTessellator,
    stroke_tessellator: &mut StrokeTessellator,
    geometry: &mut VertexBuffers<[f32; 2], u32>,
) -> Result<(), TessError> {
    match node {
        usvg::Node::Path(path) => {
            let mut builder = TessPath::builder();
            let mut has_begun = false;

            for segment in path.data().segments() {
                match segment {
                    PathSegment::MoveTo(p) => {
                        if has_begun {
                            builder.end(false);
                        }
                        builder.begin(lyon_tessellation::math::Point::new(p.x, p.y));
                        has_begun = true;
                    }
                    PathSegment::LineTo(p) => {
                        builder.line_to(lyon_tessellation::math::Point::new(p.x, p.y));
                    }
                    PathSegment::QuadTo(p1, p2) => {
                        builder.quadratic_bezier_to(
                            lyon_tessellation::math::Point::new(p1.x, p1.y),
                            lyon_tessellation::math::Point::new(p2.x, p2.y),
                        );
                    }
                    PathSegment::CubicTo(p1, p2, p3) => {
                        builder.cubic_bezier_to(
                            lyon_tessellation::math::Point::new(p1.x, p1.y),
                            lyon_tessellation::math::Point::new(p2.x, p2.y),
                            lyon_tessellation::math::Point::new(p3.x, p3.y),
                        );
                    }
                    PathSegment::Close => {
                        builder.end(true);
                        has_begun = false;
                    }
                }
            }

            if has_begun {
                builder.end(false);
            }

            let lyon_path = builder.build();

            if path.fill().is_some() {
                fill_tessellator.tessellate_path(
                    &lyon_path,
                    &FillOptions::default(),
                    &mut BuffersBuilder::new(geometry, |vertex: FillVertex| {
                        [vertex.position().x, vertex.position().y]
                    }),
                )?;
            }

            if let Some(stroke) = path.stroke() {
                let stroke_width = stroke.width().get();
                let stroke_options = StrokeOptions::default()
                    .with_line_width(stroke_width)
                    .with_line_cap(lyon_tessellation::LineCap::Round)
                    .with_line_join(lyon_tessellation::LineJoin::Round);

                stroke_tessellator.tessellate_path(
                    &lyon_path,
                    &stroke_options,
                    &mut BuffersBuilder::new(geometry, |vertex: StrokeVertex| {
                        [vertex.position().x, vertex.position().y]
                    }),
                )?;
            }
        }
        usvg::Node::Group(group) => {
            tessellate_group(group, fill_tessellator, stroke_tessellator, geometry)?;
        }
        _ => {}
    }
    Ok(())
}
