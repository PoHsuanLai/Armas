//! Markdown parsing for showcase pages

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

/// A parsed content block from the markdown
pub enum ContentBlock {
    /// Raw markdown text to render
    Markdown(String),
    /// A demo code block (code, language)
    Demo(String, String),
}

/// Parse markdown content into a list of content blocks
pub fn parse_markdown(markdown: &str) -> Result<Vec<ContentBlock>, String> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(markdown, options);
    let mut state = ParseState::default();

    for event in parser {
        state.handle_event(event);
    }

    state.finish()
}

#[derive(Default)]
struct ParseState {
    blocks: Vec<ContentBlock>,
    current_text: String,
    in_code_block: bool,
    code_block_lang: String,
    code_block_content: String,
    in_table_head: bool,
    table_column_count: usize,
}

impl ParseState {
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Start(tag) => self.handle_start_tag(tag),
            Event::End(tag) => self.handle_end_tag(tag),
            Event::Text(text) => self.handle_text(&text),
            Event::Code(code) => self.handle_inline_code(&code),
            Event::SoftBreak | Event::HardBreak => self.handle_break(),
            Event::Rule => self.current_text.push_str("\n---\n"),
            _ => {}
        }
    }

    fn handle_start_tag(&mut self, tag: Tag) {
        match tag {
            Tag::CodeBlock(kind) => {
                self.flush_text();
                self.in_code_block = true;
                self.code_block_lang = match kind {
                    pulldown_cmark::CodeBlockKind::Fenced(lang) => lang.to_string(),
                    pulldown_cmark::CodeBlockKind::Indented => String::new(),
                };
                self.code_block_content.clear();
            }
            Tag::Heading { level, .. } => {
                self.flush_text();
                let hashes = heading_prefix(level);
                self.current_text.push_str(hashes);
                self.current_text.push(' ');
            }
            Tag::Paragraph => {}
            Tag::List(_) => self.current_text.push('\n'),
            Tag::Item => self.current_text.push_str("- "),
            Tag::Strong => self.current_text.push_str("**"),
            Tag::Emphasis => self.current_text.push('*'),
            Tag::Table(_) => {
                self.current_text.push('\n');
                self.table_column_count = 0;
            }
            Tag::TableHead => {
                self.in_table_head = true;
                self.table_column_count = 0;
            }
            Tag::TableRow => self.current_text.push('|'),
            Tag::TableCell => {
                self.current_text.push(' ');
                if self.in_table_head {
                    self.table_column_count += 1;
                }
            }
            _ => {}
        }
    }

    fn handle_end_tag(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::CodeBlock => {
                self.in_code_block = false;
                if self.code_block_lang == "demo" {
                    self.blocks.push(ContentBlock::Demo(
                        self.code_block_content.clone(),
                        "rust".to_string(),
                    ));
                } else {
                    self.blocks.push(ContentBlock::Markdown(format!(
                        "```{}\n{}\n```",
                        self.code_block_lang, self.code_block_content
                    )));
                }
                self.code_block_lang.clear();
                self.code_block_content.clear();
            }
            TagEnd::Heading(_) => self.current_text.push_str("\n\n"),
            TagEnd::Paragraph => self.current_text.push_str("\n\n"),
            TagEnd::List(_) => self.current_text.push_str("\n\n"),
            TagEnd::Item => self.current_text.push('\n'),
            TagEnd::Strong => self.current_text.push_str("**"),
            TagEnd::Emphasis => self.current_text.push('*'),
            TagEnd::Table => self.current_text.push_str("\n\n"),
            TagEnd::TableHead => {
                self.current_text.push('\n');
                self.current_text.push('|');
                for _ in 0..self.table_column_count {
                    self.current_text.push_str("--------|");
                }
                self.current_text.push('\n');
                self.in_table_head = false;
            }
            TagEnd::TableRow => self.current_text.push('\n'),
            TagEnd::TableCell => self.current_text.push_str(" |"),
            _ => {}
        }
    }

    fn handle_text(&mut self, text: &str) {
        if self.in_code_block {
            self.code_block_content.push_str(text);
        } else {
            self.current_text.push_str(text);
        }
    }

    fn handle_inline_code(&mut self, code: &str) {
        if !self.in_code_block {
            self.current_text.push('`');
            self.current_text.push_str(code);
            self.current_text.push('`');
        }
    }

    fn handle_break(&mut self) {
        if !self.in_code_block {
            self.current_text.push('\n');
        }
    }

    fn flush_text(&mut self) {
        if !self.current_text.is_empty() {
            self.blocks
                .push(ContentBlock::Markdown(self.current_text.clone()));
            self.current_text.clear();
        }
    }

    fn finish(mut self) -> Result<Vec<ContentBlock>, String> {
        self.flush_text();
        Ok(self.blocks)
    }
}

fn heading_prefix(level: pulldown_cmark::HeadingLevel) -> &'static str {
    match level {
        pulldown_cmark::HeadingLevel::H1 => "#",
        pulldown_cmark::HeadingLevel::H2 => "##",
        pulldown_cmark::HeadingLevel::H3 => "###",
        pulldown_cmark::HeadingLevel::H4 => "####",
        pulldown_cmark::HeadingLevel::H5 => "#####",
        pulldown_cmark::HeadingLevel::H6 => "######",
    }
}
