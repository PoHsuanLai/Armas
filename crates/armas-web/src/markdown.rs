//! Markdown rendering for egui

use armas::*;
use eframe::egui;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

/// Renders markdown content to egui UI
pub fn render_markdown(ui: &mut egui::Ui, markdown: &str, theme: &Theme) {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(markdown, options);

    let mut current_text = String::new();
    let mut in_heading = None;
    let mut in_code_block = false;
    let mut code_block_text = String::new();
    let mut code_block_lang = String::new();
    let mut in_list = false;
    let mut list_item_text = String::new();
    let mut in_table = false;
    let mut in_table_head = false;
    let mut table_headers: Vec<String> = Vec::new();
    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut current_row: Vec<String> = Vec::new();
    let mut current_cell = String::new();

    // Use hash of markdown content as base ID to ensure uniqueness across multiple render_markdown calls
    let base_id = {
        let mut hash = 0u64;
        for byte in markdown.as_bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(*byte as u64);
        }
        hash
    };
    let mut element_counter = 0usize;

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    in_heading = Some(level);
                }
                Tag::CodeBlock(kind) => {
                    in_code_block = true;
                    code_block_text.clear();
                    code_block_lang = match kind {
                        pulldown_cmark::CodeBlockKind::Fenced(lang) => lang.to_string(),
                        pulldown_cmark::CodeBlockKind::Indented => String::new(),
                    };
                }
                Tag::List(_) => {
                    in_list = true;
                }
                Tag::Item => {
                    list_item_text.clear();
                }
                Tag::Emphasis => {
                    // Emphasis tracking not currently used
                }
                Tag::Strong => {
                    // Strong tracking not currently used
                }
                Tag::Strikethrough => {
                    // Strikethrough tracking not currently used
                }
                Tag::Table(_) => {
                    in_table = true;
                    table_headers.clear();
                    table_rows.clear();
                }
                Tag::TableHead => {
                    in_table_head = true;
                    current_row.clear();
                }
                Tag::TableRow => {
                    current_row.clear();
                }
                Tag::TableCell => {
                    current_cell.clear();
                }
                _ => {}
            },

            Event::End(tag_end) => match tag_end {
                TagEnd::Heading(_) => {
                    if let Some(level) = in_heading {
                        render_heading(ui, &current_text, level, theme);
                        current_text.clear();
                        in_heading = None;
                    }
                }
                TagEnd::CodeBlock => {
                    render_code_block(
                        ui,
                        &code_block_text,
                        &code_block_lang,
                        theme,
                        base_id,
                        element_counter,
                    );
                    element_counter += 1;
                    code_block_text.clear();
                    code_block_lang.clear();
                    in_code_block = false;
                }
                TagEnd::Paragraph => {
                    if !current_text.is_empty() && !in_list && !in_table {
                        render_paragraph(ui, &current_text, theme, base_id, element_counter);
                        element_counter += 1;
                        current_text.clear();
                    }
                }
                TagEnd::List(_) => {
                    in_list = false;
                    ui.add_space(8.0);
                }
                TagEnd::Item => {
                    if !list_item_text.is_empty() {
                        render_list_item(ui, &list_item_text, theme, base_id, element_counter);
                        element_counter += 1;
                        list_item_text.clear();
                    }
                }
                TagEnd::Emphasis => {
                    // Emphasis tracking not currently used
                }
                TagEnd::Strong => {
                    // Strong tracking not currently used
                }
                TagEnd::Strikethrough => {
                    // Strikethrough tracking not currently used
                }
                TagEnd::Table => {
                    if in_table {
                        render_table(
                            ui,
                            &table_headers,
                            &table_rows,
                            theme,
                            base_id,
                            element_counter,
                        );
                        element_counter += 1;
                        in_table = false;
                    }
                }
                TagEnd::TableHead => {
                    if in_table_head {
                        table_headers = current_row.clone();
                        current_row.clear();
                        in_table_head = false;
                    }
                }
                TagEnd::TableRow => {
                    if in_table && !in_table_head {
                        table_rows.push(current_row.clone());
                        current_row.clear();
                    }
                }
                TagEnd::TableCell => {
                    current_row.push(current_cell.clone());
                    current_cell.clear();
                }
                _ => {}
            },

            Event::Text(text) => {
                if in_code_block {
                    code_block_text.push_str(&text);
                } else if in_table {
                    current_cell.push_str(&text);
                } else if in_list {
                    list_item_text.push_str(&text);
                } else {
                    current_text.push_str(&text);
                }
            }

            Event::Code(code) => {
                if in_table {
                    current_cell.push('`');
                    current_cell.push_str(&code);
                    current_cell.push('`');
                } else if in_list {
                    list_item_text.push('`');
                    list_item_text.push_str(&code);
                    list_item_text.push('`');
                } else {
                    current_text.push('`');
                    current_text.push_str(&code);
                    current_text.push('`');
                }
            }

            Event::SoftBreak | Event::HardBreak => {
                if in_table {
                    current_cell.push(' ');
                } else if in_list {
                    list_item_text.push(' ');
                } else {
                    current_text.push(' ');
                }
            }

            Event::Rule => {
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);
            }

            _ => {}
        }
    }

    // Render any remaining text
    if !current_text.is_empty() {
        render_paragraph(ui, &current_text, theme, base_id, element_counter);
    }
}

