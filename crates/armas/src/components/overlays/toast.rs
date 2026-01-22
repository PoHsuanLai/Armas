//! Toast/Notification Components
//!
//! Temporary notification messages with auto-dismiss
//! Built on top of Card component for consistency

use crate::animation::SpringAnimation;
use crate::ext::ArmasContextExt;
use crate::{Badge, Button, ButtonVariant, Card, CardVariant, Theme};
use egui::{vec2, Align2, Color32, Id, Sense, Vec2};
use std::collections::VecDeque;

/// Toast notification variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastVariant {
    /// Default notification
    #[default]
    Default,
    /// Destructive/error notification (red)
    Destructive,
}

impl ToastVariant {
    fn icon(&self) -> &'static str {
        match self {
            ToastVariant::Default => "ℹ",
            ToastVariant::Destructive => "✕",
        }
    }

    fn color(&self, theme: &Theme) -> Color32 {
        match self {
            ToastVariant::Default => theme.foreground(),
            ToastVariant::Destructive => theme.destructive(),
        }
    }
}

/// Position for toast notifications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastPosition {
    /// Top left corner
    TopLeft,
    /// Top center
    TopCenter,
    /// Top right corner
    TopRight,
    /// Bottom left corner
    BottomLeft,
    /// Bottom center
    BottomCenter,
    /// Bottom right corner
    BottomRight,
}

impl ToastPosition {
    fn anchor(&self) -> Align2 {
        match self {
            ToastPosition::TopLeft => Align2::LEFT_TOP,
            ToastPosition::TopCenter => Align2::CENTER_TOP,
            ToastPosition::TopRight => Align2::RIGHT_TOP,
            ToastPosition::BottomLeft => Align2::LEFT_BOTTOM,
            ToastPosition::BottomCenter => Align2::CENTER_BOTTOM,
            ToastPosition::BottomRight => Align2::RIGHT_BOTTOM,
        }
    }

    fn offset(&self, index: usize, toast_height: f32) -> Vec2 {
        let spacing = 10.0;
        let margin = 20.0;
        let y_offset = (toast_height + spacing) * index as f32;

        match self {
            ToastPosition::TopLeft => vec2(margin, margin + y_offset),
            ToastPosition::TopCenter => vec2(0.0, margin + y_offset),
            ToastPosition::TopRight => vec2(-margin, margin + y_offset),
            ToastPosition::BottomLeft => vec2(margin, -margin - y_offset),
            ToastPosition::BottomCenter => vec2(0.0, -margin - y_offset),
            ToastPosition::BottomRight => vec2(-margin, -margin - y_offset),
        }
    }
}

/// Individual toast notification
#[derive(Clone)]
struct Toast {
    id: u64,
    title: Option<String>,
    message: String,
    variant: ToastVariant,
    custom_color: Option<Color32>,
    duration_secs: f32,
    created_at: f64,
    slide_animation: SpringAnimation,
    dismissible: bool,
}

impl Toast {
    fn new(message: impl Into<String>, variant: ToastVariant, current_time: f64) -> Self {
        static mut NEXT_ID: u64 = 0;
        let id = unsafe {
            NEXT_ID += 1;
            NEXT_ID
        };

        Self {
            id,
            title: None,
            message: message.into(),
            variant,
            custom_color: None,
            duration_secs: 3.0,
            created_at: current_time,
            slide_animation: SpringAnimation::new(0.0, 1.0).params(250.0, 25.0),
            dismissible: true,
        }
    }

    #[allow(dead_code)]
    fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    #[allow(dead_code)]
    fn with_duration_secs(mut self, duration_secs: f32) -> Self {
        self.duration_secs = duration_secs;
        self
    }

    #[allow(dead_code)]
    fn with_dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    fn is_expired(&self, current_time: f64) -> bool {
        (current_time - self.created_at) as f32 >= self.duration_secs
    }

    fn progress(&self, current_time: f64) -> f32 {
        ((current_time - self.created_at) as f32 / self.duration_secs).min(1.0)
    }

