use armas::Theme;
use egui::text::{LayoutJob, TextFormat};

pub fn highlight_code(ui: &mut egui::Ui, code: &str, language: &str, theme: &Theme) {
    match language {
        "rust" | "rs" => highlight_rust_code(ui, code, theme),
        "toml" => highlight_toml_code(ui, code, theme),
        _ => highlight_rust_code(ui, code, theme), // Default to Rust
    }
}

pub fn highlight_rust_code(ui: &mut egui::Ui, code: &str, theme: &Theme) {
    let mut job = LayoutJob::default();

    // Define color scheme using theme colors only
    let keyword_color = theme.primary(); // Primary color for keywords
    let string_color = egui::Color32::from_rgb(206, 145, 120); // Orange/peach
    let comment_color = egui::Color32::from_rgb(106, 153, 85); // Green (muted)
    let function_color = theme.on_surface_variant(); // Use theme color instead of yellow
    let number_color = egui::Color32::from_rgb(181, 206, 168); // Light green
    let type_color = theme.secondary(); // Secondary color for types
    let default_color = theme.on_surface(); // Theme's text color

    let keywords = [
        "let", "mut", "fn", "if", "else", "for", "while", "loop", "match", "return", "struct",
        "enum", "impl", "trait", "pub", "use", "mod", "const", "static", "async", "await",
        "unsafe", "extern", "as", "break", "continue", "crate", "super", "self", "Self", "where",
        "type", "ref", "move", "true", "false",
    ];

    let types = [
        "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
        "f32", "f64", "bool", "char", "str", "String", "Vec", "Option", "Result", "Some", "None",
        "Ok", "Err",
    ];

    // Process line by line
    for (line_idx, line) in code.lines().enumerate() {
        let mut pos = 0;
        let chars: Vec<char> = line.chars().collect();

        while pos < chars.len() {
            // Skip whitespace
            if chars[pos].is_whitespace() {
                let start = pos;
                while pos < chars.len() && chars[pos].is_whitespace() {
                    pos += 1;
                }
                job.append(
                    &chars[start..pos].iter().collect::<String>(),
                    0.0,
                    TextFormat {
                        font_id: egui::FontId::monospace(14.0),
                        color: default_color,
                        ..Default::default()
                    },
                );
                continue;
            }

            // Comments
            if pos + 1 < chars.len() && chars[pos] == '/' && chars[pos + 1] == '/' {
                job.append(
                    &chars[pos..].iter().collect::<String>(),
                    0.0,
                    TextFormat {
                        font_id: egui::FontId::monospace(14.0),
                        color: comment_color,
                        ..Default::default()
                    },
                );
                break;
            }

            // Strings
            if chars[pos] == '"' {
                let start = pos;
                pos += 1;
                while pos < chars.len() {
                    if chars[pos] == '\\' && pos + 1 < chars.len() {
                        pos += 2;
                    } else if chars[pos] == '"' {
                        pos += 1;
                        break;
                    } else {
                        pos += 1;
                    }
                }
                job.append(
                    &chars[start..pos].iter().collect::<String>(),
                    0.0,
                    TextFormat {
                        font_id: egui::FontId::monospace(14.0),
                        color: string_color,
                        ..Default::default()
                    },
                );
                continue;
            }

            // Numbers
            if chars[pos].is_numeric() {
                let start = pos;
                while pos < chars.len()
                    && (chars[pos].is_numeric() || chars[pos] == '.' || chars[pos] == '_')
                {
                    pos += 1;
                }
                job.append(
                    &chars[start..pos].iter().collect::<String>(),
                    0.0,
                    TextFormat {
                        font_id: egui::FontId::monospace(14.0),
                        color: number_color,
                        ..Default::default()
                    },
                );
                continue;
            }

            // Identifiers (keywords, types, functions)
            if chars[pos].is_alphabetic() || chars[pos] == '_' {
                let start = pos;
                while pos < chars.len() && (chars[pos].is_alphanumeric() || chars[pos] == '_') {
                    pos += 1;
                }
                let word: String = chars[start..pos].iter().collect();

                let color = if keywords.contains(&word.as_str()) {
                    keyword_color
                } else if types.contains(&word.as_str()) {
                    type_color
                } else if pos < chars.len() && chars[pos] == '(' {
                    function_color
                } else if word.chars().next().unwrap().is_uppercase() {
                    type_color
                } else {
                    default_color
                };

                job.append(
                    &word,
                    0.0,
                    TextFormat {
                        font_id: egui::FontId::monospace(14.0),
                        color,
                        ..Default::default()
                    },
                );
                continue;
            }

            // Punctuation and operators
            let c = chars[pos].to_string();
            job.append(
                &c,
                0.0,
                TextFormat {
                    font_id: egui::FontId::monospace(14.0),
                    color: default_color,
                    ..Default::default()
                },
            );
            pos += 1;
        }

        // Add newline except for last line
        if line_idx < code.lines().count() - 1 {
            job.append(
                "\n",
                0.0,
                TextFormat {
                    font_id: egui::FontId::monospace(14.0),
                    color: default_color,
                    ..Default::default()
                },
            );
        }
    }

    ui.label(job);
}

