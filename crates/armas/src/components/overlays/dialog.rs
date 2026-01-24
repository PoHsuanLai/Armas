//! Dialog Components
//!
//! Overlays for focused user interactions.
//! Styled to match shadcn/ui Dialog conventions.

use crate::animation::{Animation, EasingFunction};
use crate::Theme;
use egui::{vec2, Align, Align2, Color32, Key, Layout, Pos2, Sense, Stroke, Ui};

// shadcn/ui Dialog constants
const CORNER_RADIUS: f32 = 8.0; // rounded-lg
const PADDING: f32 = 24.0; // p-6
const GAP: f32 = 16.0; // gap-4
const HEADER_GAP: f32 = 8.0; // gap-2
const FOOTER_GAP: f32 = 8.0; // gap-2
const OVERLAY_ALPHA: u8 = 128; // bg-black/50
const CLOSE_BUTTON_SIZE: f32 = 16.0;

/// Dialog size presets (max-width based like shadcn)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DialogSize {
    /// Small dialog (384px - sm:max-w-sm)
    Small,
    /// Medium dialog (512px - sm:max-w-lg, default)
    Medium,
    /// Large dialog (672px - sm:max-w-2xl)
    Large,
    /// Extra large dialog (896px - sm:max-w-4xl)
    ExtraLarge,
    /// Full screen dialog
    FullScreen,
    /// Custom max width
    Custom(f32),
}

impl DialogSize {
    fn max_width(&self, screen_width: f32) -> f32 {
        let max = screen_width - 32.0; // max-w-[calc(100%-2rem)]
        match self {
            DialogSize::Small => 384.0_f32.min(max),
            DialogSize::Medium => 512.0_f32.min(max),
            DialogSize::Large => 672.0_f32.min(max),
            DialogSize::ExtraLarge => 896.0_f32.min(max),
            DialogSize::FullScreen => max,
            DialogSize::Custom(w) => w.min(max),
        }
    }
}

/// Dialog component styled like shadcn/ui Dialog
pub struct Dialog {
    id: egui::Id,
    title: Option<String>,
    description: Option<String>,
    size: DialogSize,
    closable: bool,
    fade_animation: Animation<f32>,
    is_open: Option<bool>,
}

impl Dialog {
    /// Create a new dialog with a unique ID
    pub fn new(id: impl Into<egui::Id>) -> Self {
        Self {
            id: id.into(),
            title: None,
            description: None,
            size: DialogSize::Medium,
            closable: true,
            fade_animation: Animation::new(0.0, 1.0, 0.15).easing(EasingFunction::CubicOut),
            is_open: None,
        }
    }

    /// Set the dialog to be open (for external control)
    pub fn open(mut self, is_open: bool) -> Self {
        self.is_open = Some(is_open);
        self
    }

    /// Set the dialog title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the dialog description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the dialog size
    pub fn size(mut self, size: DialogSize) -> Self {
        self.size = size;
        self
    }

