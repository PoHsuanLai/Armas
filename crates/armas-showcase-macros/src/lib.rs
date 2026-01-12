//! Procedural macros for generating showcase pages from markdown
//!
//! This crate provides the `showcase_page!` macro that parses markdown files
//! at compile time and generates Rust code that renders both markdown content
//! and live component demos.

use proc_macro::TokenStream;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Generate a showcase page function from a markdown file
///
/// # Example
///
/// ```rust,ignore
/// showcase_page!("content/buttons.md");
/// ```
///
/// This will generate a `show` function that renders the markdown content
/// and any live component demos marked with `demo` code blocks.
#[proc_macro]
pub fn showcase_page(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as LitStr);
    let markdown_path = path.value();

    // Read the markdown file at compile time
    let markdown_content = match std::fs::read_to_string(&markdown_path) {
        Ok(content) => content,
        Err(e) => {
            return syn::Error::new(
                path.span(),
                format!("Failed to read markdown file '{}': {}", markdown_path, e),
            )
            .to_compile_error()
            .into();
        }
    };

    // Parse the markdown and generate code
    match parse_showcase_markdown(&markdown_content) {
        Ok(tokens) => tokens.into(),
        Err(e) => syn::Error::new(path.span(), e).to_compile_error().into(),
    }
}

/// Parse markdown content and generate Rust code
fn parse_showcase_markdown(markdown: &str) -> Result<proc_macro2::TokenStream, String> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(markdown, options);

    let mut code_blocks = Vec::new();
    let mut current_text = String::new();
    let mut in_code_block = false;
    let mut code_block_lang = String::new();
    let mut code_block_content = String::new();
    let mut in_table_head = false;
    let mut table_column_count = 0;

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                // Save any accumulated text before code block
                if !current_text.is_empty() {
                    code_blocks.push(CodeBlock::Markdown(current_text.clone()));
                    current_text.clear();
                }

                in_code_block = true;
                code_block_lang = match kind {
                    pulldown_cmark::CodeBlockKind::Fenced(lang) => lang.to_string(),
                    pulldown_cmark::CodeBlockKind::Indented => String::new(),
                };
                code_block_content.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;

                // Check if this is a demo block
                if code_block_lang == "demo" {
                    code_blocks.push(CodeBlock::Demo(
                        code_block_content.clone(),
                        "rust".to_string(), // Demo blocks are always Rust
                    ));
                    // Demo now handles its own code display via tabs, no need to add markdown
                } else {
                    // Regular code block, add to markdown
                    code_blocks.push(CodeBlock::Markdown(format!(
                        "```{}\n{}\n```",
                        code_block_lang, code_block_content
                    )));
                }

                code_block_lang.clear();
                code_block_content.clear();
            }
            Event::Text(text) => {
                if in_code_block {
                    code_block_content.push_str(&text);
                } else {
                    current_text.push_str(&text);
                }
            }
            Event::Code(code) => {
                if !in_code_block {
                    current_text.push('`');
                    current_text.push_str(&code);
                    current_text.push('`');
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                if !in_code_block {
                    current_text.push('\n');
                }
            }
            Event::Start(Tag::Heading { level, .. }) => {
                if !current_text.is_empty() {
                    code_blocks.push(CodeBlock::Markdown(current_text.clone()));
                    current_text.clear();
                }
                // Preserve the heading markdown syntax
                let hashes = match level {
                    pulldown_cmark::HeadingLevel::H1 => "#",
                    pulldown_cmark::HeadingLevel::H2 => "##",
                    pulldown_cmark::HeadingLevel::H3 => "###",
                    pulldown_cmark::HeadingLevel::H4 => "####",
                    pulldown_cmark::HeadingLevel::H5 => "#####",
                    pulldown_cmark::HeadingLevel::H6 => "######",
                };
                current_text.push_str(hashes);
                current_text.push(' ');
            }
            Event::End(TagEnd::Heading(_)) => {
                current_text.push_str("\n\n");
            }
            Event::Start(Tag::Paragraph) => {}
            Event::End(TagEnd::Paragraph) => {
                current_text.push_str("\n\n");
            }
            Event::Start(Tag::List(_)) => {
                current_text.push('\n');
            }
            Event::End(TagEnd::List(_)) => {
                current_text.push_str("\n\n");
            }
            Event::Start(Tag::Item) => {
                current_text.push_str("- ");
            }
            Event::End(TagEnd::Item) => {
                current_text.push('\n');
            }
            Event::Start(Tag::Strong) => current_text.push_str("**"),
            Event::End(TagEnd::Strong) => current_text.push_str("**"),
            Event::Start(Tag::Emphasis) => current_text.push('*'),
            Event::End(TagEnd::Emphasis) => current_text.push('*'),
            Event::Rule => {
                current_text.push_str("\n---\n");
            }
            Event::Start(Tag::Table(_)) => {
                current_text.push('\n');
                table_column_count = 0;
            }
            Event::End(TagEnd::Table) => {
                current_text.push_str("\n\n");
            }
            Event::Start(Tag::TableHead) => {
                in_table_head = true;
                table_column_count = 0;
            }
            Event::End(TagEnd::TableHead) => {
                // Add separator row after header
                current_text.push('\n'); // Newline to end the header row first
                current_text.push('|');
                for _ in 0..table_column_count {
                    current_text.push_str("--------|");
                }
                current_text.push('\n');
                in_table_head = false;
            }
            Event::Start(Tag::TableRow) => {
                current_text.push('|');
            }
            Event::End(TagEnd::TableRow) => {
                current_text.push('\n');
            }
            Event::Start(Tag::TableCell) => {
                current_text.push(' ');
                if in_table_head {
                    table_column_count += 1;
                }
            }
            Event::End(TagEnd::TableCell) => {
                current_text.push_str(" |");
            }
            _ => {}
        }
    }

    // Add any remaining text
    if !current_text.is_empty() {
        code_blocks.push(CodeBlock::Markdown(current_text));
    }

    // Generate the Rust code
    generate_show_function(&code_blocks)
}