    fn color(&self, theme: &Theme) -> Color32 {
        self.custom_color.unwrap_or_else(|| self.variant.color(theme))
    }
}

/// Toast notification manager
#[derive(Clone)]
pub struct ToastManager {
    toasts: VecDeque<Toast>,
    position: ToastPosition,
    max_toasts: usize,
}

impl ToastManager {
    /// Create a new toast manager
    pub fn new() -> Self {
        Self {
            toasts: VecDeque::new(),
            position: ToastPosition::TopRight,
            max_toasts: 5,
        }
    }

    /// Set the position where toasts appear
    pub fn position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the maximum number of toasts to show at once
    pub fn max_toasts(mut self, max: usize) -> Self {
        self.max_toasts = max;
        self
    }

    /// Add a new toast notification
    pub fn add(&mut self, message: impl Into<String>, variant: ToastVariant, current_time: f64) {
        let toast = Toast::new(message, variant, current_time);
        self.toasts.push_back(toast);

        while self.toasts.len() > self.max_toasts {
            self.toasts.pop_front();
        }
    }

    /// Add a default toast
    pub fn toast(&mut self, message: impl Into<String>) {
        self.add(message, ToastVariant::Default, 0.0);
    }

    /// Add a destructive/error toast
    pub fn error(&mut self, message: impl Into<String>) {
        self.add(message, ToastVariant::Destructive, 0.0);
    }

    /// Add a custom toast with builder pattern
    pub fn custom(&mut self) -> ToastBuilder<'_> {
        ToastBuilder {
            manager: self,
            toast: None,
        }
    }

    /// Show all toasts
    pub fn show(&mut self, ctx: &egui::Context) {
        let theme = ctx.armas_theme();
        let current_time = ctx.input(|i| i.time);

        // Fix newly created toasts (created_at == 0.0)
        for toast in self.toasts.iter_mut() {
            if toast.created_at == 0.0 {
                toast.created_at = current_time;
            }
        }

        // Remove expired toasts
        self.toasts.retain(|toast| !toast.is_expired(current_time));

        if self.toasts.is_empty() {
            return;
        }

        // Animate and draw toasts
        let toast_height = 70.0;
        let mut to_remove = Vec::new();
        let position = self.position;

        // Update animations first
        let dt = ctx.input(|i| i.unstable_dt);
        for toast in self.toasts.iter_mut() {
            toast.slide_animation.update(dt);
            if !toast.slide_animation.is_settled(0.001, 0.001) {
                ctx.request_repaint();
            }
        }

        // Clone toast data for rendering to avoid borrow conflicts
        let toasts_to_render: Vec<_> = self.toasts.iter().cloned().collect();

        for (index, toast) in toasts_to_render.iter().enumerate() {
            // Fade out animation near end
            let progress = toast.progress(current_time);
            let fade_start = 0.9;
            let opacity = if progress > fade_start {
                1.0 - ((progress - fade_start) / (1.0 - fade_start))
            } else {
                1.0
            };

            // Slide in animation using spring
            let slide_progress = toast.slide_animation.value;

            let offset = position.offset(index, toast_height);
            let slide_offset = match position {
                ToastPosition::TopRight | ToastPosition::BottomRight => {
                    vec2(50.0 * (1.0 - slide_progress), 0.0)
                }
                ToastPosition::TopLeft | ToastPosition::BottomLeft => {
                    vec2(-50.0 * (1.0 - slide_progress), 0.0)
                }
                _ => vec2(0.0, 0.0),
            };

            let dismissed = Self::show_toast_static(
                ctx,
                &theme,
                toast,
                position,
                offset + slide_offset,
                opacity,
                current_time,
            );

            if dismissed {
                to_remove.push(toast.id);
            }
        }

        // Remove dismissed toasts
        for id in to_remove {
            self.toasts.retain(|t| t.id != id);
        }

        // Request repaint if any toasts are active
        if !self.toasts.is_empty() {
            ctx.request_repaint();
        }
    }

    fn show_toast_static(
        ctx: &egui::Context,
        theme: &Theme,
        toast: &Toast,
        position: ToastPosition,
        offset: Vec2,
        opacity: f32,
        current_time: f64,
    ) -> bool {
        let mut dismissed = false;

        egui::Area::new(Id::new("toast").with(toast.id))
            .order(egui::Order::Foreground)
            .anchor(position.anchor(), offset)
            .show(ctx, |ui| {
                ui.set_opacity(opacity);

                let width = 300.0;
                let accent_color = toast.color(theme);

                // Use Card for consistent styling
                Card::new()
                    .variant(CardVariant::Elevated)
                    .width(width)
                    .stroke(theme.border().linear_multiply(0.3))
                    .corner_radius(8.0)
                    .inner_margin(12.0)
                    .show(ui, theme, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 8.0;
                            // Icon badge
                            Badge::new(toast.variant.icon())
                                .color(accent_color)
                                .show(ui);

                            // Content
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 0.0;
                                ui.set_width(width - 100.0);

                                if let Some(title) = &toast.title {
                                    ui.strong(title);
                                }
                                ui.label(&toast.message);
                            });

                            // Close button
                            if toast.dismissible
                                && Button::new("✕")
                                    .variant(ButtonVariant::Text)
                                    .min_size(vec2(24.0, 24.0))
                                    .show(ui)
                                    .clicked()
                            {
                                dismissed = true;
                            }
                        });

                        // Progress bar
                        let progress = toast.progress(current_time).min(1.0);
                        if progress < 1.0 {
                            ui.add_space(4.0);
                            let progress_height = 3.0;
                            let (rect, _) = ui.allocate_exact_size(
                                vec2(ui.available_width(), progress_height),
                                Sense::hover(),
                            );

                            // Background
                            ui.painter().rect_filled(rect, 2.0, theme.muted());

                            // Progress fill
                            let fill_width = rect.width() * progress;
                            let fill_rect = egui::Rect::from_min_size(
                                rect.min,
                                vec2(fill_width, progress_height),
                            );

                            ui.painter().rect_filled(fill_rect, 2.0, accent_color);
                        }
                    });
            });

        dismissed
    }
}