    /// Set whether the dialog can be closed with ESC or backdrop click
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Show the dialog
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        theme: &Theme,
        content: impl FnOnce(&mut Ui),
    ) -> DialogResponse {
        let mut response = DialogResponse {
            closed: false,
            backdrop_clicked: false,
        };

        let state_id = self.id.with("dialog_state");
        let mut is_open = if let Some(external_open) = self.is_open {
            external_open
        } else {
            ctx.data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false))
        };

        if !is_open {
            self.fade_animation.reset();
            return response;
        }

        if !self.fade_animation.is_running() && !self.fade_animation.is_complete() {
            self.fade_animation.start();
        }

        let dt = ctx.input(|i| i.unstable_dt);
        self.fade_animation.update(dt);

        if self.fade_animation.is_running() {
            ctx.request_repaint();
        }

        let screen_rect = ctx.content_rect();
        let dialog_width = self.size.max_width(screen_rect.width());
        let eased = self.fade_animation.value();

        // Draw backdrop - bg-black/50
        let backdrop_alpha = (eased * OVERLAY_ALPHA as f32) as u8;
        let backdrop_color = Color32::from_rgba_unmultiplied(0, 0, 0, backdrop_alpha);

        let backdrop_id = self.id.with("dialog_backdrop");
        egui::Area::new(backdrop_id)
            .order(egui::Order::Foreground)
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .show(ctx, |ui| {
                // Only capture clicks if closable, otherwise just block input
                let sense = if self.closable {
                    Sense::click()
                } else {
                    Sense::hover()
                };
                let backdrop_response = ui.allocate_response(screen_rect.size(), sense);
                ui.painter().rect_filled(screen_rect, 0.0, backdrop_color);

                if self.closable && backdrop_response.clicked() {
                    is_open = false;
                    response.closed = true;
                    response.backdrop_clicked = true;
                    self.fade_animation.reset();
                }
            });

        // Draw dialog content
        let content_id = self.id.with("dialog_content");
        egui::Area::new(content_id)
            .order(egui::Order::Foreground)
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .show(ctx, |ui| {
                let frame = egui::Frame::NONE
                    .fill(theme.background())
                    .stroke(Stroke::new(1.0, theme.border()))
                    .corner_radius(CORNER_RADIUS)
                    .shadow(egui::epaint::Shadow {
                        offset: [0, 4],
                        blur: 16,
                        spread: 0,
                        color: Color32::from_black_alpha(60),
                    })
                    .inner_margin(PADDING);

                frame.show(ui, |ui| {
                    ui.set_width(dialog_width);
                    ui.spacing_mut().item_spacing.y = GAP;

                    // Header section
                    let has_header = self.title.is_some() || self.description.is_some();
                    if has_header || self.closable {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = HEADER_GAP;

                                if let Some(title) = &self.title {
                                    ui.label(
                                        egui::RichText::new(title)
                                            .size(18.0)
                                            .strong()
                                            .color(theme.foreground()),
                                    );
                                }

                                if let Some(desc) = &self.description {
                                    ui.label(
                                        egui::RichText::new(desc)
                                            .size(14.0)
                                            .color(theme.muted_foreground()),
                                    );
                                }
                            });

                            ui.allocate_space(ui.available_size() - vec2(CLOSE_BUTTON_SIZE + 4.0, 0.0));

                            if self.closable {
                                let (close_rect, close_response) = ui.allocate_exact_size(
                                    vec2(CLOSE_BUTTON_SIZE, CLOSE_BUTTON_SIZE),
                                    Sense::click(),
                                );

                                let close_color = if close_response.hovered() {
                                    theme.foreground()
                                } else {
                                    theme.muted_foreground()
                                };

                                let center = close_rect.center();
                                let half = CLOSE_BUTTON_SIZE * 0.35;
                                ui.painter().line_segment(
                                    [
                                        Pos2::new(center.x - half, center.y - half),
                                        Pos2::new(center.x + half, center.y + half),
                                    ],
                                    Stroke::new(1.5, close_color),
                                );
                                ui.painter().line_segment(
                                    [
                                        Pos2::new(center.x + half, center.y - half),
                                        Pos2::new(center.x - half, center.y + half),
                                    ],
                                    Stroke::new(1.5, close_color),
                                );

                                if close_response.clicked() {
                                    is_open = false;
                                    response.closed = true;
                                    self.fade_animation.reset();
                                }
                            }
                        });
                    }

                    content(ui);
                });
            });

        if self.closable && ctx.input(|i| i.key_pressed(Key::Escape)) {
            is_open = false;
            response.closed = true;
            self.fade_animation.reset();
        }

        // Re-check state after content runs (content may have modified it)
        if self.is_open.is_none() {
            let state_after_content = ctx.data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(true));
            if !state_after_content && is_open {
                // Content closed the dialog
                response.closed = true;
                self.fade_animation.reset();
            }
        }

        response
    }
}

impl Default for Dialog {
    fn default() -> Self {
        Self::new("dialog")
    }
}

/// Response from a dialog
#[derive(Debug, Clone, Copy)]
pub struct DialogResponse {
    /// Whether the dialog was closed this frame
    pub closed: bool,
    /// Whether the backdrop was clicked
    pub backdrop_clicked: bool,
}

// ============================================================================
// Helper functions for building dialog content
// ============================================================================

/// Helper to render a dialog footer (right-aligned buttons)
pub fn dialog_footer(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.spacing_mut().item_spacing.x = FOOTER_GAP;
        content(ui);
    });
}
