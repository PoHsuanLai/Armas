use std::collections::BTreeMap;
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

    // Define section order (with nested components)
    let section_order = vec![
        "introduction",
        "installation",
        "layout",
        "components/basic",
        "components/navigation",
        "components/cards",
        "components/animated",
        "components/audio",
        "components/overlays",
        "backgrounds",
    ];

    // Define file order within each section (if not specified, alphabetical)
    let mut file_order: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    file_order.insert(
        "introduction",
        vec!["introduction", "why_egui", "philosophy", "attributions"],
    );
    file_order.insert("installation", vec!["quick_start", "cargo_setup", "wasm"]);

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

        // Sort files according to file_order if specified, otherwise alphabetically
        if let Some(order) = file_order.get(section) {
            files.sort_by(|a, b| {
                let a_pos = order.iter().position(|&s| s == a.0.as_str());
                let b_pos = order.iter().position(|&s| s == b.0.as_str());
                match (a_pos, b_pos) {
                    (Some(a_idx), Some(b_idx)) => a_idx.cmp(&b_idx),
                    (Some(_), None) => std::cmp::Ordering::Less, // Ordered items come first
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => a.0.cmp(&b.0), // Both unordered, sort alphabetically
                }
            });
        } else {
            // Sort files alphabetically within each section
            files.sort_by(|a, b| a.0.cmp(&b.0));
        }

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
    code.push_str("use armas_animated::*;\n");
    code.push_str("use armas_audio::*;\n");
    code.push_str("use armas_showcase_macros::showcase_page;\n");
    code.push_str("use eframe::egui;\n");
    code.push('\n');
    code.push_str("// Type aliases for complex types\n");
    code.push_str("type PageShowFn = fn(&mut egui::Ui);\n");
    code.push_str("type Page = (&'static str, PageShowFn);\n");
    code.push_str("type Section = (&'static str, Vec<Page>);\n");
    code.push_str("type NestedSections = Vec<(&'static str, Vec<Section>)>;\n\n");

    // Generate modules for each file
    for (section_name, files) in &sections {
        for (file_name, file_path) in files {
            let module_name = format!("{}_{}", section_name, file_name)
                .replace("-", "_")
                .replace("/", "_");
            code.push_str(&format!("pub mod {} {{\n", module_name));
            code.push_str("    use super::*;\n");
            // Add crates/armas-web/ prefix for workspace-relative path
            let workspace_path = format!(
                "crates/armas-web/{}",
                file_path.to_str().unwrap().replace("\\", "/")
            );
            code.push_str(&format!("    showcase_page!(\"{}\");\n", workspace_path));
            code.push_str("}\n\n");
        }
    }

    // Generate the get_pages function (flat list)
    code.push_str("pub fn get_pages() -> Vec<Page> {\n");
    code.push_str("    vec![\n");

    for (section_name, files) in &sections {
        for (file_name, _) in files {
            let module_name = format!("{}_{}", section_name, file_name)
                .replace("-", "_")
                .replace("/", "_");
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

            code.push_str(&format!(
                "        (\"{}\", {}::show as fn(&mut egui::Ui)),\n",
                display_name, module_name
            ));
        }
    }

    code.push_str("    ]\n");
    code.push_str("}\n\n");

    // Generate get_nested_sections function (hierarchical structure)
    code.push_str("#[allow(clippy::vec_init_then_push)]\n");
    code.push_str("pub fn get_nested_sections() -> NestedSections {\n");
    code.push_str("    let mut nested = Vec::new();\n\n");

    // Group sections by parent, preserving order
    type SectionGroup = Vec<(String, Vec<(String, PathBuf)>)>;
    let mut grouped: Vec<(String, SectionGroup)> = Vec::new();
    let mut seen_parents: Vec<String> = Vec::new();

    for (section_name, files) in &sections {
        if section_name.contains('/') {
            let parts: Vec<&str> = section_name.split('/').collect();
            let parent = parts[0].to_string();

            // Find or create parent entry
            if let Some(pos) = seen_parents.iter().position(|p| p == &parent) {
                grouped[pos].1.push((section_name.clone(), files.clone()));
            } else {
                seen_parents.push(parent.clone());
                grouped.push((parent, vec![(section_name.clone(), files.clone())]));
            }
        } else {
            // Top-level section
            if !seen_parents.contains(section_name) {
                seen_parents.push(section_name.clone());
                grouped.push((section_name.clone(), Vec::new()));
            }
        }
    }

    for (parent_name, subsections) in &grouped {
        let parent_display = parent_name.replace("_", " ");
        let parent_display = parent_display
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

        if subsections.is_empty() {
            // Top-level section with no subsections - wrap in single subsection for consistency
            if let Some((section_name, files)) = sections.iter().find(|(s, _)| s == parent_name) {
                code.push_str(&format!("    nested.push((\"{}\", vec![\n", parent_display));
                code.push_str(&format!("        (\"{}\", vec![\n", parent_display));

                for (file_name, _) in files {
                    let module_name = format!("{}_{}", section_name, file_name)
                        .replace("-", "_")
                        .replace("/", "_");
                    let display_name = file_name.replace("-", " ").replace("_", " ");
                    let display_name = display_name
                        .split_whitespace()
                        .map(|word| {
                            let mut chars = word.chars();
                            match chars.next() {
                                None => String::new(),
                                Some(first) => {
                                    first.to_uppercase().collect::<String>() + chars.as_str()
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ");

                    code.push_str(&format!(
                        "            (\"{}\", {}::show as fn(&mut egui::Ui)),\n",
                        display_name, module_name
                    ));
                }

                code.push_str("        ]),\n");
                code.push_str("    ]));\n\n");
            }
        } else {
            // Parent with subsections
            code.push_str(&format!("    nested.push((\"{}\", vec![\n", parent_display));

            for (section_name, files) in subsections {
                let display_section = section_name.split('/').next_back().unwrap();
                let section_display = display_section.replace("_", " ");
                let section_display = section_display
                    .split_whitespace()
                    .map(|word| {
                        let mut chars = word.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => {
                                first.to_uppercase().collect::<String>() + chars.as_str()
                            }
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ");

                code.push_str(&format!("        (\"{}\", vec![\n", section_display));

                for (file_name, _) in files {
                    let module_name = format!("{}_{}", section_name, file_name)
                        .replace("-", "_")
                        .replace("/", "_");
                    let display_name = file_name.replace("-", " ").replace("_", " ");
                    let display_name = display_name
                        .split_whitespace()
                        .map(|word| {
                            let mut chars = word.chars();
                            match chars.next() {
                                None => String::new(),
                                Some(first) => {
                                    first.to_uppercase().collect::<String>() + chars.as_str()
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ");

                    code.push_str(&format!(
                        "            (\"{}\", {}::show as fn(&mut egui::Ui)),\n",
                        display_name, module_name
                    ));
                }

                code.push_str("        ]),\n");
            }

            code.push_str("    ]));\n\n");
        }
    }

    code.push_str("    nested\n");
    code.push_str("}\n");

    fs::write(&dest_path, code).unwrap();
}