enum CodeBlock {
    Markdown(String),
    Demo(String, String), // (code, language)
}

/// Wraps demo code to persist mutable state across frames using egui's memory system
fn wrap_demo_with_persistence(code: &str) -> String {
    // No wrapping needed - components now handle their own state persistence internally
    code.to_string()
}

fn generate_show_function(blocks: &[CodeBlock]) -> Result<proc_macro2::TokenStream, String> {
    let mut statements = Vec::new();

    for block in blocks {
        match block {
            CodeBlock::Markdown(md) => {
                let md_content = md.trim();
                if !md_content.is_empty() {
                    statements.push(quote! {
                        markdown::render_markdown(ui, #md_content, &theme);
                        ui.add_space(8.0);
                    });
                }
            }
            CodeBlock::Demo(code, lang) => {
                // Wrap the code to persist mutable state
                let wrapped_code = wrap_demo_with_persistence(code);

                // Parse the wrapped demo code as Rust statements
                let demo_code: proc_macro2::TokenStream = wrapped_code.parse().map_err(|e| {
                    format!(
                        "Failed to parse demo code: {}\n\nCode:\n{}",
                        e, wrapped_code
                    )
                })?;

                let code_string = code.to_string();
                let lang_string = lang.to_string();

                statements.push(quote! {
                    // Demo container with tabs
                    {
                        use armas::{AnimatedTabs, TabStyle};

                        let demo_id = ui.id().with(#code_string);
                        let mut active_tab = ui.ctx().data_mut(|d| d.get_temp::<usize>(demo_id).unwrap_or(0));

                        // Container with border
                        let container_rect = ui.available_rect_before_wrap();
                        let mut tabs = AnimatedTabs::new(vec!["Preview", "Code"])
                            .style(TabStyle::Underline)
                            .active(active_tab);

                        let _ = ui.vertical(|ui| {
                            ui.set_max_width(ui.available_width());

                            // Tabs at the top (narrower)
                            ui.horizontal(|ui| {
                                let tab_width = ui.available_width() / 3.0;
                                ui.allocate_ui(egui::vec2(tab_width, ui.available_height()), |ui| {
                                    if let Some(new_tab) = tabs.show(ui) {
                                        active_tab = new_tab;
                                        ui.ctx().data_mut(|d| d.insert_temp(demo_id, active_tab));
                                    }
                                });
                            });

                            ui.add_space(12.0);

                            // Content area with border
                            let _ = egui::Frame::NONE
                                .fill(theme.surface())
                                .stroke(egui::Stroke::new(1.0, theme.outline()))
                                .corner_radius(8.0)
                                .inner_margin(24.0)
                                .show(ui, |ui| {
                                    if active_tab == 0 {
                                        // Preview - centered, expands vertically with content
                                        // Set fixed width to prevent jumping
                                        ui.set_width(ui.available_width());

                                        // Use vertical layout with centered main axis
                                        let _ = ui.with_layout(
                                            egui::Layout::top_down(egui::Align::Center),
                                            |ui| {
                                                // Request repaint for animations
                                                ui.ctx().request_repaint();

                                                // Execute demo code in a unique scope
                                                let _ = ui.push_id(demo_id.with("preview"), |ui| {
                                                    #[allow(unused_must_use)]
                                                    {
                                                        #demo_code
                                                    }
                                                });
                                            }
                                        );
                                    } else {
                                        // Code display with syntax highlighting
                                        let code = #code_string;

                                        ui.set_width(ui.available_width());

                                        // Use a stack to overlay the copy button
                                        let available_rect = ui.available_rect_before_wrap();
                                        let (scroll_rect, button_rect) = {
                                            let button_size = egui::vec2(80.0, 32.0);
                                            let scroll_rect = egui::Rect::from_min_size(
                                                available_rect.min,
                                                egui::vec2(available_rect.width(), 400.0)
                                            );
                                            let button_rect = egui::Rect::from_min_size(
                                                egui::pos2(
                                                    available_rect.max.x - button_size.x - 12.0,
                                                    available_rect.min.y + 12.0
                                                ),
                                                button_size
                                            );
                                            (scroll_rect, button_rect)
                                        };

                                        // Render scroll area with code
                                        let _ = ui.scope_builder(egui::UiBuilder::new().max_rect(scroll_rect), |ui| {
                                            egui::ScrollArea::vertical()
                                                .id_salt(demo_id.with("code_scroll"))
                                                .max_height(400.0)
                                                .show(ui, |ui| {
                                                    ui.set_width(ui.available_width());

                                                    egui::Frame::NONE
                                                        .fill(egui::Color32::from_gray(20))
                                                        .inner_margin(12.0)
                                                        .corner_radius(4.0)
                                                        .show(ui, |ui| {
                                                            ui.set_width(ui.available_width());
                                                            crate::syntax::highlight_code(ui, code, #lang_string, &theme);
                                                        });
                                                });
                                        });

                                        // Overlay copy button on top
                                        let _ = ui.scope_builder(egui::UiBuilder::new().max_rect(button_rect), |ui| {
                                            use armas::{Button, ButtonVariant};
                                            if Button::new("Copy")
                                                .variant(ButtonVariant::Text)
                                                .show(ui)
                                                .clicked()
                                            {
                                                ui.ctx().copy_text(code.to_string());
                                            }
                                        });
                                    }
                                });
                        });

                        ui.add_space(16.0);
                    }
                });
            }
        }
    }

    Ok(quote! {
        pub fn show(ui: &mut egui::Ui) {
            let theme = ui.ctx().armas_theme();
            #(#statements)*
        }
    })
}
