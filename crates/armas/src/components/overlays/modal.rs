//! Modal/Dialog Components
//!
//! Overlays for focused user interactions
//! Built on top of Card component for consistency

use crate::animation::{Animation, EasingFunction};
use crate::{Button, ButtonVariant, Card, CardVariant, Theme};
use egui::{vec2, Align2, Color32, Key, Sense, Ui, Vec2};

/// Modal size presets
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModalSize {
    /// Small modal (400px wide)
    Small,
    /// Medium modal (600px wide)
    Medium,
    /// Large modal (800px wide)
    Large,
    /// Full screen modal
    FullScreen,
    /// Custom size
    Custom(f32, f32),
}

impl ModalSize {
    fn dimensions(&self, screen_size: Vec2) -> Vec2 {
        match self {
            ModalSize::Small => vec2(400.0, 300.0),
            ModalSize::Medium => vec2(600.0, 400.0),
            ModalSize::Large => vec2(800.0, 500.0),
            ModalSize::FullScreen => screen_size * 0.95,
            ModalSize::Custom(w, h) => vec2(*w, *h),
        }
    }
}

/// Modal dialog component
pub struct Modal {
    id: egui::Id,
    title: Option<String>,
    size: ModalSize,
    closable: bool,
    backdrop_blur: bool,
    // Use animation system for smooth fade-in
    fade_animation: Animation<f32>,
    // Internal state management
    is_open: Option<bool>, // None = use internal state, Some = external control
}

impl Modal {
    /// Create a new modal dialog with a unique ID
    pub fn new(id: impl Into<egui::Id>) -> Self {
        Self {
            id: id.into(),
            title: None,
            size: ModalSize::Medium,
            closable: true,
            backdrop_blur: true,
            // Smooth fade-in animation with cubic easing
            fade_animation: Animation::new(0.0, 1.0, 0.15).easing(EasingFunction::CubicOut),
            is_open: None, // Use internal state by default
        }
    }

    /// Set the modal to be open (for external control)
    pub fn open(mut self, is_open: bool) -> Self {
        self.is_open = Some(is_open);
        self
    }

    /// Set the modal title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the modal size
    pub fn size(mut self, size: ModalSize) -> Self {
        self.size = size;
        self
    }

    /// Set whether the modal can be closed with ESC or backdrop click
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Set whether to blur the backdrop
    pub fn backdrop_blur(mut self, blur: bool) -> Self {
        self.backdrop_blur = blur;
        self
    }

