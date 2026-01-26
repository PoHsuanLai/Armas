//! Procedural macros for generating showcase pages from markdown
//!
//! This crate provides the `showcase_page!` macro that parses markdown files
//! at compile time and generates Rust code that renders both markdown content
//! and live component demos.

mod parser;
mod codegen;

use proc_macro::TokenStream;
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

    match parser::parse_markdown(&markdown_content) {
        Ok(blocks) => match codegen::generate_show_function(&blocks, &markdown_path) {
            Ok(tokens) => tokens.into(),
            Err(e) => syn::Error::new(path.span(), e).to_compile_error().into(),
        },
        Err(e) => syn::Error::new(path.span(), e).to_compile_error().into(),
    }
}
