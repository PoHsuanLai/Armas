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
    let mut in_emphasis = false;
    let mut in_strong = false;
    let mut in_strikethrough = false;

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
                    in_emphasis = true;
                }
                Tag::Strong => {
                    in_strong = true;
                }
                Tag::Strikethrough => {
                    in_strikethrough = true;
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
                    if !current_text.is_empty() && !in_list {
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
                    in_emphasis = false;
                }
                TagEnd::Strong => {
                    in_strong = false;
                }
                TagEnd::Strikethrough => {
                    in_strikethrough = false;
                }
                _ => {}
            },

            Event::Text(text) => {
                if in_code_block {
                    code_block_text.push_str(&text);
                } else if in_list {
                    list_item_text.push_str(&text);
                } else {
                    current_text.push_str(&text);
                }
            }

            Event::Code(code) => {
                if in_list {
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
                if in_list {
                    list_item_text.push(' ');
                } else {
                    current_text.push(' ');
                }
            }

            Event::Rule => {
                ui.add_space(8.0);
                Divider::horizontal().show(ui);
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
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
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
                            .code()
                            .background_color(theme.surface_variant())
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

    let frame = egui::Frame::none()
        .fill(theme.surface_variant())
        .rounding(4.0)
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
            ui.label(
                egui::RichText::new("•")
                    .size(14.0)
                    .color(theme.primary()),
            );
            ui.add_space(8.0);

            // Parse inline code
            let mut segments = Vec::new();
            let mut current = String::new();
            let mut in_code = false;
            let mut chars = text.chars().peekable();

            while let Some(c) = chars.next() {
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
                                .code()
                                .background_color(theme.surface_variant())
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
