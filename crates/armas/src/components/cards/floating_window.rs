//! Floating Window Container
//!
//! Draggable, resizable floating window with glassmorphic styling.
//! Built on top of egui::Window with Armas theme integration.

use crate::icon::{WindowIcon, WindowIconWidget};
use crate::theme::Theme;
use egui::{self, Align2, Color32, CornerRadius, Id, Pos2, Vec2};

/// Floating window style variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatingWindowStyle {
    /// Glassmorphic style with semi-transparent background and glow
    Glass,
    /// Standard Material Design 3 style
    Surface,
    /// Elevated style with shadow
    Elevated,
}

/// Floating window container with drag, resize, and close functionality
///
/// A draggable, resizable window with optional close button and glassmorphic styling.
/// Uses egui::Window as the base for built-in dragging and resizing.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # use armas::{Theme, FloatingWindow, FloatingWindowStyle};
/// # fn example(ui: &mut Ui, theme: &Theme, mut open: bool) {
/// let mut window = FloatingWindow::new("My Window")
///     .id("window_1")
///     .style(FloatingWindowStyle::Glass)
///     .width(400.0)
///     .height(300.0)
///     .closable(true);
///
/// let response = window.show(ui.ctx(), theme, |ui| {
///     ui.label("Window content here");
/// });
///
/// if response.closed {
///     open = false;
/// }
/// # }
/// ```
pub struct FloatingWindow {
    /// Window title
    title: String,
    /// Unique identifier
    id: Id,
    /// Visual style
    style: FloatingWindowStyle,
    /// Glass opacity (0.0 to 1.0) - only used with Glass style
    opacity: f32,
    /// Glow intensity (0.0 to 1.0) - only used with Glass style
    glow_intensity: f32,
    /// Window width
    width: Option<f32>,
    /// Window height
    height: Option<f32>,
    /// Whether window is closable
    closable: bool,
    /// Whether window is open
    open: bool,
    /// Initial position (None = centered)
    initial_pos: Option<Pos2>,
    /// Corner radius override
    corner_radius: Option<f32>,
    /// Inner padding override
    inner_margin: Option<f32>,
}

