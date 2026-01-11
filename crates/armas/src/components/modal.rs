//! Modal/Dialog Components
//!
//! Overlays for focused user interactions
//! Built on top of Card component for consistency

use crate::animation::{Animation, EasingFunction};
use crate::layout::{HStack, Spacer, VStack};
use crate::{Button, ButtonVariant, Card, Theme};
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
    title: Option<String>,
    size: ModalSize,
    closable: bool,
    backdrop_blur: bool,
    // Use animation system for smooth fade-in
    fade_animation: Animation<f32>,
}

impl Modal {
    /// Create a new modal dialog
    pub fn new() -> Self {
        Self {
            title: None,
            size: ModalSize::Medium,
            closable: true,
            backdrop_blur: true,
            // Smooth fade-in animation with cubic easing
            fade_animation: Animation::new(0.0, 1.0, 0.15).with_easing(EasingFunction::CubicOut),
        }
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
        is_open: &mut bool,
        content: impl FnOnce(&mut Ui),
    ) -> ModalResponse {
        let mut response = ModalResponse {
            closed: false,
            backdrop_clicked: false,
        };

        if !*is_open {
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

        let screen_rect = ctx.screen_rect();
        let modal_size = self.size.dimensions(screen_rect.size());

        // Get eased animation value
        let eased = self.fade_animation.value();

        // Draw backdrop
        let backdrop_alpha = (eased * 180.0) as u8;
        let backdrop_color = Color32::from_rgba_unmultiplied(0, 0, 0, backdrop_alpha);

        egui::Area::new(egui::Id::new("modal_backdrop"))
            .order(egui::Order::Foreground)
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .show(ctx, |ui| {
                let backdrop_response = ui.allocate_response(screen_rect.size(), Sense::click());

                ui.painter().rect_filled(screen_rect, 0.0, backdrop_color);

                // Check for backdrop click
                if self.closable && backdrop_response.clicked() {
                    *is_open = false;
                    response.closed = true;
                    response.backdrop_clicked = true;
                    self.fade_animation.reset();
                }
            });

        // Draw modal
        egui::Area::new(egui::Id::new("modal_content"))
            .order(egui::Order::Foreground)
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .show(ctx, |ui| {
                // Use full size immediately to prevent text jitter
                // Only animate opacity instead of scale
                let modal_rect = egui::Rect::from_center_size(screen_rect.center(), modal_size);

                ui.allocate_ui_at_rect(modal_rect, |ui| {
                    // Use Card for consistent styling with elevated appearance
                    Card::new()
                        .stroke(theme.outline().linear_multiply(0.3))
                        .rounding(8.0)
                        .inner_margin(0.0) // We'll handle margins manually for title bar
                        .elevation(3) // Higher elevation for modal
                        .show(ui, theme, |ui| {
                            // Layout: title bar + content
                            VStack::new(8.0).show(ui, |ui| {
                                // Title bar
                                if let Some(title) = &self.title {
                                    HStack::new(16.0).show(ui, |ui| {
                                        ui.heading(title);

                                        Spacer::new().show(ui);

                                        if self.closable {
                                            if Button::new("✕")
                                                .variant(ButtonVariant::Text)
                                                .min_size(vec2(32.0, 32.0))
                                                .show(ui)
                                                .clicked()
                                            {
                                                *is_open = false;
                                                response.closed = true;
                                                self.fade_animation.reset();
                                            }
                                        }
                                    });
                                    ui.separator();
                                }

                                // Content area with padding
                                HStack::new(0.0).show(ui, |ui| {
                                    ui.add_space(16.0);
                                    VStack::new(0.0).show(ui, |ui| {
                                        content(ui);
                                    });
                                    ui.add_space(16.0);
                                });
                            });
                        }); // End Card.show
                });
            });

        // Handle ESC key to close
        if self.closable && ctx.input(|i| i.key_pressed(Key::Escape)) {
            *is_open = false;
            response.closed = true;
            self.fade_animation.reset();
        }

        response
    }
}

impl Default for Modal {
    fn default() -> Self {
        Self::new()
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
    let mut modal = Modal::new().title(title);
    modal.show(ctx, theme, is_open, content)
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

    let mut modal = Modal::new()
        .title(title)
        .size(ModalSize::Small)
        .closable(false);

    modal.show(ctx, theme, is_open, |ui| {
        ui.label(&message);
        ui.add_space(20.0);

        HStack::new(8.0).show(ui, |ui| {
            if Button::new("Cancel")
                .variant(ButtonVariant::Outlined)
                .show(ui)
                .clicked()
            {
                result = ConfirmResponse::Cancel;
                should_close = true;
            }

            Spacer::new().show(ui);

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
