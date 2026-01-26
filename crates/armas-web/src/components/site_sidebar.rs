//! Site sidebar component for documentation navigation

use armas::*;
use eframe::egui;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::showcase_gen;

pub struct SiteSidebar<'a> {
    theme: &'a Theme,
    search_text: &'a mut String,
    pages: &'a [(&'static str, fn(&mut egui::Ui))],
}

pub struct SiteSidebarResponse {
    pub selected_page: Option<usize>,
}

impl<'a> SiteSidebar<'a> {
    pub fn new(
        theme: &'a Theme,
        search_text: &'a mut String,
        pages: &'a [(&'static str, fn(&mut egui::Ui))],
    ) -> Self {
        Self {
            theme,
            search_text,
            pages,
        }
    }

    pub fn show(self, ui: &mut egui::Ui) -> SiteSidebarResponse {
        let mut response = SiteSidebarResponse { selected_page: None };
        let mut clicked_name: Option<String> = None;

        let sections = showcase_gen::get_nested_sections();
        let search = self.search_text.trim().to_lowercase();
        let matcher = if !search.is_empty() {
            Some(SkimMatcherV2::default())
        } else {
            None
        };

        egui::Frame::new()
            .fill(self.theme.background())
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.add_space(4.0);

                // Search
                Input::new("Search...")
                    .width(ui.available_width() - 8.0)
                    .variant(InputVariant::Outlined)
                    .show(ui, self.search_text);

                ui.add_space(12.0);

                egui::ScrollArea::vertical()
                    .id_salt("sidebar")
                    .show(ui, |ui| {
                        let sidebar_response = armas::Sidebar::new()
                            .collapsible(armas::CollapsibleMode::None)
                            .show_icons(false)
                            .show(ui, |sidebar| {
                                for (parent, subsections) in sections.iter() {
                                    let has_matches = subsections.iter().any(|(_, pages)| {
                                        if let Some(ref m) = matcher {
                                            pages
                                                .iter()
                                                .any(|(n, _)| m.fuzzy_match(n, &search).is_some())
                                        } else {
                                            !pages.is_empty()
                                        }
                                    });

                                    if !has_matches {
                                        continue;
                                    }

                                    let is_flat =
                                        subsections.len() == 1 && !subsections[0].1.is_empty();

                                    if is_flat {
                                        sidebar.group("", parent, |group| {
                                            let pages = &subsections[0].1;
                                            let filtered: Vec<_> = if let Some(ref m) = matcher {
                                                pages
                                                    .iter()
                                                    .filter(|(n, _)| {
                                                        m.fuzzy_match(n, &search).is_some()
                                                    })
                                                    .collect()
                                            } else {
                                                pages.iter().collect()
                                            };

                                            for (name, _) in filtered {
                                                group.item("", name);
                                            }
                                        });
                                    } else {
                                        sidebar.group("", parent, |parent_group| {
                                            for (section, pages) in subsections.iter() {
                                                let filtered: Vec<_> = if let Some(ref m) = matcher
                                                {
                                                    pages
                                                        .iter()
                                                        .filter(|(n, _)| {
                                                            m.fuzzy_match(n, &search).is_some()
                                                        })
                                                        .collect()
                                                } else {
                                                    pages.iter().collect()
                                                };

                                                if !filtered.is_empty() {
                                                    parent_group.group("", section, |subgroup| {
                                                        for (name, _) in filtered {
                                                            subgroup.item("", name);
                                                        }
                                                    });
                                                }
                                            }
                                        });
                                    }
                                }
                            });

                        if let Some(id) = sidebar_response.clicked {
                            if let Some(pos) = id.rfind('_') {
                                clicked_name = Some(id[pos + 1..].to_string());
                            }
                        }
                    });
            });

        if let Some(name) = clicked_name {
            for (i, (page_name, _)) in self.pages.iter().enumerate() {
                if *page_name == name {
                    response.selected_page = Some(i);
                    break;
                }
            }
        }

        response
    }
}