fn render_heading(ui: &mut egui::Ui, text: &str, level: HeadingLevel, theme: &Theme) {
    ui.add_space(16.0);

    let (font_size, spacing_after) = match level {
        HeadingLevel::H1 => (32.0, 12.0),
        HeadingLevel::H2 => (26.0, 10.0),
        HeadingLevel::H3 => (22.0, 8.0),
        HeadingLevel::H4 => (19.0, 6.0),
        HeadingLevel::H5 => (17.0, 4.0),
        HeadingLevel::H6 => (15.0, 4.0),
    };

    // Use InterBold font family for extra bold headers
    ui.label(
        egui::RichText::new(text)
            .size(font_size)
            .family(egui::FontFamily::Name("InterBold".into()))
            .color(theme.primary()),
    );

    ui.add_space(spacing_after);
}

fn render_paragraph(ui: &mut egui::Ui, text: &str, theme: &Theme, base_id: u64, id: usize) {
    // Parse inline formatting
    let mut segments = Vec::new();
    let mut current = String::new();
    let mut in_code = false;
    let chars = text.chars().peekable();

    for c in chars {
        if c == '`' {
            if !current.is_empty() {
                segments.push((current.clone(), in_code));
                current.clear();
            }
            in_code = !in_code;
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        segments.push((current, in_code));
    }

    // Render the segments with unique ID combining base_id and element counter
    ui.push_id((base_id, id), |ui| {
        ui.horizontal_wrapped(|ui| {
            for (text, is_code) in segments {
                if is_code {
                    ui.label(
                        egui::RichText::new(&text)
                            .size(14.0)
                            .family(egui::FontFamily::Name("FiraMono".into()))
                            .background_color(theme.muted())
                            .color(theme.primary()),
                    );
                } else {
                    ui.label(
                        egui::RichText::new(&text)
                            .size(14.0)
                            .color(theme.muted_foreground()),
                    );
                }
            }
        });
    });

    ui.add_space(8.0);
}

fn render_code_block(
    ui: &mut egui::Ui,
    code: &str,
    language: &str,
    theme: &Theme,
    base_id: u64,
    id: usize,
) {
    ui.add_space(8.0);

    // Use theme-aware background color
    let bg_color = theme.muted();

    let frame = egui::Frame::NONE
        .fill(bg_color)
        .corner_radius(8.0)
        .stroke(egui::Stroke::new(1.0, theme.border()))
        .inner_margin(0.0);

    frame.show(ui, |ui| {
        ui.set_width(ui.available_width());

        // Get available rect for overlaying button
        let available_rect = ui.available_rect_before_wrap();

        // Code content with syntax highlighting
        ui.push_id((base_id, id), |ui| {
            // Calculate height based on number of lines
            let line_count = code.trim().lines().count();
            let line_height = 20.0; // Approximate height per line
            let padding = 24.0; // Top and bottom padding
            let content_height = line_count as f32 * line_height + padding;

            egui::ScrollArea::both()
                .max_height(content_height)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    egui::Frame::NONE
                        .fill(bg_color)
                        .inner_margin(12.0)
                        .show(ui, |ui| {
                            // Use syntax highlighting with detected language
                            let lang = if language.is_empty() {
                                "rust"
                            } else {
                                language
                            };
                            crate::syntax::highlight_code(ui, code.trim(), lang, theme);
                        });
                });
        });

        // Overlay buttons
        use armas::{Button, ButtonVariant};

        if language == "demo" {
            // Theme toggle button for demo blocks
            let toggle_id = egui::Id::new("theme_toggle_request");

            // Position button in top-right corner
            let button_size = egui::vec2(100.0, 32.0);
            let button_pos = egui::pos2(
                available_rect.max.x - button_size.x - 12.0,
                available_rect.min.y + 12.0,
            );
            let button_rect = egui::Rect::from_min_size(button_pos, button_size);

            let _ = ui.scope_builder(egui::UiBuilder::new().max_rect(button_rect), |ui| {
                if Button::new("🌓 Theme")
                    .variant(ButtonVariant::Text)
                    .show(ui, theme)
                    .on_hover_text("Toggle light/dark theme")
                    .clicked()
                {
                    ui.ctx().data_mut(|d| d.insert_temp(toggle_id, true));
                }
            });
        } else {
            // Copy button for code blocks
            let copy_id = ui.id().with((base_id, id, "copy_state"));
            let copied_at: Option<f64> = ui.ctx().data(|d| d.get_temp(copy_id));
            let current_time = ui.input(|i| i.time);

            // Show check mark for 2 seconds after copying
            let show_check = if let Some(copied_time) = copied_at {
                current_time - copied_time < 2.0
            } else {
                false
            };

            // Request repaint if we're showing the check mark
            if show_check {
                ui.ctx()
                    .request_repaint_after(std::time::Duration::from_millis(100));
            }

            let button_text = if show_check {
                "✓ Copied"
            } else {
                "📋 Copy"
            };
            let button_tooltip = if show_check {
                "Copied to clipboard"
            } else {
                "Copy code to clipboard"
            };

            // Position button in top-right corner
            let button_size = egui::vec2(100.0, 32.0);
            let button_pos = egui::pos2(
                available_rect.max.x - button_size.x - 12.0,
                available_rect.min.y + 12.0,
            );
            let button_rect = egui::Rect::from_min_size(button_pos, button_size);

            let _ = ui.scope_builder(egui::UiBuilder::new().max_rect(button_rect), |ui| {
                if Button::new(button_text)
                    .variant(ButtonVariant::Text)
                    .show(ui, theme)
                    .on_hover_text(button_tooltip)
                    .clicked()
                {
                    ui.ctx().copy_text(code.to_string());
                    ui.ctx().data_mut(|d| d.insert_temp(copy_id, current_time));

                    // For WASM, also use web Clipboard API
                    #[cfg(target_arch = "wasm32")]
                    {
                        if let Some(window) = web_sys::window() {
                            let navigator = window.navigator().clipboard();
                            let code_copy = code.to_string();
                            let promise = navigator.write_text(&code_copy);
                            let _ = wasm_bindgen_futures::JsFuture::from(promise);
                        }
                    }
                }
            });
        }
    });

    ui.add_space(8.0);
}