impl FloatingWindow {
    /// Create a new floating window with title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            id: Id::new("floating_window"),
            style: FloatingWindowStyle::Glass,
            opacity: 0.7,
            glow_intensity: 0.3,
            width: Some(400.0),
            height: Some(300.0),
            closable: true,
            open: true,
            initial_pos: None,
            corner_radius: None,
            inner_margin: None,
        }
    }

    /// Set the window ID (required for state persistence)
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = id.into();
        self
    }

    /// Set the window style
    pub fn style(mut self, style: FloatingWindowStyle) -> Self {
        self.style = style;
        self
    }

    /// Set glass opacity (0.0 to 1.0)
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set glow intensity (0.0 to 1.0)
    pub fn glow_intensity(mut self, intensity: f32) -> Self {
        self.glow_intensity = intensity.clamp(0.0, 1.0);
        self
    }

    /// Set window width
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set window height
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set window to be closable
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Set initial open state
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Set initial position (None = centered)
    pub fn initial_pos(mut self, pos: Pos2) -> Self {
        self.initial_pos = Some(pos);
        self
    }

    /// Set corner radius override
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = Some(radius);
        self
    }

    /// Set inner margin/padding override
    pub fn inner_margin(mut self, margin: f32) -> Self {
        self.inner_margin = Some(margin);
        self
    }

    /// Show the floating window
    pub fn show<R>(
        mut self,
        ctx: &egui::Context,
        theme: &Theme,
        content: impl FnOnce(&mut egui::Ui) -> R,
    ) -> FloatingWindowResponse<R> {
        let mut response = FloatingWindowResponse {
            closed: false,
            inner: None,
        };

        // Load state from egui memory
        let state_id = self.id.with("floating_window_state");
        self.open = ctx.data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(self.open));

        // Capture self properties before the closure
        let style = self.style;
        let opacity = self.opacity;
        let glow_intensity = self.glow_intensity;
        let corner_radius = self.corner_radius;
        let inner_margin = self.inner_margin;
        let title = self.title.clone();
        let id = self.id;
        let mut open = self.open;

        let mut window = egui::Window::new(&title)
            .id(id)
            .open(&mut open)
            .resizable(true)
            .collapsible(false)
            .title_bar(false)
            .vscroll(true);

        // Set dimensions if provided
        if let Some(width) = self.width {
            window = window.default_width(width);
        }
        if let Some(height) = self.height {
            window = window.default_height(height);
        }

        // Set initial position
        if let Some(pos) = self.initial_pos {
            window = window.fixed_pos(pos);
        } else {
            window = window.anchor(Align2::CENTER_CENTER, Vec2::ZERO);
        }

        // Show the window
        window.show(ctx, |ui| {
            // Custom thin Mac-style title bar
            Self::draw_title_bar(ui, theme, &title, &id);
            ui.separator();

            // Apply custom styling based on style variant
            match style {
                FloatingWindowStyle::Glass => {
                    Self::apply_glass_style_static(ui, theme, opacity, glow_intensity, corner_radius, inner_margin);
                }
                FloatingWindowStyle::Surface => {
                    Self::apply_surface_style_static(ui, theme, corner_radius, inner_margin);
                }
                FloatingWindowStyle::Elevated => {
                    Self::apply_elevated_style_static(ui, theme, corner_radius, inner_margin);
                }
            }

            // User content
            response.inner = Some(content(ui));
        });

        // Save open state
        ctx.data_mut(|d| {
            d.insert_temp(state_id, open);
        });

        // Check if closed
        if !open {
            response.closed = true;
        }

        response
    }

    /// Apply glassmorphic styling (static version for closure compatibility)
    fn apply_glass_style_static(
        ui: &mut egui::Ui,
        theme: &Theme,
        opacity: f32,
        glow_intensity: f32,
        corner_radius: Option<f32>,
        inner_margin: Option<f32>,
    ) {
        let glass_color = {
            let surface = theme.card();
            Color32::from_rgba_unmultiplied(
                surface.r(),
                surface.g(),
                surface.b(),
                (255.0 * opacity) as u8,
            )
        };

        let glow_color = {
            let primary = theme.primary();
            Color32::from_rgba_unmultiplied(
                primary.r(),
                primary.g(),
                primary.b(),
                (255.0 * glow_intensity * 0.3) as u8,
            )
        };

        let corner_rad = corner_radius
            .unwrap_or(theme.spacing.corner_radius as f32) as u8;

        let inner_margin_val = inner_margin.unwrap_or(theme.spacing.md);

        // Apply frame styling
        let frame = egui::Frame::new()
            .fill(glass_color)
            .corner_radius(CornerRadius::same(corner_rad))
            .stroke(egui::Stroke::new(1.0, theme.border()))
            .inner_margin(inner_margin_val);

        frame.show(ui, |_ui| {
            // Content is rendered by the caller
        });

        // Draw shimmer on top
        let rect = ui.painter().clip_rect();
        let shimmer_rect = egui::Rect::from_min_size(
            rect.min,
            egui::vec2(rect.width(), theme.spacing.xs / 2.0),
        );
        ui.painter().rect_filled(
            shimmer_rect,
            CornerRadius::same(corner_rad),
            theme.border(),
        );

        // Draw glow border
        if glow_intensity > 0.0 {
            ui.painter().rect_stroke(
                rect,
                CornerRadius::same(corner_rad),
                egui::Stroke::new(1.5, glow_color),
                egui::StrokeKind::Middle,
            );
        }
    }

    /// Apply standard surface styling (static version for closure compatibility)
    fn apply_surface_style_static(
        _ui: &mut egui::Ui,
        theme: &Theme,
        corner_radius: Option<f32>,
        inner_margin: Option<f32>,
    ) {
        let corner_rad = corner_radius
            .unwrap_or(theme.spacing.corner_radius as f32) as u8;

        let inner_margin_val = inner_margin.unwrap_or(theme.spacing.md);

        let _frame = egui::Frame::new()
            .fill(theme.card())
            .corner_radius(CornerRadius::same(corner_rad))
            .stroke(egui::Stroke::new(1.0, theme.border()))
            .inner_margin(inner_margin_val);
    }

    /// Apply elevated styling (static version for closure compatibility)
    fn apply_elevated_style_static(
        ui: &mut egui::Ui,
        theme: &Theme,
        corner_radius: Option<f32>,
        inner_margin: Option<f32>,
    ) {
        let corner_rad = corner_radius
            .unwrap_or(theme.spacing.corner_radius as f32) as u8;

        let inner_margin_val = inner_margin.unwrap_or(theme.spacing.md);

        // Draw shadow for elevation effect
        let rect = ui.painter().clip_rect();
        let shadow_color = Color32::from_rgba_unmultiplied(0, 0, 0, 30);

        // Shadow offset
        let shadow_offset = Vec2::new(0.0, 4.0);
        let shadow_rect = rect.translate(shadow_offset);

        ui.painter().rect_filled(
            shadow_rect,
            CornerRadius::same(corner_rad),
            shadow_color,
        );

        let _frame = egui::Frame::new()
            .fill(theme.card())
            .corner_radius(CornerRadius::same(corner_rad))
            .stroke(egui::Stroke::new(1.0, theme.border()))
            .inner_margin(inner_margin_val);
    }

    /// Draw thin title bar with enlarge and close buttons
    fn draw_title_bar(ui: &mut egui::Ui, theme: &Theme, title: &str, _id: &Id) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;

            // Enlarge/Maximize button (left)
            if WindowIconWidget::new(WindowIcon::IntoFullScreen)
                .size(12.0)
                .color(theme.muted_foreground())
                .show(ui)
                .clicked()
            {
                // Maximize action placeholder
            }

            // Title text (center-left)
            ui.label(
                egui::RichText::new(title)
                    .size(12.0)
                    .color(theme.foreground())
            );

            // Fill remaining space
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Close button (right)
                if WindowIconWidget::new(WindowIcon::Close)
                    .size(12.0)
                    .color(theme.muted_foreground())
                    .show(ui)
                    .clicked()
                {
                    // Close is handled by egui::Window automatically
                }
            });
        });
    }
}

impl Default for FloatingWindow {
    fn default() -> Self {
        Self::new("Window")
    }
}

/// Response from showing a floating window
pub struct FloatingWindowResponse<R> {
    /// Whether the window was closed
    pub closed: bool,
    /// The result from the content closure
    pub inner: Option<R>,
}

impl<R> FloatingWindowResponse<R> {
    /// Get the inner content result
    pub fn inner(self) -> Option<R> {
        self.inner
    }
}
