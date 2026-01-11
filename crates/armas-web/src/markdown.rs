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
                Tag::CodeBlock(_) => {
                    in_code_block = true;
                    code_block_text.clear();
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
                    render_code_block(ui, &code_block_text, theme, base_id, element_counter);
                    element_counter += 1;
                    code_block_text.clear();
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
        HeadingLevel::H1 => (28.0, 12.0),
        HeadingLevel::H2 => (24.0, 10.0),
        HeadingLevel::H3 => (20.0, 8.0),
        HeadingLevel::H4 => (18.0, 6.0),
        HeadingLevel::H5 => (16.0, 4.0),
        HeadingLevel::H6 => (14.0, 4.0),
    };

    ui.label(
        egui::RichText::new(text)
            .size(font_size)
            .color(theme.on_background())
            .strong(),
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
                            .monospace()
                            .background_color(egui::Color32::from_rgb(25, 25, 25))
                            .color(theme.primary()),
                    );
                } else {
                    ui.label(
                        egui::RichText::new(&text)
                            .size(14.0)
                            .color(theme.on_surface_variant()),
                    );
                }
            }
        });
    });

    ui.add_space(8.0);
}

fn render_code_block(ui: &mut egui::Ui, code: &str, theme: &Theme, base_id: u64, id: usize) {
    ui.add_space(8.0);

    let frame = egui::Frame::NONE
        .fill(egui::Color32::from_rgb(25, 25, 25))
        .corner_radius(4.0)
        .inner_margin(12.0);

    frame.show(ui, |ui| {
        ui.style_mut().override_font_id = Some(egui::FontId::monospace(13.0));

        ui.push_id((base_id, id), |ui| {
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.label(
                    egui::RichText::new(code.trim())
                        .color(theme.on_surface())
                        .monospace(),
                );
            });
        });
    });

    ui.add_space(8.0);
}

fn render_list_item(ui: &mut egui::Ui, text: &str, theme: &Theme, base_id: u64, id: usize) {
    ui.push_id((base_id, id), |ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("•").size(14.0).color(theme.primary()));
            ui.add_space(8.0);

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

            ui.horizontal_wrapped(|ui| {
                for (text, is_code) in segments {
                    if is_code {
                        ui.label(
                            egui::RichText::new(&text)
                                .monospace()
                                .background_color(egui::Color32::from_rgb(25, 25, 25))
                                .color(theme.primary()),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new(&text)
                                .size(14.0)
                                .color(theme.on_surface_variant()),
                        );
                    }
                }
            });
        });
    });
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
        use armas::{Table, TableStyle};

        Table::new().style(TableStyle::Lined).show(ui, |table| {
            // Render headers
            table.header_row(|row| {
                for header in headers {
                    row.cell(header);
                }
            });

            // Render rows
            for data_row in rows {
                table.row(|row| {
                    for cell in data_row {
                        // Parse inline code in cells
                        let mut segments = Vec::new();
                        let mut current = String::new();
                        let mut in_code = false;
                        let chars = cell.chars().peekable();

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
                        row.cell_ui(|ui| {
                            ui.horizontal_wrapped(|ui| {
                                for (text, is_code) in segments {
                                    if is_code {
                                        ui.label(
                                            egui::RichText::new(&text)
                                                .monospace()
                                                .background_color(egui::Color32::from_rgb(
                                                    25, 25, 25,
                                                ))
                                                .color(theme.primary()),
                                        );
                                    } else {
                                        ui.label(
                                            egui::RichText::new(&text)
                                                .size(14.0)
                                                .color(theme.on_surface_variant()),
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