pub fn highlight_toml_code(ui: &mut egui::Ui, code: &str, theme: &Theme) {
    let mut job = LayoutJob::default();

    // Define color scheme using theme colors
    let key_color = egui::Color32::from_rgb(156, 220, 254); // Light blue
    let string_color = egui::Color32::from_rgb(206, 145, 120); // Orange/peach
    let comment_color = egui::Color32::from_rgb(106, 153, 85); // Green
    let section_color = theme.primary(); // Primary for sections
    let boolean_color = theme.secondary(); // Secondary for booleans
    let number_color = egui::Color32::from_rgb(181, 206, 168); // Light green
    let default_color = theme.on_surface(); // Theme's text color

    for (line_idx, line) in code.lines().enumerate() {
        let trimmed = line.trim_start();

        // Comments
        if trimmed.starts_with('#') {
            job.append(
                line,
                0.0,
                TextFormat {
                    font_id: egui::FontId::monospace(14.0),
                    color: comment_color,
                    ..Default::default()
                },
            );
        }
        // Section headers [section]
        else if trimmed.starts_with('[') {
            job.append(
                line,
                0.0,
                TextFormat {
                    font_id: egui::FontId::monospace(14.0),
                    color: section_color,
                    ..Default::default()
                },
            );
        }
        // Key = value pairs
        else if let Some(eq_pos) = line.find('=') {
            // Key part
            job.append(
                &line[..eq_pos],
                0.0,
                TextFormat {
                    font_id: egui::FontId::monospace(14.0),
                    color: key_color,
                    ..Default::default()
                },
            );

            // Equals sign
            job.append(
                "=",
                0.0,
                TextFormat {
                    font_id: egui::FontId::monospace(14.0),
                    color: default_color,
                    ..Default::default()
                },
            );

            // Value part
            let value = line[eq_pos + 1..].trim_start();
            let value_color = if value.starts_with('"') || value.starts_with('\'') {
                string_color
            } else if value == "true" || value == "false" {
                boolean_color
            } else if value
                .chars()
                .next()
                .map(|c| c.is_numeric())
                .unwrap_or(false)
            {
                number_color
            } else {
                default_color
            };

            job.append(
                &line[eq_pos + 1..],
                0.0,
                TextFormat {
                    font_id: egui::FontId::monospace(14.0),
                    color: value_color,
                    ..Default::default()
                },
            );
        }
        // Everything else
        else {
            job.append(
                line,
                0.0,
                TextFormat {
                    font_id: egui::FontId::monospace(14.0),
                    color: default_color,
                    ..Default::default()
                },
            );
        }

        // Add newline except for last line
        if line_idx < code.lines().count() - 1 {
            job.append(
                "\n",
                0.0,
                TextFormat {
                    font_id: egui::FontId::monospace(14.0),
                    color: default_color,
                    ..Default::default()
                },
            );
        }
    }

    ui.label(job);
}
