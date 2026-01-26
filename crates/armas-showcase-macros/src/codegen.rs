//! Code generation for showcase pages

use crate::parser::ContentBlock;
use quote::quote;

/// Generate the `show` function from parsed content blocks
pub fn generate_show_function(
    blocks: &[ContentBlock],
    page_path: &str,
) -> Result<proc_macro2::TokenStream, String> {
    let mut statements = Vec::new();
    let mut demo_counter = 0usize;

    for block in blocks {
        match block {
            ContentBlock::Markdown(md) => {
                if let Some(stmt) = generate_markdown_block(md) {
                    statements.push(stmt);
                }
            }
            ContentBlock::Demo(code, lang) => {
                statements.push(generate_demo_block(code, lang, demo_counter, page_path)?);
                demo_counter += 1;
            }
        }
    }

    Ok(quote! {
        pub fn show(ui: &mut egui::Ui) {
            use egui::Color32;
            let theme = ui.ctx().armas_theme();
            ui.spacing_mut().item_spacing.y = 8.0;
            #(#statements)*
        }
    })
}

fn generate_markdown_block(md: &str) -> Option<proc_macro2::TokenStream> {
    let content = md.trim();
    if content.is_empty() {
        return None;
    }
    Some(quote! {
        markdown::render_markdown(ui, #content, &theme);
    })
}

fn generate_demo_block(
    code: &str,
    lang: &str,
    index: usize,
    page_path: &str,
) -> Result<proc_macro2::TokenStream, String> {
    let demo_code: proc_macro2::TokenStream = code
        .parse()
        .map_err(|e| format!("Failed to parse demo code: {}\n\nCode:\n{}", e, code))?;

    let code_string = code.to_string();
    let lang_string = lang.to_string();

    let preview_area = generate_preview_area(&demo_code);
    let code_area = generate_code_area(&code_string, &lang_string);
    let toolbar = generate_toolbar(&code_string);

    Ok(quote! {
        {
            let demo_id = ui.id().with((#page_path, "demo", #index));
            let active_tab: usize = ui.ctx().data(|d| d.get_temp(demo_id).unwrap_or(0));

            egui::Frame::NONE
                .stroke(egui::Stroke::new(1.0, theme.border()))
                .corner_radius(8.0)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());

                    if active_tab == 0 {
                        #preview_area
                    } else {
                        #code_area
                    }

                    #toolbar
                });

            ui.add_space(24.0);
        }
    })
}

fn generate_preview_area(demo_code: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    quote! {
        egui::Frame::NONE
            .fill(theme.background())
            .inner_margin(32.0)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.ctx().request_repaint();
                    ui.push_id(demo_id.with("preview"), |ui| {
                        #[allow(unused_must_use)]
                        { #demo_code }
                    });
                });
            });
    }
}

fn generate_code_area(code: &str, lang: &str) -> proc_macro2::TokenStream {
    quote! {
        egui::Frame::NONE
            .fill(theme.muted())
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                egui::ScrollArea::vertical()
                    .id_salt(demo_id.with("code_scroll"))
                    .max_height(300.0)
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        crate::syntax::highlight_code(ui, #code, #lang, &theme);
                    });
            });
    }
}

fn generate_toolbar(code: &str) -> proc_macro2::TokenStream {
    let tab_button = generate_tab_button();
    let copy_button = generate_copy_button(code);

    quote! {
        egui::Frame::NONE
            .fill(theme.muted())
            .inner_margin(egui::Margin::symmetric(12, 8))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;

                    // Preview tab
                    #tab_button
                    let preview_response = render_tab(ui, "Preview", active_tab == 0, &theme);
                    if preview_response.clicked() {
                        ui.ctx().data_mut(|d| d.insert_temp(demo_id, 0usize));
                    }

                    // Code tab
                    let code_response = render_tab(ui, "Code", active_tab == 1, &theme);
                    if code_response.clicked() {
                        ui.ctx().data_mut(|d| d.insert_temp(demo_id, 1usize));
                    }

                    // Spacer
                    ui.add_space(ui.available_width() - 70.0);

                    // Copy button
                    #copy_button
                });
            });
    }
}

fn generate_tab_button() -> proc_macro2::TokenStream {
    quote! {
        fn render_tab(
            ui: &mut egui::Ui,
            label: &str,
            selected: bool,
            theme: &armas::Theme,
        ) -> egui::Response {
            let bg = if selected {
                theme.background()
            } else {
                Color32::TRANSPARENT
            };
            let text_color = if selected {
                theme.foreground()
            } else {
                theme.muted_foreground()
            };

            let response = egui::Frame::NONE
                .fill(bg)
                .corner_radius(4.0)
                .inner_margin(egui::Margin::symmetric(12, 6))
                .show(ui, |ui| {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(label).size(12.0).color(text_color),
                        )
                        .sense(egui::Sense::click()),
                    )
                })
                .inner;

            if response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
            response
        }
    }
}

fn generate_copy_button(code: &str) -> proc_macro2::TokenStream {
    quote! {
        {
            let copy_id = demo_id.with("copy_state");
            let copied_at: Option<f64> = ui.ctx().data(|d| d.get_temp(copy_id));
            let current_time = ui.input(|i| i.time);
            let show_check = copied_at.map(|t| current_time - t < 2.0).unwrap_or(false);

            if show_check {
                ui.ctx().request_repaint_after(std::time::Duration::from_millis(100));
            }

            let copy_text = if show_check { "Copied!" } else { "Copy" };
            let copy_response = ui.add(
                egui::Label::new(
                    egui::RichText::new(copy_text)
                        .size(12.0)
                        .color(theme.muted_foreground()),
                )
                .sense(egui::Sense::click()),
            );

            if copy_response.clicked() {
                ui.ctx().copy_text(#code.to_string());
                ui.ctx().data_mut(|d| d.insert_temp(copy_id, current_time));
            }
            if copy_response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
        }
    }
}