    /// Show the modal dialog
    ///
    /// Returns `ModalResponse` with information about user interaction
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        theme: &Theme,
        content: impl FnOnce(&mut Ui),
    ) -> ModalResponse {
        let mut response = ModalResponse {
            closed: false,
            backdrop_clicked: false,
        };

        // Load state from egui memory if not externally controlled
        let state_id = self.id.with("modal_state");
        let mut is_open = if let Some(external_open) = self.is_open {
            external_open
        } else {
            ctx.data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false))
        };

        if !is_open {
            self.fade_animation.reset();
            return response;
        }

        // Start animation if not started
        if !self.fade_animation.is_running() && !self.fade_animation.is_complete() {
            self.fade_animation.start();
        }

        // Update animation
        let dt = ctx.input(|i| i.unstable_dt);
        self.fade_animation.update(dt);

        if self.fade_animation.is_running() {
            ctx.request_repaint();
        }

        let screen_rect = ctx.content_rect();
        let modal_size = self.size.dimensions(screen_rect.size());

        // Get eased animation value
        let eased = self.fade_animation.value();

        // Draw backdrop using theme background color
        let backdrop_base = theme.background();
        let backdrop_alpha = (eased * 180.0) as u8;
        let backdrop_color = Color32::from_rgba_unmultiplied(
            backdrop_base.r(),
            backdrop_base.g(),
            backdrop_base.b(),
            backdrop_alpha,
        );

        let backdrop_id = self.id.with("modal_backdrop");
        egui::Area::new(backdrop_id)
            .order(egui::Order::Foreground)
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .show(ctx, |ui| {
                let backdrop_response = ui.allocate_response(screen_rect.size(), Sense::click());

                ui.painter().rect_filled(screen_rect, 0.0, backdrop_color);

                // Check for backdrop click
                if self.closable && backdrop_response.clicked() {
                    is_open = false;
                    response.closed = true;
                    response.backdrop_clicked = true;
                    self.fade_animation.reset();
                }
            });

        // Draw modal
        let content_id = self.id.with("modal_content");
        egui::Area::new(content_id)
            .order(egui::Order::Foreground)
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .show(ctx, |ui| {
                // Use full size immediately to prevent text jitter
                // Only animate opacity instead of scale
                let modal_rect = egui::Rect::from_center_size(screen_rect.center(), modal_size);

                ui.scope_builder(egui::UiBuilder::new().max_rect(modal_rect), |ui| {
                    // Use Card with MD3 Elevated variant for visual separation
                    Card::new()
                        .variant(CardVariant::Elevated) // Use Elevated for strong visual separation
                        .stroke(theme.border().linear_multiply(0.3))
                        .corner_radius(theme.spacing.corner_radius_small as f32)
                        .inner_margin(0.0) // We'll handle margins manually for title bar
                        .show(ui, theme, |ui| {
                            // Layout: title bar + content
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 0.0;
                                // Title bar
                                if let Some(title) = &self.title {
                                    ui.add_space(theme.spacing.md);
                                    ui.horizontal(|ui| {
                                        ui.spacing_mut().item_spacing.x = 0.0;
                                        ui.add_space(theme.spacing.lg);
                                        ui.heading(title);

                                        ui.allocate_space(ui.available_size());

                                        if self.closable
                                            && Button::new("✕")
                                                .variant(ButtonVariant::Text)
                                                .min_size(vec2(32.0, 32.0))
                                                .show(ui)
                                                .clicked()
                                        {
                                            is_open = false;
                                            response.closed = true;
                                            self.fade_animation.reset();
                                        }
                                        ui.add_space(theme.spacing.lg);
                                    });
                                    ui.add_space(theme.spacing.md);
                                    ui.separator();
                                }

                                // Content area with padding
                                ui.add_space(theme.spacing.md);
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing.x = 0.0;
                                    ui.add_space(theme.spacing.lg);
                                    ui.vertical(|ui| {
                                        ui.spacing_mut().item_spacing.y = 0.0;
                                        content(ui);
                                    });
                                    ui.add_space(theme.spacing.lg);
                                });
                                ui.add_space(theme.spacing.md);
                            });
                        }); // End Card.show
                });
            });

        // Handle ESC key to close
        if self.closable && ctx.input(|i| i.key_pressed(Key::Escape)) {
            is_open = false;
            response.closed = true;
            self.fade_animation.reset();
        }

        // Persist state if not externally controlled
        if self.is_open.is_none() {
            ctx.data_mut(|d| d.insert_temp(state_id, is_open));
        }

        response
    }
}

impl Default for Modal {
    fn default() -> Self {
        Self::new("modal")
    }
}

/// Response from a modal dialog
#[derive(Debug, Clone, Copy)]
pub struct ModalResponse {
    /// Whether the modal was closed this frame
    pub closed: bool,
    /// Whether the backdrop was clicked (only true if modal was closable)
    pub backdrop_clicked: bool,
}

/// Simple dialog with title and content
pub fn dialog(
    ctx: &egui::Context,
    theme: &Theme,
    is_open: &mut bool,
    title: impl Into<String>,
    content: impl FnOnce(&mut Ui),
) -> ModalResponse {
    let mut modal = Modal::new("dialog").title(title).open(*is_open);
    let response = modal.show(ctx, theme, content);
    *is_open = !response.closed && *is_open;
    response
}

/// Confirmation dialog with Yes/No buttons
pub fn confirm_dialog(
    ctx: &egui::Context,
    theme: &Theme,
    is_open: &mut bool,
    title: impl Into<String>,
    message: impl Into<String>,
) -> ConfirmResponse {
    let message = message.into();
    let mut result = ConfirmResponse::None;
    let mut should_close = false;

    let mut modal = Modal::new("confirm_dialog")
        .title(title)
        .size(ModalSize::Small)
        .closable(false)
        .open(*is_open);

    modal.show(ctx, theme, |ui| {
        ui.label(&message);
        ui.add_space(theme.spacing.lg);

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = theme.spacing.sm;
            if Button::new("Cancel")
                .variant(ButtonVariant::Outlined)
                .show(ui)
                .clicked()
            {
                result = ConfirmResponse::Cancel;
                should_close = true;
            }

            ui.allocate_space(ui.available_size());

            if Button::new("Confirm")
                .variant(ButtonVariant::Filled)
                .show(ui)
                .clicked()
            {
                result = ConfirmResponse::Confirm;
                should_close = true;
            }
        });
    });

    if should_close {
        *is_open = false;
    }

    result
}

/// Response from a confirmation dialog
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmResponse {
    /// No action taken
    None,
    /// User confirmed
    Confirm,
    /// User cancelled
    Cancel,
}
