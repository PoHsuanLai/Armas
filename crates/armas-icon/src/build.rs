//! Build-time SVG tessellation utilities.
//!
//! This module provides functions for tessellating SVG files into [`IconData`](crate::IconData)
//! constants at compile time. Use it in your `build.rs` to generate icon data.
//!
//! # Example
//!
//! In `Cargo.toml`:
//! ```toml
//! [build-dependencies]
//! armas-icon = { path = "...", features = ["build"] }
//! ```
//!
//! In `build.rs`:
//! ```no_run
//! use std::path::Path;
//! use std::fs::File;
//! use std::io::Write;
//!
//! let out_dir = std::env::var("OUT_DIR").unwrap();
//! let dest = Path::new(&out_dir).join("my_icons.rs");
//! let mut output = File::create(&dest).unwrap();
//!
//! writeln!(output, "// Generated icons\n").unwrap();
//!
//! // Tessellate a single SVG into an IconData constant
//! let code = armas_icon::build::generate_icon_constant(
//!     Path::new("assets/icons/sun.svg"),
//!     "SUN",
//! ).unwrap();
//! writeln!(output, "{}", code).unwrap();
//!
//! // Or tessellate an entire directory
//! armas_icon::build::generate_icons_from_dir(
//!     Path::new("assets/icons/"),
//!     &mut output,
//! ).unwrap();
//! ```

use lyon_tessellation::{
    path::Path as TessPath, BuffersBuilder, FillOptions, FillTessellator, FillVertex,
    StrokeOptions, StrokeTessellator, StrokeVertex, VertexBuffers,
};
use std::fs;
use std::io::Write;
use std::path::Path;
use usvg::tiny_skia_path::PathSegment;

/// Result type for build operations.
pub type BuildResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Tessellate a single SVG file and return the Rust code for an `IconData` constant.
///
/// The constant will be named `const_name` in `SCREAMING_SNAKE_CASE`.
///
/// # Errors
///
/// Returns an error if the SVG file cannot be read or parsed.
///
/// # Example
/// ```rust,no_run
/// let code = armas_icon::build::generate_icon_constant(
///     std::path::Path::new("icons/sun.svg"),
///     "SUN",
/// ).unwrap();
/// // code contains: `pub static SUN: IconData = IconData { ... };`
/// ```
pub fn generate_icon_constant(svg_path: &Path, const_name: &str) -> BuildResult<String> {
    use std::fmt::Write;
    let (vertices, indices, width, height) = tessellate_svg(svg_path)?;

    let mut code = String::new();
    code.push_str("#[allow(missing_docs)]\n");
    writeln!(code, "pub static {const_name}: IconData = IconData {{").unwrap();

    let file_name = svg_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");
    writeln!(code, "    name: \"{file_name}\",").unwrap();
    writeln!(code, "    vertices: &[{vertices}],").unwrap();
    writeln!(code, "    indices: &[{indices}],").unwrap();
    writeln!(code, "    viewbox_width: {width:.1},").unwrap();
    writeln!(code, "    viewbox_height: {height:.1},").unwrap();
    code.push_str("};\n");

    Ok(code)
}

/// Tessellate all SVG files in a directory and write `IconData` constants to the output.
///
/// Each SVG file is converted to a constant named after the file in `SCREAMING_SNAKE_CASE`
/// (e.g., `my-icon.svg` → `MY_ICON`).
///
/// # Errors
///
/// Returns an error if the directory cannot be read or if any SVG file cannot be parsed.
///
/// # Panics
///
/// Panics if a file name cannot be converted to UTF-8.
pub fn generate_icons_from_dir(dir: &Path, output: &mut impl Write) -> BuildResult<()> {
    if !dir.exists() {
        return Err(format!("Icon directory does not exist: {}", dir.display()).into());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("svg") {
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            let const_name = file_name.to_uppercase().replace('-', "_");

            println!("cargo:rerun-if-changed={}", path.display());

            match generate_icon_constant(&path, &const_name) {
                Ok(code) => {
                    writeln!(output, "{code}")?;
                }
                Err(e) => {
                    eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                }
            }
        }
    }

    Ok(())
}

/// Tessellate an SVG file into vertex/index data.
///
/// Returns `(vertices_code, indices_code, viewbox_width, viewbox_height)` where
/// the code strings are ready to embed in Rust source.
///
/// # Errors
///
/// Returns an error if the SVG file cannot be read, parsed, or tessellated.
pub fn tessellate_svg(path: &Path) -> BuildResult<(String, String, f32, f32)> {
    use std::fmt::Write;
    let svg_data = fs::read_to_string(path)?;

    let (width, height) = extract_viewbox(&svg_data).unwrap_or((24.0, 24.0));

    let options = usvg::Options::default();
    let tree = usvg::Tree::from_str(&svg_data, &options)?;

    let mut geometry: VertexBuffers<[f32; 2], u32> = VertexBuffers::new();
    let mut fill_tessellator = FillTessellator::new();
    let mut stroke_tessellator = StrokeTessellator::new();

    tessellate_group(
        tree.root(),
        &mut fill_tessellator,
        &mut stroke_tessellator,
        &mut geometry,
    )?;

    // Generate vertices code
    let mut vertices_code = String::new();
    for (i, vertex) in geometry.vertices.iter().enumerate() {
        if i > 0 {
            vertices_code.push_str(", ");
        }
        write!(vertices_code, "({:.2}, {:.2})", vertex[0], vertex[1]).unwrap();
    }

    // Generate indices code
    let mut indices_code = String::new();
    for (i, index) in geometry.indices.iter().enumerate() {
        if i > 0 {
            indices_code.push_str(", ");
        }
        write!(indices_code, "{index}").unwrap();
    }

    Ok((vertices_code, indices_code, width, height))
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
) -> BuildResult<()> {
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
) -> BuildResult<()> {
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
