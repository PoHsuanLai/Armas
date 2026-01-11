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
                    code_blocks.push(CodeBlock::Demo(code_block_content.clone()));
                    // Also add the code as a rust code block for display
                    code_blocks.push(CodeBlock::Markdown(format!(
                        "```rust\n{}\n```",
                        code_block_content
                    )));
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
            Event::Start(Tag::Heading { .. }) => {
                if !current_text.is_empty() {
                    code_blocks.push(CodeBlock::Markdown(current_text.clone()));
                    current_text.clear();
                }
                current_text.push_str("\n");
            }
            Event::End(TagEnd::Heading(_)) => {
                current_text.push_str("\n");
            }
            Event::Start(Tag::Paragraph) => {}
            Event::End(TagEnd::Paragraph) => {
                current_text.push_str("\n\n");
            }
            Event::Start(Tag::Strong) => current_text.push_str("**"),
            Event::End(TagEnd::Strong) => current_text.push_str("**"),
            Event::Start(Tag::Emphasis) => current_text.push('*'),
            Event::End(TagEnd::Emphasis) => current_text.push('*'),
            Event::Rule => {
                current_text.push_str("\n---\n");
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
    Demo(String),
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
            CodeBlock::Demo(code) => {
                // Parse the demo code as Rust statements
                let demo_code: proc_macro2::TokenStream = code
                    .parse()
                    .map_err(|e| format!("Failed to parse demo code: {}", e))?;

                statements.push(quote! {
                    ui.horizontal(|ui| {
                        #demo_code
                    });
                    ui.add_space(12.0);
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
