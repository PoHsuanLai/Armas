//! Build script to parse SVG icons at compile time
//!
//! This script:
//! 1. Scans the `icons/` directory for SVG files
//! 2. Parses each SVG using usvg
//! 3. Tessellates paths into triangles using Lyon (properly handles holes)
//! 4. Generates Rust code with icon data as constants

use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use lyon_tessellation::{
    BuffersBuilder, FillOptions, FillTessellator, FillVertex, VertexBuffers,
    path::Path as TessPath,
};
use usvg::tiny_skia_path::PathSegment;

fn main() {
    println!("cargo:rerun-if-changed=icons/");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("icon_data.rs");
    let mut output = File::create(&dest_path).unwrap();

    // Start generating the Rust code
    writeln!(
        output,
        "// Generated icon data - DO NOT EDIT MANUALLY\n"
    )
    .unwrap();
    writeln!(output, "use egui::{{epaint::Vertex, Color32, Pos2, Rect, Painter, Mesh}};").unwrap();
    writeln!(output, "").unwrap();

    // Icon data structure
    writeln!(output, "#[derive(Debug, Clone)]").unwrap();
    writeln!(output, "pub struct IconData {{").unwrap();
    writeln!(output, "    pub name: &'static str,").unwrap();
    writeln!(output, "    pub vertices: &'static [(f32, f32)],").unwrap();
    writeln!(output, "    pub indices: &'static [u32],").unwrap();
    writeln!(output, "    pub viewbox_width: f32,").unwrap();
    writeln!(output, "    pub viewbox_height: f32,").unwrap();
    writeln!(output, "}}\n").unwrap();

    // Start the icon registry
    writeln!(output, "pub static TRANSPORT_ICONS: &[(&str, IconData)] = &[").unwrap();

    // Parse transport icons
    let icons_dir = PathBuf::from("icons/transport");
    if icons_dir.exists() {
        for entry in fs::read_dir(&icons_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("svg") {
                let file_name = path.file_stem().unwrap().to_str().unwrap();
                println!("cargo:rerun-if-changed={}", path.display());

                match parse_svg(&path) {
                    Ok(icon_code) => {
                        writeln!(output, "    (\"{}\", {}),", file_name, icon_code).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    writeln!(output, "];\n").unwrap();

    // Helper function to render icons
    writeln!(output, r#"
pub fn render_icon(painter: &Painter, rect: Rect, icon_data: &IconData, color: Color32) {{
    let scale_x = rect.width() / icon_data.viewbox_width;
    let scale_y = rect.height() / icon_data.viewbox_height;
    let scale = scale_x.min(scale_y);

    let offset_x = rect.left() + (rect.width() - icon_data.viewbox_width * scale) / 2.0;
    let offset_y = rect.top() + (rect.height() - icon_data.viewbox_height * scale) / 2.0;

    let mut mesh = Mesh::default();

    // Transform vertices and add to mesh
    for (x, y) in icon_data.vertices {{
        let pos = Pos2::new(
            offset_x + x * scale,
            offset_y + y * scale,
        );
        mesh.vertices.push(Vertex {{
            pos,
            uv: Pos2::ZERO,
            color,
        }});
    }}

    // Add indices
    mesh.indices.extend_from_slice(icon_data.indices);

    painter.add(mesh);
}}
"#).unwrap();

    println!("cargo:rustc-env=ICON_DATA_PATH={}", dest_path.display());
}

fn parse_svg(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let svg_data = fs::read_to_string(path)?;

    // Extract viewBox dimensions from the raw SVG
    let (width, height) = extract_viewbox(&svg_data).unwrap_or((24.0, 24.0));

    // Parse SVG with usvg
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_str(&svg_data, &options)?;

    // Tessellate the SVG paths into triangles
    let mut geometry: VertexBuffers<[f32; 2], u32> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();

    // Extract and tessellate all paths
    tessellate_group(tree.root(), &mut tessellator, &mut geometry)?;

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

    let icon_name = path.file_stem().unwrap().to_str().unwrap();
    let code = format!(
        "IconData {{\n        name: \"{}\",\n        vertices: &[{}],\n        indices: &[{}],\n        viewbox_width: {:.1},\n        viewbox_height: {:.1},\n    }}",
        icon_name,
        vertices_code,
        indices_code,
        width,
        height
    );

    Ok(code)
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
    tessellator: &mut FillTessellator,
    geometry: &mut VertexBuffers<[f32; 2], u32>,
) -> Result<(), Box<dyn std::error::Error>> {
    for node in group.children() {
        tessellate_node(node, tessellator, geometry)?;
    }
    Ok(())
}

fn tessellate_node(
    node: &usvg::Node,
    tessellator: &mut FillTessellator,
    geometry: &mut VertexBuffers<[f32; 2], u32>,
) -> Result<(), Box<dyn std::error::Error>> {
    match node {
        usvg::Node::Path(path) => {
            // Convert usvg path to Lyon path
            let mut builder = TessPath::builder();

            for segment in path.data().segments() {
                match segment {
                    PathSegment::MoveTo(p) => {
                        builder.begin(lyon_tessellation::math::Point::new(p.x, p.y));
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
                    }
                }
            }

            let lyon_path = builder.build();

            // Tessellate with fill rule
            tessellator.tessellate_path(
                &lyon_path,
                &FillOptions::default(),
                &mut BuffersBuilder::new(geometry, |vertex: FillVertex| {
                    [vertex.position().x, vertex.position().y]
                }),
            )?;
        }
        usvg::Node::Group(group) => {
            tessellate_group(group, tessellator, geometry)?;
        }
        _ => {}
    }
    Ok(())
}
