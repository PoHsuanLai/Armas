//! Components list page — shows all components grouped by section

use armas::*;
use eframe::egui;

use crate::showcase_gen;

pub struct ComponentsListPage<'a> {
    theme: &'a Theme,
    pages: &'a [(&'static str, fn(&mut egui::Ui))],
}

pub struct ComponentsListResponse {
    pub selected_page: Option<usize>,
}

impl<'a> ComponentsListPage<'a> {
    pub fn new(theme: &'a Theme, pages: &'a [(&'static str, fn(&mut egui::Ui))]) -> Self {
        Self { theme, pages }
    }

    pub fn show(self, ui: &mut egui::Ui) -> ComponentsListResponse {
        let mut response = ComponentsListResponse {
            selected_page: None,
        };

        let sections = showcase_gen::get_nested_sections();
        let rect = ui.available_rect_before_wrap();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.set_width(rect.width());

            let content_width = 768.0f32.min(rect.width() - 48.0);
            let margin = (rect.width() - content_width) / 2.0;

            ui.add_space(margin.max(24.0));

            ui.vertical(|ui| {
                ui.set_max_width(content_width);

                ui.add_space(32.0);

                // Title
                ui.label(
                    egui::RichText::new("Components")
                        .size(32.0)
                        .family(egui::FontFamily::Name("InterBold".into()))
                        .color(self.theme.foreground()),
                );

                ui.add_space(32.0);

                // Sections (skip non-component sections)
                let skip = ["Introduction", "Installation"];
                for (parent, subsections) in sections.iter() {
                    if skip.iter().any(|s| s.eq_ignore_ascii_case(parent)) {
                        continue;
                    }
                    let is_flat = subsections.len() == 1 && !subsections[0].1.is_empty();

                    if is_flat {
                        self.render_section(ui, parent, &subsections[0].1, &mut response);
                    } else {
                        for (section_name, pages) in subsections.iter() {
                            if !pages.is_empty() {
                                self.render_section(ui, section_name, pages, &mut response);
                            }
                        }
                    }
                }

                ui.add_space(64.0);
            });
        });

        response
    }

    fn render_section(
        &self,
        ui: &mut egui::Ui,
        title: &str,
        pages: &[(&'static str, fn(&mut egui::Ui))],
        response: &mut ComponentsListResponse,
    ) {
        // Section header
        ui.label(
            egui::RichText::new(title)
                .size(14.0)
                .family(egui::FontFamily::Name("InterSemiBold".into()))
                .color(self.theme.foreground()),
        );

        ui.add_space(8.0);

        // Component links in a wrapped horizontal layout
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 4.0);

            for (name, _) in pages.iter() {
                let link = ui.add(
                    egui::Label::new(
                        egui::RichText::new(*name)
                            .size(14.0)
                            .color(self.theme.muted_foreground()),
                    )
                    .sense(egui::Sense::click()),
                );

                if link.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    // Underline on hover
                    let rect = link.rect;
                    ui.painter().line_segment(
                        [
                            egui::pos2(rect.left(), rect.bottom()),
                            egui::pos2(rect.right(), rect.bottom()),
                        ],
                        egui::Stroke::new(1.0, self.theme.muted_foreground()),
                    );
                }

                if link.clicked() {
                    // Find the page index by name
                    for (i, (page_name, _)) in self.pages.iter().enumerate() {
                        if *page_name == *name {
                            response.selected_page = Some(i);
                            break;
                        }
                    }
                }

                // Add separator between items
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new("·")
                        .size(14.0)
                        .color(self.theme.muted()),
                );
                ui.add_space(6.0);
            }
        });

        ui.add_space(20.0);
    }
}
