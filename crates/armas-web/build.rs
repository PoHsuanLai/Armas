use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // Tell cargo to rerun if content directory changes
    println!("cargo:rerun-if-changed=content");

    // Read all markdown files from content directory recursively
    let content_dir = Path::new("content");
    if !content_dir.exists() {
        return;
    }

    // Define section order
    let section_order = vec![
        "getting_started",
        "components",
        "effects",
        "examples",
    ];

    let mut sections: Vec<(String, Vec<(String, PathBuf)>)> = Vec::new();

    // Scan each section directory
    for section in &section_order {
        let section_path = content_dir.join(section);
        if !section_path.exists() {
            continue;
        }

        let mut files = Vec::new();
        if let Ok(entries) = fs::read_dir(&section_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("md") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        files.push((stem.to_string(), path));
                    }
                }
            }
        }

        // Sort files alphabetically within each section
        files.sort_by(|a, b| a.0.cmp(&b.0));

        if !files.is_empty() {
            sections.push((section.to_string(), files));
        }
    }

    // Generate showcase module file
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("showcase_generated.rs");

    let mut code = String::new();
    code.push_str("// Auto-generated showcase pages from markdown files\n\n");
    code.push_str("use crate::markdown;\n");
    code.push_str("use armas::*;\n");
    code.push_str("use armas_showcase_macros::showcase_page;\n");
    code.push_str("use eframe::egui;\n\n");

    // Generate modules for each file
    for (section_name, files) in &sections {
        for (file_name, file_path) in files {
            let module_name = format!("{}_{}", section_name, file_name).replace("-", "_");
            code.push_str(&format!("pub mod {} {{\n", module_name));
            code.push_str("    use super::*;\n");
            // Add crates/armas-web/ prefix for workspace-relative path
            let workspace_path = format!("crates/armas-web/{}",
                file_path.to_str().unwrap().replace("\\", "/"));
            code.push_str(&format!("    showcase_page!(\"{}\");\n", workspace_path));
            code.push_str("}\n\n");
        }
    }

    // Generate the get_pages function (flat list)
    code.push_str("pub fn get_pages() -> Vec<(&'static str, fn(&mut egui::Ui))> {\n");
    code.push_str("    vec![\n");

    for (section_name, files) in &sections {
        for (file_name, _) in files {
            let module_name = format!("{}_{}", section_name, file_name).replace("-", "_");
            let display_name = file_name.replace("-", " ").replace("_", " ");
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
                display_name, module_name));
        }
    }

    code.push_str("    ]\n");
    code.push_str("}\n\n");

    // Generate get_sections function (grouped by section)
    code.push_str("pub fn get_sections() -> Vec<(&'static str, Vec<(&'static str, fn(&mut egui::Ui))>)> {\n");
    code.push_str("    vec![\n");

    for (section_name, files) in &sections {
        let section_display = section_name.replace("_", " ");
        let section_display = section_display
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

        code.push_str(&format!("        (\"{}\", vec![\n", section_display));

        for (file_name, _) in files {
            let module_name = format!("{}_{}", section_name, file_name).replace("-", "_");
            let display_name = file_name.replace("-", " ").replace("_", " ");
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

            code.push_str(&format!("            (\"{}\", {}::show as fn(&mut egui::Ui)),\n",
                display_name, module_name));
        }

        code.push_str("        ]),\n");
    }

    code.push_str("    ]\n");
    code.push_str("}\n");

    fs::write(&dest_path, code).unwrap();
}
