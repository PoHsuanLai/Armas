//! Code display component with syntax highlighting and copy functionality

use armas::*;
use eframe::egui;

/// Simple syntax highlighter for Rust code
pub struct RustHighlighter;

impl RustHighlighter {
    /// Highlight a line of Rust code
    pub fn highlight_line(line: &str) -> Vec<(String, egui::Color32)> {
        let keywords = [
            "fn", "let", "mut", "pub", "impl", "struct", "enum", "use", "mod",
            "const", "static", "match", "if", "else", "for", "while", "loop",
            "return", "break", "continue", "self", "Self", "super", "crate",
        ];
        let types = [
            "String", "Vec2", "bool", "f32", "u32", "i32", "f64", "i64",
            "usize", "isize", "Vec", "Option", "Result", "Box", "Rc", "Arc",
            "Ui", "Response", "Color32", "Theme",
        ];
        let functions = [
            "new", "show", "variant", "min_size", "width", "height",
            "into", "to_string", "clone", "unwrap", "expect",
        ];

        // Colors (Nord-inspired palette)
        let keyword_color = egui::Color32::from_rgb(129, 161, 193); // Blue
        let type_color = egui::Color32::from_rgb(143, 188, 187);    // Cyan
        let string_color = egui::Color32::from_rgb(163, 190, 140);  // Green
        let comment_color = egui::Color32::from_rgb(76, 86, 106);   // Gray
        let function_color = egui::Color32::from_rgb(136, 192, 208); // Teal
        let number_color = egui::Color32::from_rgb(180, 142, 173);  // Purple
        let default_color = egui::Color32::from_rgb(216, 222, 233); // Light gray

        let mut result = Vec::new();
        let mut chars = line.chars().peekable();
        let mut current_token = String::new();

        while let Some(ch) = chars.next() {
            // Handle comments
            if ch == '/' && chars.peek() == Some(&'/') {
                current_token.push(ch);
                current_token.push(chars.next().unwrap());
                // Consume rest of line
                while let Some(ch) = chars.next() {
                    current_token.push(ch);
                }
                result.push((current_token.clone(), comment_color));
                current_token.clear();
                break;
            }

            // Handle strings
            if ch == '"' {
                current_token.push(ch);
                while let Some(ch) = chars.next() {
                    current_token.push(ch);
                    if ch == '"' {
                        break;
                    }
                    if ch == '\\' {
                        if let Some(escaped) = chars.next() {
                            current_token.push(escaped);
                        }
                    }
                }
                result.push((current_token.clone(), string_color));
                current_token.clear();
                continue;
            }

            // Handle numbers
            if ch.is_numeric() {
                current_token.push(ch);
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_numeric() || next_ch == '.' || next_ch == '_' {
                        current_token.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                result.push((current_token.clone(), number_color));
                current_token.clear();
                continue;
            }

            // Handle identifiers and keywords
            if ch.is_alphabetic() || ch == '_' {
                current_token.push(ch);
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        current_token.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                let color = if keywords.contains(&current_token.as_str()) {
                    keyword_color
                } else if types.contains(&current_token.as_str()) {
                    type_color
                } else if functions.contains(&current_token.as_str()) {
                    function_color
                } else {
                    default_color
                };

                result.push((current_token.clone(), color));
                current_token.clear();
                continue;
            }

            // Handle other characters (operators, punctuation)
            result.push((ch.to_string(), default_color));
        }

        result
    }
}

/// Code display card with syntax highlighting
pub struct CodeDisplayCard {
    code: String,
    language: String,
    show_line_numbers: bool,
    show_copy_button: bool,
    github_url: Option<String>,
}

impl CodeDisplayCard {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            language: "rust".to_string(),
            show_line_numbers: true,
            show_copy_button: true,
            github_url: None,
        }
    }

    pub fn github_url(mut self, url: impl Into<String>) -> Self {
        self.github_url = Some(url.into());
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = language.into();
        self
    }

    pub fn line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    pub fn copy_button(mut self, show: bool) -> Self {
        self.show_copy_button = show;
        self
    }

    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = ui.ctx().armas_theme();
        let mut response = ui.label("");

        Card::new()
            .corner_radius(12.0)
            .show(ui, &theme, |ui| {
                // Header with language badge, GitHub link, and copy button
                ui.horizontal(|ui| {
                    Badge::new(&self.language.to_uppercase())
                        .color(BadgeColor::Info)
                        .show(ui);

                    Spacer::new().show(ui);

                    // GitHub link button
                    if let Some(github_url) = &self.github_url {
                        if Button::new("</> GitHub")
                            .variant(ButtonVariant::Text)
                            .show(ui)
                            .on_hover_text("View source on GitHub")
                            .clicked()
                        {
                            #[cfg(target_arch = "wasm32")]
                            {
                                if let Some(window) = web_sys::window() {
                                    let _ = window.open_with_url(github_url);
                                }
                            }
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                let _ = open::that(github_url);
                            }
                        }
                    }

                    if self.show_copy_button {
                        if Button::new("📋 Copy")
                            .variant(ButtonVariant::Text)
                            .show(ui)
                            .clicked()
                        {
                            ui.ctx().copy_text(self.code.clone());
                        }
                    }
                });

                ui.add_space(8.0);
                Divider::horizontal().show(ui);
                ui.add_space(8.0);

                // Code area with scroll
                egui::ScrollArea::both()
                    .max_height(600.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 16.0;

                            // Line numbers
                            if self.show_line_numbers {
                                ui.vertical(|ui| {
                                    ui.spacing_mut().item_spacing.y = 0.0;
                                    for (i, _) in self.code.lines().enumerate() {
                                        ui.label(
                                            egui::RichText::new(format!("{:>3}", i + 1))
                                                .monospace()
                                                .color(theme.on_surface_variant()),
                                        );
                                    }
                                });
                            }

                            // Code content
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 0.0;

                                for line in self.code.lines() {
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 0.0;

                                        if self.language == "rust" {
                                            let tokens = RustHighlighter::highlight_line(line);
                                            for (token, color) in tokens {
                                                ui.label(
                                                    egui::RichText::new(token)
                                                        .monospace()
                                                        .color(color),
                                                );
                                            }
                                        } else {
                                            // Fallback for other languages
                                            ui.label(
                                                egui::RichText::new(line)
                                                    .monospace()
                                                    .color(theme.on_surface()),
                                            );
                                        }
                                    });
                                }
                            });
                        });

                        response = ui.interact(
                            ui.min_rect(),
                            ui.id(),
                            egui::Sense::hover(),
                        );
                    });
            });

        response
    }
}

/// Inline code snippet (for small code snippets in text)
pub fn code_inline(ui: &mut egui::Ui, code: &str, theme: &Theme) {
    ui.label(
        egui::RichText::new(code)
            .monospace()
            .background_color(theme.surface_variant())
            .color(theme.primary()),
    );
}
