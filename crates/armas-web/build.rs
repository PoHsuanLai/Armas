use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

type FileList = Vec<(String, PathBuf)>;
type Sections = Vec<(String, FileList)>;
type FileOrder = BTreeMap<&'static str, Vec<&'static str>>;

/// Capitalize first letter of each word in string
fn capitalize_words(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Convert file name to display name (replace separators and capitalize)
fn file_to_display_name(file_name: &str) -> String {
    let display_name = file_name.replace("-", " ").replace("_", " ");
    capitalize_words(&display_name)
}

/// Define section order
fn get_section_order() -> Vec<&'static str> {
    vec![
        "introduction",
        "installation",
        "layout",
        "basic",
        "navigation",
        "cards",
        "animated",
        "audio",
        "overlays",
        "backgrounds",
    ]
}

/// Define file order within each section
fn get_file_order() -> FileOrder {
    let mut file_order = BTreeMap::new();
    file_order.insert(
        "introduction",
        vec![
            "introduction",
            "api_guide",
            "why_egui",
            "philosophy",
            "attributions",
        ],
    );
    file_order.insert("installation", vec!["quick_start", "cargo_setup", "wasm"]);
    file_order
}

/// Scan content directory and collect markdown files by section
fn scan_content_directory(content_dir: &Path) -> Sections {
    let section_order = get_section_order();
    let file_order = get_file_order();
    let mut sections = Vec::new();

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
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => a.0.cmp(&b.0),
                }
            });
        } else {
            files.sort_by(|a, b| a.0.cmp(&b.0));
        }

        if !files.is_empty() {
            sections.push((section.to_string(), files));
        }
    }

    sections
}

/// Generate module imports and type aliases
fn generate_header() -> String {
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
    code
}

/// Generate module definitions for each markdown file
fn generate_modules(sections: &Sections) -> String {
    let mut code = String::new();
    for (section_name, files) in sections {
        for (file_name, file_path) in files {
            let module_name = format!("{}_{}", section_name, file_name)
                .replace("-", "_")
                .replace("/", "_");
            code.push_str(&format!("pub mod {} {{\n", module_name));
            code.push_str("    use super::*;\n");
            let workspace_path = format!(
                "crates/armas-web/{}",
                file_path.to_str().unwrap().replace("\\", "/")
            );
            code.push_str(&format!("    showcase_page!(\"{}\");\n", workspace_path));
            code.push_str("}\n\n");
        }
    }
    code
}

/// Generate get_pages function (flat list)
fn generate_pages_function(sections: &Sections) -> String {
    let mut code = String::new();
    code.push_str("pub fn get_pages() -> Vec<Page> {\n");
    code.push_str("    vec![\n");

    for (section_name, files) in sections {
        for (file_name, _) in files {
            let module_name = format!("{}_{}", section_name, file_name)
                .replace("-", "_")
                .replace("/", "_");
            let display_name = file_to_display_name(file_name);
            code.push_str(&format!(
                "        (\"{}\", {}::show as fn(&mut egui::Ui)),\n",
                display_name, module_name
            ));
        }
    }

    code.push_str("    ]\n");
    code.push_str("}\n\n");
    code
}

/// Generate route mapping function (route -> index)
fn generate_route_mapping(sections: &Sections) -> String {
    let mut code = String::new();
    code.push_str("#[allow(dead_code)]\n");
    code.push_str("pub fn get_page_by_route(section: &str, component: &str) -> Option<usize> {\n");
    code.push_str("    let route = format!(\"{}/{}\", section, component);\n");
    code.push_str("    match route.as_str() {\n");

    let mut idx = 0;
    for (section_name, files) in sections {
        for (file_name, _) in files {
            code.push_str(&format!(
                "        \"{}/{}\" => Some({}),\n",
                section_name, file_name, idx
            ));
            idx += 1;
        }
    }

    code.push_str("        _ => None,\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");
    code
}

/// Generate reverse mapping (index -> route)
fn generate_reverse_mapping(sections: &Sections) -> String {
    let mut code = String::new();
    code.push_str("#[allow(dead_code)]\n");
    code.push_str(
        "pub fn get_route_by_index(idx: usize) -> Option<(&'static str, &'static str)> {\n",
    );
    code.push_str("    match idx {\n");

    let mut idx = 0;
    for (section_name, files) in sections {
        for (file_name, _) in files {
            code.push_str(&format!(
                "        {} => Some((\"{}\", \"{}\")),\n",
                idx, section_name, file_name
            ));
            idx += 1;
        }
    }

    code.push_str("        _ => None,\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");
    code
}

