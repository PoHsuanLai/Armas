//! Components list page — shows all components grouped by section

use armas::*;
use eframe::egui;

use crate::showcase_gen;

const CARD_GAP: f32 = 12.0;
const CARD_PADDING: f32 = 16.0;
const CARD_ROUNDING: f32 = 8.0;

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
        let is_mobile = rect.width() < 768.0;

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.set_width(rect.width());

            let h_pad = if is_mobile { 24.0 } else { 48.0 };
            let content_width = 768.0f32.min(rect.width() - h_pad);
            let margin = (rect.width() - content_width) / 2.0;

            ui.add_space(margin.max(if is_mobile { 16.0 } else { 24.0 }));

            ui.vertical(|ui| {
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
        pages: &[(&'static str, fn(&mut egui::Ui))],
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

        // Lay out cards in a manual grid
        let available = ui.available_width();
        let card_width = (available - CARD_GAP * (cols as f32 - 1.0)) / cols as f32;

        for row in pages.chunks(cols) {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = CARD_GAP;

                for (name, _) in row.iter() {
                    let (rect, card_response) =
                        ui.allocate_exact_size(egui::vec2(card_width, 44.0), egui::Sense::click());

                    // Border color: highlight on hover
                    let border_color = if card_response.hovered() {
                        self.theme.foreground()
                    } else {
                        self.theme.border()
                    };

                    if card_response.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }

                    // Card background + border
                    ui.painter()
                        .rect_filled(rect, CARD_ROUNDING, self.theme.background());
                    ui.painter().rect_stroke(
                        rect,
                        CARD_ROUNDING,
                        egui::Stroke::new(1.0, border_color),
                        egui::StrokeKind::Inside,
                    );

                    // Component name
                    let text_pos = egui::pos2(rect.left() + CARD_PADDING, rect.center().y - 7.0);
                    ui.painter().text(
                        text_pos,
                        egui::Align2::LEFT_CENTER,
                        *name,
                        egui::FontId::new(14.0, egui::FontFamily::Name("InterMedium".into())),
                        self.theme.foreground(),
                    );

                    if card_response.clicked() {
                        for (i, (page_name, _)) in self.pages.iter().enumerate() {
                            if *page_name == *name {
                                response.selected_page = Some(i);
                                break;
                            }
                        }
                    }
                }
            });

            ui.add_space(CARD_GAP);
        }

        ui.add_space(12.0);
    }
}
