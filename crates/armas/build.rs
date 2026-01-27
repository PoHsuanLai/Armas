//! Build script to parse window SVG icons at compile time
//!
//! This script:
//! 1. Scans the `icons/window/` directory for SVG files
//! 2. Parses each SVG using usvg
//! 3. Tessellates paths into triangles using Lyon
//! 4. Generates Rust code with icon data as constants

use lyon_tessellation::{
    path::Path as TessPath, BuffersBuilder, FillOptions, FillTessellator, FillVertex,
    StrokeOptions, StrokeTessellator, StrokeVertex, VertexBuffers,
};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use usvg::tiny_skia_path::PathSegment;

fn main() {
    println!("cargo:rerun-if-changed=icons/");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("window_icons.rs");
    let mut output = File::create(&dest_path).unwrap();

    // Start generating the Rust code
    // Note: IconData is already imported by the parent module via `pub use armas_icon::IconData`
    writeln!(output, "// Generated window icon data - DO NOT EDIT MANUALLY\n").unwrap();

    // Parse window icons
    let icons_dir = PathBuf::from("icons/window");
    if icons_dir.exists() {
        for entry in fs::read_dir(&icons_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("svg") {
                let file_name = path.file_stem().unwrap().to_str().unwrap();
                println!("cargo:rerun-if-changed={}", path.display());

                // Convert file name to SCREAMING_SNAKE_CASE for constant name
                let const_name = file_name.to_uppercase().replace('-', "_");

                match parse_svg(&path) {
                    Ok((vertices, indices, width, height)) => {
                        writeln!(output, "#[allow(missing_docs)]").unwrap();
                        writeln!(
                            output,
                            "pub static {}: IconData = IconData {{",
                            const_name
                        )
                        .unwrap();
                        writeln!(output, "    name: \"{}\",", file_name).unwrap();
                        writeln!(output, "    vertices: &[{}],", vertices).unwrap();
                        writeln!(output, "    indices: &[{}],", indices).unwrap();
                        writeln!(output, "    viewbox_width: {:.1},", width).unwrap();
                        writeln!(output, "    viewbox_height: {:.1},", height).unwrap();
                        writeln!(output, "}};\n").unwrap();
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    println!("cargo:rustc-env=WINDOW_ICONS_PATH={}", dest_path.display());
}

fn parse_svg(path: &Path) -> Result<(String, String, f32, f32), Box<dyn std::error::Error>> {
    let svg_data = fs::read_to_string(path)?;

    // Extract viewBox dimensions from the raw SVG
    let (width, height) = extract_viewbox(&svg_data).unwrap_or((24.0, 24.0));

    // Parse SVG with usvg
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_str(&svg_data, &options)?;

    // Tessellate the SVG paths into triangles
    let mut geometry: VertexBuffers<[f32; 2], u32> = VertexBuffers::new();
    let mut fill_tessellator = FillTessellator::new();
    let mut stroke_tessellator = StrokeTessellator::new();

    // Extract and tessellate all paths
    tessellate_group(tree.root(), &mut fill_tessellator, &mut stroke_tessellator, &mut geometry)?;

    // Generate Rust code for vertices
    let mut vertices_code = String::new();
    for (i, vertex) in geometry.vertices.iter().enumerate() {
        if i > 0 {
            vertices_code.push_str(", ");
        }
        vertices_code.push_str(&format!("({:.2}, {:.2})", vertex[0], vertex[1]));
    }

    // Generate Rust code for indices
    let mut indices_code = String::new();
    for (i, index) in geometry.indices.iter().enumerate() {
        if i > 0 {
            indices_code.push_str(", ");
        }
        indices_code.push_str(&format!("{}", index));
    }

    Ok((vertices_code, indices_code, width, height))
}

fn extract_viewbox(svg_data: &str) -> Option<(f32, f32)> {
    // Try to extract viewBox attribute first
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

    // Fallback: try to extract width and height attributes
    let width = extract_dimension(svg_data, "width");
    let height = extract_dimension(svg_data, "height");

    if let (Some(w), Some(h)) = (width, height) {
        return Some((w, h));
    }

    None
}

fn extract_dimension(svg_data: &str, attr: &str) -> Option<f32> {
    let pattern = format!("{}=\"", attr);
    if let Some(start) = svg_data.find(&pattern) {
        let value_str = &svg_data[start + pattern.len()..];
        if let Some(end) = value_str.find('"') {
            let value = &value_str[..end];
            // Remove "px" suffix if present
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
) -> Result<(), Box<dyn std::error::Error>> {
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
) -> Result<(), Box<dyn std::error::Error>> {
    match node {
        usvg::Node::Path(path) => {
            // Convert usvg path to Lyon path
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

            // End path if not closed
            if has_begun {
                builder.end(false);
            }

            let lyon_path = builder.build();

            // Check if path has fill
            if path.fill().is_some() {
                fill_tessellator.tessellate_path(
                    &lyon_path,
                    &FillOptions::default(),
                    &mut BuffersBuilder::new(geometry, |vertex: FillVertex| {
                        [vertex.position().x, vertex.position().y]
                    }),
                )?;
            }

            // Check if path has stroke
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
