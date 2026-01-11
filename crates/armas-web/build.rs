use std::fs;
use std::path::Path;

fn main() {
    // Tell cargo to rerun if content directory changes
    println!("cargo:rerun-if-changed=content");

    // Read all markdown files from content directory
    let content_dir = Path::new("content");
    if !content_dir.exists() {
        return;
    }

    let mut md_files = Vec::new();
    if let Ok(entries) = fs::read_dir(content_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    md_files.push(stem.to_string());
                }
            }
        }
    }

    // Sort alphabetically
    md_files.sort();

    // Generate showcase module file
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("showcase_generated.rs");

    let mut code = String::new();
    code.push_str("// Auto-generated showcase pages from markdown files\n\n");
    code.push_str("use crate::markdown;\n");
    code.push_str("use armas::*;\n");
    code.push_str("use armas_showcase_macros::showcase_page;\n");
    code.push_str("use eframe::egui;\n\n");

    // Generate a module for each markdown file
    for name in &md_files {
        code.push_str(&format!("pub mod {} {{\n", name.replace("-", "_")));
        code.push_str("    use super::*;\n");
        code.push_str(&format!("    showcase_page!(\"crates/armas-web/content/{}.md\");\n", name));
        code.push_str("}\n\n");
    }

    // Generate the get_pages function
    code.push_str("pub fn get_pages() -> Vec<(&'static str, fn(&mut egui::Ui))> {\n");
    code.push_str("    vec![\n");
    for name in &md_files {
        let display_name = name.replace("-", " ").replace("_", " ");
        let display_name = display_name
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        code.push_str(&format!("        (\"{}\", {}::show as fn(&mut egui::Ui)),\n",
            display_name, name.replace("-", "_")));
    }
    code.push_str("    ]\n");
    code.push_str("}\n\n");

    // Generate get_page_by_name function
    code.push_str("pub fn get_page_by_name(name: &str) -> Option<fn(&mut egui::Ui)> {\n");
    code.push_str("    match name {\n");
    for name in &md_files {
        code.push_str(&format!("        \"{}\" => Some({}::show),\n",
            name, name.replace("-", "_")));
    }
    code.push_str("        _ => None,\n");
    code.push_str("    }\n");
    code.push_str("}\n");

    fs::write(&dest_path, code).unwrap();
}
