//! Section header component for organizing showcase pages

use armas::*;
use eframe::egui;

/// Section header with title, optional badge, and description
pub struct ShowcaseSectionHeader {
    title: String,
    badge: Option<String>,
    description: Option<String>,
    size: SectionHeaderSize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SectionHeaderSize {
    Large,
    Medium,
    Small,
}

impl ShowcaseSectionHeader {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            badge: None,
            description: None,
            size: SectionHeaderSize::Large,
        }
    }

    pub fn medium(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            badge: None,
            description: None,
            size: SectionHeaderSize::Medium,
        }
    }

    pub fn small(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            badge: None,
            description: None,
            size: SectionHeaderSize::Small,
        }
    }

    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = ui.ctx().armas_theme();
        let (title_size, spacing) = match self.size {
            SectionHeaderSize::Large => (32.0, 12.0),
            SectionHeaderSize::Medium => (24.0, 8.0),
            SectionHeaderSize::Small => (18.0, 6.0),
        };

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = spacing;
            // Title and badge
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(&self.title)
                        .size(title_size)
                        .strong()
                        .color(theme.on_surface()),
                );

                if let Some(badge_text) = &self.badge {
                    ui.add_space(8.0);
                    Badge::new(badge_text)
                        .variant(BadgeVariant::Filled)
                        .color(BadgeColor::Info)
                        .show(ui);
                }
            });

            // Description
            if let Some(desc) = &self.description {
                ui.label(
                    egui::RichText::new(desc)
                        .size(14.0)
                        .color(theme.on_surface_variant()),
                );
            }
        });

        ui.interact(
            ui.min_rect(),
            ui.id().with(&self.title),
            egui::Sense::hover(),
        )
    }
}

/// Quick install section
pub struct QuickInstall {
    command: String,
}

impl QuickInstall {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
        }
    }

    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = ui.ctx().armas_theme();
        let response = GlassPanel::new()
            .opacity(0.05)
            .corner_radius(12.0)
            .show(ui, &theme, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 16.0;

                    // Package icon
                    ui.label(egui::RichText::new("📦").size(20.0));

                    // Command
                    ui.label(
                        egui::RichText::new(&self.command)
                            .monospace()
                            .size(14.0)
                            .color(theme.primary()),
                    );

                    ui.add_space(ui.available_width() - 80.0);

                    // Copy button
                    if Button::new("Copy")
                        .variant(ButtonVariant::Outlined)
                        .show(ui)
                        .clicked()
                    {
                        ui.ctx().copy_text(self.command.clone());
                    }
                });
            });

        response.response
    }
}

/// Related component link card
pub struct RelatedComponentCard {
    name: String,
}

impl RelatedComponentCard {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
        }
    }

    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let theme = ui.ctx().armas_theme();
        let card_response = Card::new()
            .corner_radius(8.0)
            .hover_effect(true)
            .show(ui, &theme, |ui| {
                ui.set_min_size(egui::vec2(150.0, 80.0));

                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.strong(&self.name);
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new("View →")
                            .size(12.0)
                            .color(theme.primary()),
                    );
                });
            });

        card_response.response
    }
}
