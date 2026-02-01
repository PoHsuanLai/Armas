//! Toast/Notification Components
//!
//! Toast notifications styled like shadcn/ui Sonner (Toast).
//! Supports multiple positions, variants, and auto-dismiss with progress indicators.
//!
//! # Example
//!
//! ```rust,no_run
//! # use egui::Context;
//! # fn example(ctx: &Context) {
//! use armas_basic::components::{ToastManager, ToastVariant};
//!
//! let mut toasts = ToastManager::new();
//!
//! // Simple toast
//! toasts.toast("Changes saved");
//!
//! // Error toast
//! toasts.error("Something went wrong");
//!
//! // Custom toast
//! toasts.custom()
//!     .title("Scheduled")
//!     .message("Your message has been scheduled")
//!     .duration(std::time::Duration::from_secs(5))
//!     .show();
//!
//! // Render all toasts
//! toasts.show(ctx);
//! # }
//! ```

use crate::animation::SpringAnimation;
use crate::components::button::IconButton;
use crate::ext::ArmasContextExt;
use crate::icon::{render_icon, WindowIcon};
use crate::{ButtonVariant, Card, CardVariant, Theme};
use egui::{vec2, Align2, Color32, Id, Sense, Vec2};
use std::collections::VecDeque;

// shadcn Sonner (Toast) constants
const TOAST_WIDTH: f32 = 356.0; // w-[356px]
const TOAST_PADDING: f32 = 16.0; // p-4
const TOAST_CORNER_RADIUS: f32 = 8.0; // rounded-lg
const TOAST_HEIGHT: f32 = 70.0; // Approximate height
const TOAST_SPACING: f32 = 8.0; // gap-2
const TOAST_MARGIN: f32 = 16.0; // 1rem margin
const DEFAULT_DURATION_SECS: f32 = 4.0; // 4s default
const PROGRESS_HEIGHT: f32 = 2.0; // h-0.5
const MAX_TOASTS: usize = 5;

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
    const fn icon(self) -> WindowIcon {
        match self {
            Self::Default => WindowIcon::Info,
            Self::Destructive => WindowIcon::Error,
        }
    }

    const fn color(self, theme: &Theme) -> Color32 {
        match self {
            Self::Default => theme.foreground(),
            Self::Destructive => theme.destructive(),
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
    const fn anchor(self) -> Align2 {
        match self {
            Self::TopLeft => Align2::LEFT_TOP,
            Self::TopCenter => Align2::CENTER_TOP,
            Self::TopRight => Align2::RIGHT_TOP,
            Self::BottomLeft => Align2::LEFT_BOTTOM,
            Self::BottomCenter => Align2::CENTER_BOTTOM,
            Self::BottomRight => Align2::RIGHT_BOTTOM,
        }
    }

    fn offset(self, index: usize, toast_height: f32) -> Vec2 {
        let y_offset = (toast_height + TOAST_SPACING) * index as f32;

        match self {
            Self::TopLeft => vec2(TOAST_MARGIN, TOAST_MARGIN + y_offset),
            Self::TopCenter => vec2(0.0, TOAST_MARGIN + y_offset),
            Self::TopRight => vec2(-TOAST_MARGIN, TOAST_MARGIN + y_offset),
            Self::BottomLeft => vec2(TOAST_MARGIN, -TOAST_MARGIN - y_offset),
            Self::BottomCenter => vec2(0.0, -TOAST_MARGIN - y_offset),
            Self::BottomRight => vec2(-TOAST_MARGIN, -TOAST_MARGIN - y_offset),
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

use std::sync::atomic::{AtomicU64, Ordering};

impl Toast {
    fn new(message: impl Into<String>, variant: ToastVariant, current_time: f64) -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed) + 1;

        Self {
            id,
            title: None,
            message: message.into(),
            variant,
            custom_color: None,
            duration_secs: DEFAULT_DURATION_SECS,
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
    const fn with_duration_secs(mut self, duration_secs: f32) -> Self {
        self.duration_secs = duration_secs;
        self
    }

    #[allow(dead_code)]
    const fn with_dismissible(mut self, dismissible: bool) -> Self {
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
        self.custom_color
            .unwrap_or_else(|| self.variant.color(theme))
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
    #[must_use]
    pub const fn new() -> Self {
        Self {
            toasts: VecDeque::new(),
            position: ToastPosition::BottomRight, // shadcn default
            max_toasts: MAX_TOASTS,
        }
    }

    /// Set the position where toasts appear
    #[must_use]
    pub const fn position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the maximum number of toasts to show at once
    #[must_use]
    pub const fn max_toasts(mut self, max: usize) -> Self {
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
    pub const fn custom(&mut self) -> ToastBuilder<'_> {
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
        for toast in &mut self.toasts {
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
        let mut to_remove = Vec::new();
        let position = self.position;

        // Update animations first
        let dt = ctx.input(|i| i.unstable_dt);
        for toast in &mut self.toasts {
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

            let offset = position.offset(index, TOAST_HEIGHT);
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

                let accent_color = toast.color(theme);

                // Use Card for consistent styling (shadcn toast style)
                Card::new()
                    .variant(CardVariant::Outlined) // shadcn uses border
                    .width(TOAST_WIDTH)
                    .stroke(theme.border())
                    .corner_radius(TOAST_CORNER_RADIUS)
                    .inner_margin(TOAST_PADDING)
                    .show(ui, theme, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = TOAST_SPACING;

                            // Icon
                            let icon_size = 16.0;
                            let (rect, _) =
                                ui.allocate_exact_size(vec2(icon_size, icon_size), Sense::hover());
                            render_icon(
                                ui.painter(),
                                rect,
                                toast.variant.icon().data(),
                                accent_color,
                            );

                            // Content
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 0.0;
                                ui.set_width(TOAST_WIDTH - 100.0);

                                if let Some(title) = &toast.title {
                                    ui.strong(title);
                                }
                                ui.label(&toast.message);
                            });

                            // Close button
                            if toast.dismissible {
                                let close_response = IconButton::new(WindowIcon::Close.data())
                                    .variant(ButtonVariant::Ghost)
                                    .size(12.0)
                                    .padding(6.0)
                                    .icon_color(theme.muted_foreground())
                                    .hover_icon_color(theme.foreground())
                                    .show(ui, theme);

                                if close_response.clicked() {
                                    dismissed = true;
                                }
                            }
                        });

                        // Progress bar (shadcn style)
                        let progress = toast.progress(current_time).min(1.0);
                        if progress < 1.0 {
                            ui.add_space(TOAST_SPACING);
                            let (rect, _) = ui.allocate_exact_size(
                                vec2(ui.available_width(), PROGRESS_HEIGHT),
                                Sense::hover(),
                            );

                            // Background
                            ui.painter().rect_filled(rect, 1.0, theme.muted());

                            // Progress fill
                            let fill_width = rect.width() * progress;
                            let fill_rect = egui::Rect::from_min_size(
                                rect.min,
                                vec2(fill_width, PROGRESS_HEIGHT),
                            );

                            ui.painter().rect_filled(fill_rect, 1.0, accent_color);
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

impl ToastBuilder<'_> {
    /// Set the toast message
    #[must_use]
    pub fn message(mut self, message: impl Into<String>) -> Self {
        if let Some(toast) = &mut self.toast {
            toast.message = message.into();
        } else {
            self.toast = Some(Toast::new(message, ToastVariant::Default, 0.0));
        }
        self
    }

    /// Set the toast title
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        if let Some(toast) = &mut self.toast {
            toast.title = Some(title.into());
        }
        self
    }

    /// Set the toast variant
    #[must_use]
    pub fn variant(mut self, variant: ToastVariant) -> Self {
        if let Some(toast) = &mut self.toast {
            toast.variant = variant;
        } else {
            self.toast = Some(Toast::new("", variant, 0.0));
        }
        self
    }

    /// Make this a destructive toast
    #[must_use]
    pub fn destructive(mut self) -> Self {
        if let Some(toast) = &mut self.toast {
            toast.variant = ToastVariant::Destructive;
        } else {
            self.toast = Some(Toast::new("", ToastVariant::Destructive, 0.0));
        }
        self
    }

    /// Set custom color (overrides variant)
    #[must_use]
    pub const fn color(mut self, color: Color32) -> Self {
        if let Some(toast) = &mut self.toast {
            toast.custom_color = Some(color);
        }
        self
    }

    /// Set the display duration
    #[must_use]
    pub const fn duration(mut self, duration: std::time::Duration) -> Self {
        if let Some(toast) = &mut self.toast {
            toast.duration_secs = duration.as_secs_f32();
        }
        self
    }

    /// Set whether the toast can be manually dismissed
    #[must_use]
    pub const fn dismissible(mut self, dismissible: bool) -> Self {
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
