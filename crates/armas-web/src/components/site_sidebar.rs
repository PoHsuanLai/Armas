//! Site sidebar component for documentation navigation

use armas::*;
use eframe::egui::{self, Color32, FontId, Sense, Vec2};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::showcase_gen;

type PageEntry = (&'static str, fn(&mut egui::Ui));

pub struct SiteSidebar<'a> {
    theme: &'a Theme,
    search_text: &'a mut String,
    pages: &'a [PageEntry],
}

pub struct SiteSidebarResponse {
    pub selected_page: Option<usize>,
}

impl<'a> SiteSidebar<'a> {
    pub fn new(theme: &'a Theme, search_text: &'a mut String, pages: &'a [PageEntry]) -> Self {
        Self { theme, search_text, pages }
    }

    pub fn show(self, ui: &mut egui::Ui) -> SiteSidebarResponse {
        let mut selected_page = None;

        let sections = showcase_gen::get_nested_sections();
        let search = self.search_text.trim().to_lowercase();
        let matcher = if !search.is_empty() { Some(SkimMatcherV2::default()) } else { None };

        // Load active page index from egui state
        let active_id = ui.id().with("sidebar_active");
        let active_idx: Option<usize> = ui.ctx().data(|d| d.get_temp(active_id));

        egui::Frame::new()
            .fill(self.theme.background())
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.add_space(4.0);

                Input::new("Search...")
                    .width(ui.available_width() - 8.0)
                    .variant(InputVariant::Outlined)
                    .show(ui, self.search_text);

                ui.add_space(12.0);

                egui::ScrollArea::vertical()
                    .id_salt("sidebar_scroll")
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;

                        for (parent, subsections) in sections.iter() {
                            // Collect matching pages across all subsections
                            let all_pages: Vec<&(&'static str, fn(&mut egui::Ui))> = subsections
                                .iter()
                                .flat_map(|(_, pages)| pages.iter())
                                .filter(|(name, _)| {
                                    if let Some(ref m) = matcher {
                                        m.fuzzy_match(name, &search).is_some()
                                    } else {
                                        true
                                    }
                                })
                                .collect();

                            if all_pages.is_empty() {
                                continue;
                            }

                            // Section header
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new(parent.to_uppercase())
                                    .font(FontId::proportional(11.0))
                                    .color(self.theme.muted_foreground()),
                            );
                            ui.add_space(4.0);

                            // Items
                            for (name, _) in &all_pages {
                                let page_idx = self.pages.iter().position(|(n, _)| n == name);
                                let is_active = page_idx.is_some() && page_idx == active_idx;

                                let (rect, response) = ui.allocate_exact_size(
                                    Vec2::new(ui.available_width(), 28.0),
                                    Sense::click(),
                                );

                                // Background on hover or active
                                if ui.is_rect_visible(rect) {
                                    let bg = if is_active {
                                        self.theme.accent()
                                    } else if response.hovered() {
                                        self.theme.accent().gamma_multiply(0.6)
                                    } else {
                                        Color32::TRANSPARENT
                                    };

                                    if bg != Color32::TRANSPARENT {
                                        ui.painter().rect_filled(rect, 4.0, bg);
                                    }

                                    let text_color = if is_active {
                                        self.theme.accent_foreground()
                                    } else {
                                        self.theme.foreground()
                                    };

                                    ui.painter().text(
                                        egui::pos2(rect.min.x + 8.0, rect.center().y),
                                        egui::Align2::LEFT_CENTER,
                                        name,
                                        FontId::proportional(13.0),
                                        text_color,
                                    );
                                }

                                if response.clicked() {
                                    if let Some(idx) = page_idx {
                                        ui.ctx().data_mut(|d| d.insert_temp(active_id, idx));
                                        selected_page = Some(idx);
                                    }
                                }
                            }
                        }

                        ui.add_space(16.0);
                    });
            });

        SiteSidebarResponse { selected_page }
    }
}
