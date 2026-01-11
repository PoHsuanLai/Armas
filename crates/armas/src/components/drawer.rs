//! Drawer Component
//!
//! Slide-out side panels for navigation, settings, or additional content

use crate::layout::{HStack, Spacer, VStack};
use crate::{Animation, Button, ButtonVariant, EasingFunction, Theme};
use egui::{vec2, Align2, Color32, Key, Rect, Sense, Ui, Vec2};

/// Drawer position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerPosition {
    /// Slide from left
    Left,
    /// Slide from right
    Right,
    /// Slide from top
    Top,
    /// Slide from bottom
    Bottom,
}

/// Drawer size
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DrawerSize {
    /// Small drawer (256px)
    Small,
    /// Medium drawer (384px)
    Medium,
    /// Large drawer (512px)
    Large,
    /// Full screen
    Full,
    /// Custom size
    Custom(f32),
}

impl DrawerSize {
    fn size(&self) -> f32 {
        match self {
            DrawerSize::Small => 256.0,
            DrawerSize::Medium => 384.0,
            DrawerSize::Large => 512.0,
            DrawerSize::Full => 0.0, // Will be calculated based on screen
            DrawerSize::Custom(size) => *size,
        }
    }
}

/// Drawer component for slide-out panels
///
/// # Example
///
/// ```rust,no_run
/// use armas::{Drawer, DrawerPosition, DrawerSize};
///
/// let mut drawer = Drawer::new("settings")
///     .position(DrawerPosition::Right)
///     .size(DrawerSize::Medium)
///     .title("Settings");
///
/// let mut is_open = true;
/// drawer.show(ctx, &theme, &mut is_open, |ui| {
///     ui.label("Drawer content here");
/// });
/// ```
pub struct Drawer {
    id: egui::Id,
    position: DrawerPosition,
    size: DrawerSize,
    title: Option<String>,
    closable: bool,
    show_backdrop: bool,
    animation: Animation<f32>,
    is_animating: bool,
}

impl Drawer {
    /// Create a new drawer
    pub fn new(id: impl Into<egui::Id>) -> Self {
        Self {
            id: id.into(),
            position: DrawerPosition::Right,
            size: DrawerSize::Medium,
            title: None,
            closable: true,
            show_backdrop: true,
            animation: Animation::new(0.0, 1.0, 0.3).with_easing(EasingFunction::CubicOut),
            is_animating: false,
        }
    }