/// Generate subsection with pages for nested structure
fn generate_subsection(section_name: &str, files: &FileList) -> String {
    let mut code = String::new();
    for (file_name, _) in files {
        let module_name = format!("{}_{}", section_name, file_name)
            .replace("-", "_")
            .replace("/", "_");
        let display_name = file_to_display_name(file_name);
        code.push_str(&format!(
            "            (\"{}\", {}::show as fn(&mut egui::Ui)),\n",
            display_name, module_name
        ));
    }
    code
}

/// Generate nested sections function (hierarchical structure)
fn generate_nested_sections(sections: &Sections) -> String {
    let mut code = String::new();
    code.push_str("#[allow(clippy::vec_init_then_push)]\n");
    code.push_str("pub fn get_nested_sections() -> NestedSections {\n");
    code.push_str("    let mut nested = Vec::new();\n\n");

    // Group sections by parent, preserving order
    type SectionGroup = Vec<(String, FileList)>;
    let mut grouped: Vec<(String, SectionGroup)> = Vec::new();
    let mut seen_parents: Vec<String> = Vec::new();

    for (section_name, files) in sections {
        if section_name.contains('/') {
            let parts: Vec<&str> = section_name.split('/').collect();
            let parent = parts[0].to_string();

            if let Some(pos) = seen_parents.iter().position(|p| p == &parent) {
                grouped[pos].1.push((section_name.clone(), files.clone()));
            } else {
                seen_parents.push(parent.clone());
                grouped.push((parent, vec![(section_name.clone(), files.clone())]));
            }
        } else if !seen_parents.contains(section_name) {
            seen_parents.push(section_name.clone());
            grouped.push((section_name.clone(), Vec::new()));
        }
    }

    for (parent_name, subsections) in &grouped {
        let parent_display = capitalize_words(&parent_name.replace("_", " "));

        if subsections.is_empty() {
            // Top-level section with no subsections
            if let Some((section_name, files)) = sections.iter().find(|(s, _)| s == parent_name) {
                code.push_str(&format!("    nested.push((\"{}\", vec![\n", parent_display));
                code.push_str(&format!("        (\"{}\", vec![\n", parent_display));
                code.push_str(&generate_subsection(section_name, files));
                code.push_str("        ]),\n");
                code.push_str("    ]));\n\n");
            }
        } else {
            // Parent with subsections
            code.push_str(&format!("    nested.push((\"{}\", vec![\n", parent_display));

            for (section_name, files) in subsections {
                let display_section = section_name.split('/').next_back().unwrap();
                let section_display = capitalize_words(&display_section.replace("_", " "));
                code.push_str(&format!("        (\"{}\", vec![\n", section_display));
                code.push_str(&generate_subsection(section_name, files));
                code.push_str("        ]),\n");
            }

            code.push_str("    ]));\n\n");
        }
    }

    code.push_str("    nested\n");
    code.push_str("}\n");
    code
}

/// Generate web icons from SVGs
fn generate_web_icons() {
    let icons_dir = Path::new("assets/icons");
    if !icons_dir.exists() {
        return;
    }

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("web_icons.rs");
    let mut output = File::create(&dest_path).unwrap();

    writeln!(
        output,
        "// Generated web icon data - DO NOT EDIT MANUALLY\n"
    )
    .unwrap();

    if let Err(e) = armas_icon::build::generate_icons_from_dir(icons_dir, &mut output) {
        eprintln!("Warning: Failed to generate web icons: {}", e);
    }
}

fn main() {
    // Tell cargo to rerun if content directory changes
    println!("cargo:rerun-if-changed=content");
    println!("cargo:rerun-if-changed=assets/icons");

    // Generate icon data from SVGs
    generate_web_icons();

    // Read all markdown files from content directory
    let content_dir = Path::new("content");
    if !content_dir.exists() {
        return;
    }

    let sections = scan_content_directory(content_dir);

    // Generate showcase module file
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("showcase_generated.rs");

    let mut code = String::new();
    code.push_str(&generate_header());
    code.push_str(&generate_modules(&sections));
    code.push_str(&generate_pages_function(&sections));
    code.push_str(&generate_route_mapping(&sections));
    code.push_str(&generate_reverse_mapping(&sections));
    code.push_str(&generate_nested_sections(&sections));

    fs::write(&dest_path, code).unwrap();
}
