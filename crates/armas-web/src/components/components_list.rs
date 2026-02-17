//! Components list page — shows all components grouped by section

use armas::*;
use eframe::egui;

use crate::showcase_gen;

const CARD_GAP: f32 = 12.0;

/// A page entry with name and render function
type PageEntry = (&'static str, fn(&mut egui::Ui));

pub struct ComponentsListPage<'a> {
    theme: &'a Theme,
    pages: &'a [PageEntry],
}

pub struct ComponentsListResponse {
    pub selected_page: Option<usize>,
}

impl<'a> ComponentsListPage<'a> {
    pub fn new(theme: &'a Theme, pages: &'a [PageEntry]) -> Self {
        Self { theme, pages }
    }

    pub fn show(self, ui: &mut egui::Ui) -> ComponentsListResponse {
        let mut response = ComponentsListResponse {
            selected_page: None,
        };

        let sections = showcase_gen::get_nested_sections();
        let rect = ui.available_rect_before_wrap();
        let is_mobile = rect.width() < 768.0;

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.set_width(rect.width());

            let h_pad = if is_mobile { 24.0 } else { 48.0 };
            let content_width = 768.0f32.min(rect.width() - h_pad);

            ui.vertical_centered(|ui| {
                ui.set_max_width(content_width);

                ui.add_space(if is_mobile { 20.0 } else { 32.0 });

                // Title
                let title_size = if is_mobile { 24.0 } else { 32.0 };
                ui.label(
                    egui::RichText::new("Components")
                        .size(title_size)
                        .family(egui::FontFamily::Name("InterBold".into()))
                        .color(self.theme.foreground()),
                );

                ui.add_space(if is_mobile { 20.0 } else { 32.0 });

                // Responsive columns: 1 on mobile, 2 on tablet, 3 on desktop
                let cols = if content_width < 400.0 {
                    1
                } else if content_width < 600.0 {
                    2
                } else {
                    3
                };

                // Sections (skip non-component sections)
                let skip = ["Introduction", "Installation"];
                for (parent, subsections) in sections.iter() {
                    if skip.iter().any(|s| s.eq_ignore_ascii_case(parent)) {
                        continue;
                    }
                    let is_flat = subsections.len() == 1 && !subsections[0].1.is_empty();

                    if is_flat {
                        self.render_section(ui, parent, &subsections[0].1, cols, &mut response);
                    } else {
                        for (section_name, pages) in subsections.iter() {
                            if !pages.is_empty() {
                                self.render_section(ui, section_name, pages, cols, &mut response);
                            }
                        }
                    }
                }

                ui.add_space(if is_mobile { 32.0 } else { 64.0 });
            });
        });

        response
    }

    fn render_section(
        &self,
        ui: &mut egui::Ui,
        title: &str,
        pages: &[PageEntry],
        cols: usize,
        response: &mut ComponentsListResponse,
    ) {
        // Section header
        ui.label(
            egui::RichText::new(title)
                .size(14.0)
                .family(egui::FontFamily::Name("InterSemiBold".into()))
                .color(self.theme.foreground()),
        );

        ui.add_space(12.0);

        let available = ui.available_width();
        let card_width = (available - CARD_GAP * (cols as f32 - 1.0)) / cols as f32;

        egui::Grid::new(title)
            .num_columns(cols)
            .spacing(egui::vec2(CARD_GAP, CARD_GAP))
            .min_col_width(card_width)
            .max_col_width(card_width)
            .show(ui, |ui| {
                for (idx, (name, _)) in pages.iter().enumerate() {
                    let btn = Button::new(*name)
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Large)
                        .full_width(true);
                    let btn_response = btn.show(ui);

                    if btn_response.clicked() {
                        for (i, (page_name, _)) in self.pages.iter().enumerate() {
                            if *page_name == *name {
                                response.selected_page = Some(i);
                                break;
                            }
                        }
                    }

                    if (idx + 1) % cols == 0 {
                        ui.end_row();
                    }
                }
            });

        ui.add_space(12.0);
    }
}