    /// Set the drawer position
    pub fn position(mut self, position: DrawerPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the drawer size
    pub fn size(mut self, size: DrawerSize) -> Self {
        self.size = size;
        self
    }

    /// Set the drawer title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set whether the drawer can be closed
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Show or hide the backdrop
    pub fn show_backdrop(mut self, show: bool) -> Self {
        self.show_backdrop = show;
        self
    }

    /// Show the drawer
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        theme: &Theme,
        is_open: &mut bool,
        content: impl FnOnce(&mut Ui),
    ) -> DrawerResponse {
        let mut response = DrawerResponse { closed: false };

        // Handle animation state
        if *is_open && !self.is_animating && self.animation.value() < 1.0 {
            self.animation.reset();
            self.animation.start();
            self.is_animating = true;
        } else if !*is_open && self.animation.value() > 0.0 {
            self.is_animating = true;
        }

        // Update animation
        if self.is_animating {
            let target = if *is_open { 1.0 } else { 0.0 };

            if self.animation.is_running() {
                self.animation.update(ctx.input(|i| i.unstable_dt));
            } else {
                // Animation finished, manually set to target
                if (self.animation.value() - target).abs() > 0.01 {
                    self.animation = Animation::new(self.animation.value(), target, 0.3)
                        .with_easing(EasingFunction::CubicOut);
                    self.animation.start();
                } else {
                    self.is_animating = false;
                }
            }
            ctx.request_repaint();
        }

        let progress = self.animation.value();

        // Don't show if fully closed
        if progress <= 0.0 {
            return response;
        }

        let screen_rect = ctx.screen_rect();

        // Calculate drawer dimensions
        let drawer_size = match self.size {
            DrawerSize::Full => match self.position {
                DrawerPosition::Left | DrawerPosition::Right => screen_rect.width(),
                DrawerPosition::Top | DrawerPosition::Bottom => screen_rect.height(),
            },
            _ => self.size.size(),
        };

        // Draw backdrop
        if self.show_backdrop {
            let backdrop_alpha = (progress * 180.0) as u8;
            let backdrop_color = Color32::from_black_alpha(backdrop_alpha);

            egui::Area::new(self.id.with("backdrop"))
                .order(egui::Order::Middle)
                .interactable(true)
                .show(ctx, |ui| {
                    let backdrop = ui.allocate_response(screen_rect.size(), Sense::click());
                    ui.painter().rect_filled(screen_rect, 0.0, backdrop_color);

                    if backdrop.clicked() && self.closable && *is_open {
                        *is_open = false;
                        response.closed = true;
                    }
                });
        }

        // Calculate drawer position based on animation progress
        let (drawer_rect, offset) = match self.position {
            DrawerPosition::Left => {
                let x_offset = -drawer_size * (1.0 - progress);
                let rect = Rect::from_min_size(
                    screen_rect.left_top() + vec2(x_offset, 0.0),
                    vec2(drawer_size, screen_rect.height()),
                );
                (rect, vec2(x_offset, 0.0))
            }
            DrawerPosition::Right => {
                let x_offset = drawer_size * (1.0 - progress);
                let rect = Rect::from_min_size(
                    screen_rect.right_top() + vec2(-drawer_size + x_offset, 0.0),
                    vec2(drawer_size, screen_rect.height()),
                );
                (rect, vec2(x_offset, 0.0))
            }
            DrawerPosition::Top => {
                let y_offset = -drawer_size * (1.0 - progress);
                let rect = Rect::from_min_size(
                    screen_rect.left_top() + vec2(0.0, y_offset),
                    vec2(screen_rect.width(), drawer_size),
                );
                (rect, vec2(0.0, y_offset))
            }
            DrawerPosition::Bottom => {
                let y_offset = drawer_size * (1.0 - progress);
                let rect = Rect::from_min_size(
                    screen_rect.left_bottom() + vec2(0.0, -drawer_size + y_offset),
                    vec2(screen_rect.width(), drawer_size),
                );
                (rect, vec2(0.0, y_offset))
            }
        };

        // Draw drawer
        egui::Area::new(self.id)
            .order(egui::Order::Foreground)
            .fixed_pos(drawer_rect.min)
            .show(ctx, |ui| {
                ui.set_clip_rect(drawer_rect);

                // Background
                let frame = egui::Frame::none().fill(theme.surface()).inner_margin(0.0);

                frame.show(ui, |ui| {
                    ui.set_min_size(drawer_rect.size());
                    ui.set_max_size(drawer_rect.size());

                    VStack::new(8.0).show(ui, |ui| {
                        // Title bar
                        if self.title.is_some() || self.closable {
                            HStack::new(16.0).show(ui, |ui| {
                                if let Some(title) = &self.title {
                                    ui.heading(title);
                                }

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
                });

                // Draw edge shadow
                let shadow_color = Color32::from_black_alpha(60);
                let shadow_width = 8.0;

                match self.position {
                    DrawerPosition::Left => {
                        let shadow_rect = Rect::from_min_size(
                            drawer_rect.right_top(),
                            vec2(shadow_width, drawer_rect.height()),
                        );
                        ui.painter().rect_filled(shadow_rect, 0.0, shadow_color);
                    }
                    DrawerPosition::Right => {
                        let shadow_rect = Rect::from_min_size(
                            drawer_rect.left_top() - vec2(shadow_width, 0.0),
                            vec2(shadow_width, drawer_rect.height()),
                        );
                        ui.painter().rect_filled(shadow_rect, 0.0, shadow_color);
                    }
                    DrawerPosition::Top => {
                        let shadow_rect = Rect::from_min_size(
                            drawer_rect.left_bottom(),
                            vec2(drawer_rect.width(), shadow_width),
                        );
                        ui.painter().rect_filled(shadow_rect, 0.0, shadow_color);
                    }
                    DrawerPosition::Bottom => {
                        let shadow_rect = Rect::from_min_size(
                            drawer_rect.left_top() - vec2(0.0, shadow_width),
                            vec2(drawer_rect.width(), shadow_width),
                        );
                        ui.painter().rect_filled(shadow_rect, 0.0, shadow_color);
                    }
                }
            });

        // Handle ESC key to close
        if self.closable && *is_open && ctx.input(|i| i.key_pressed(Key::Escape)) {
            *is_open = false;
            response.closed = true;
        }

        response
    }
}

/// Response from a drawer
#[derive(Debug, Clone, Copy)]
pub struct DrawerResponse {
    /// Whether the drawer was closed
    pub closed: bool,
}