fn render_list_item(ui: &mut egui::Ui, text: &str, theme: &Theme, base_id: u64, id: usize) {
    ui.push_id((base_id, id), |ui| {
        ui.horizontal_top(|ui| {
            // Add left indentation
            ui.add_space(16.0);

            // Bullet point at the top
            ui.label(egui::RichText::new("•").size(14.0).color(theme.primary()));
            ui.add_space(6.0);

            // Parse inline code
            let mut segments = Vec::new();
            let mut current = String::new();
            let mut in_code = false;
            let chars = text.chars().peekable();

            for c in chars {
                if c == '`' {
                    if !current.is_empty() {
                        segments.push((current.clone(), in_code));
                        current.clear();
                    }
                    in_code = !in_code;
                } else {
                    current.push(c);
                }
            }

            if !current.is_empty() {
                segments.push((current, in_code));
            }

            // Text content with wrapping
            ui.vertical(|ui| {
                ui.horizontal_wrapped(|ui| {
                    for (text, is_code) in segments {
                        if is_code {
                            ui.label(
                                egui::RichText::new(&text)
                                    .size(14.0)
                                    .family(egui::FontFamily::Name("FiraMono".into()))
                                    .background_color(theme.muted())
                                    .color(theme.primary()),
                            );
                        } else {
                            ui.label(
                                egui::RichText::new(&text)
                                    .size(14.0)
                                    .color(theme.muted_foreground()),
                            );
                        }
                    }
                });
            });
        });
    });

    // Add spacing after each list item
    ui.add_space(4.0);
}

fn render_table(
    ui: &mut egui::Ui,
    headers: &[String],
    rows: &[Vec<String>],
    theme: &Theme,
    base_id: u64,
    id: usize,
) {
    ui.add_space(12.0);

    ui.push_id((base_id, id), |ui| {
        use armas::{table, header_row, row, cell_ui};

        table(ui, |table_rows| {
            // Render headers
            header_row(table_rows, |cells| {
                for header in headers {
                    cell_ui(cells, |ui| {
                        ui.label(header.as_str());
                    });
                }
            });

            // Render rows
            for data_row in rows {
                row(table_rows, |cells| {
                    for cell_text in data_row {
                        // Parse inline code in cells
                        let mut segments = Vec::new();
                        let mut current = String::new();
                        let mut in_code = false;
                        let chars = cell_text.chars().peekable();

                        for c in chars {
                            if c == '`' {
                                if !current.is_empty() {
                                    segments.push((current.clone(), in_code));
                                    current.clear();
                                }
                                in_code = !in_code;
                            } else {
                                current.push(c);
                            }
                        }

                        if !current.is_empty() {
                            segments.push((current, in_code));
                        }

                        // Render cell with inline formatting
                        cell_ui(cells, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                for (text, is_code) in segments {
                                    if is_code {
                                        ui.label(
                                            egui::RichText::new(&text)
                                                .size(14.0)
                                                .family(egui::FontFamily::Name("FiraMono".into()))
                                                .background_color(theme.muted())
                                                .color(theme.primary()),
                                        );
                                    } else {
                                        ui.label(
                                            egui::RichText::new(&text)
                                                .size(14.0)
                                                .color(theme.muted_foreground()),
                                        );
                                    }
                                }
                            });
                        });
                    }
                });
            }
        });
    });

    ui.add_space(12.0);
}