impl Default for ToastManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for custom toast notifications
pub struct ToastBuilder<'a> {
    manager: &'a mut ToastManager,
    toast: Option<Toast>,
}

impl<'a> ToastBuilder<'a> {
    /// Set the toast message
    pub fn message(mut self, message: impl Into<String>) -> Self {
        if let Some(toast) = &mut self.toast {
            toast.message = message.into();
        } else {
            self.toast = Some(Toast::new(message, ToastVariant::Default, 0.0));
        }
        self
    }

    /// Set the toast title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        if let Some(toast) = &mut self.toast {
            toast.title = Some(title.into());
        }
        self
    }

    /// Set the toast variant
    pub fn variant(mut self, variant: ToastVariant) -> Self {
        if let Some(toast) = &mut self.toast {
            toast.variant = variant;
        } else {
            self.toast = Some(Toast::new("", variant, 0.0));
        }
        self
    }

    /// Make this a destructive toast
    pub fn destructive(mut self) -> Self {
        if let Some(toast) = &mut self.toast {
            toast.variant = ToastVariant::Destructive;
        } else {
            self.toast = Some(Toast::new("", ToastVariant::Destructive, 0.0));
        }
        self
    }

    /// Set custom color (overrides variant)
    pub fn color(mut self, color: Color32) -> Self {
        if let Some(toast) = &mut self.toast {
            toast.custom_color = Some(color);
        }
        self
    }

    /// Set the display duration
    pub fn duration(mut self, duration: std::time::Duration) -> Self {
        if let Some(toast) = &mut self.toast {
            toast.duration_secs = duration.as_secs_f32();
        }
        self
    }

    /// Set whether the toast can be manually dismissed
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        if let Some(toast) = &mut self.toast {
            toast.dismissible = dismissible;
        }
        self
    }

    /// Add the toast to the manager
    pub fn show(self) {
        if let Some(toast) = self.toast {
            self.manager.toasts.push_back(toast);
            while self.manager.toasts.len() > self.manager.max_toasts {
                self.manager.toasts.pop_front();
            }
        }
    }
}
