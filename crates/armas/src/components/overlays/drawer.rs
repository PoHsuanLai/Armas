//! Drawer Component (vaul-style)
//!
//! Bottom drawer with drag handle and gesture-based dismissal,
//! styled like shadcn/ui Drawer (which uses vaul library).
//!
//! For side panels, use [`Sheet`](super::Sheet) instead.
//!
//! # Example
//!
//! ```rust,no_run
//! # use egui::Context;
//! # fn example(ctx: &Context) {
//! use armas::{Drawer, Theme};
//!
//! let theme = Theme::dark();
//! let mut open = true;
//!
//! let response = Drawer::new("my-drawer")
//!     .open(open)
//!     .title("Edit Profile")
//!     .description("Make changes to your profile here.")
//!     .show(ctx, &theme, |ui| {
//!         ui.label("Content goes here");
//!     });
//!
//! if response.closed {
//!     open = false;
//! }
//! # }
//! ```

use crate::Theme;
use egui::{vec2, Color32, Key, Pos2, Rect, Sense, Stroke, Ui};

// vaul Drawer constants
const DRAWER_MAX_HEIGHT_RATIO: f32 = 0.96; // max-h-[96%] like vaul
const DRAWER_DEFAULT_HEIGHT: f32 = 400.0; // Default height when not dragging
const _DRAWER_MIN_HEIGHT: f32 = 100.0; // Minimum before auto-close (reserved for snap points)

const HANDLE_WIDTH: f32 = 48.0; // w-12 (mx-auto)
const HANDLE_HEIGHT: f32 = 6.0; // h-1.5
const HANDLE_TOP_MARGIN: f32 = 16.0; // pt-4
const HANDLE_ROUNDING: f32 = 9999.0; // rounded-full

const HEADER_PADDING: f32 = 16.0; // p-4
const CONTENT_PADDING: f32 = 16.0; // p-4
const GAP_Y: f32 = 4.0; // gap-1 between title and description

const BACKDROP_ALPHA: f32 = 0.8; // bg-black/80
const BORDER_ROUNDING: f32 = 10.0; // rounded-t-[10px]

// Drag physics
const DRAG_CLOSE_THRESHOLD: f32 = 0.5; // Close if dragged past 50% of height
const DRAG_VELOCITY_THRESHOLD: f32 = 500.0; // Close if velocity exceeds this

/// Drawer snap points for partial open states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DrawerSnapPoint {
    /// Closed (0%)
    Closed,
    /// Partially open (custom percentage 0.0-1.0)
    Partial(f32),
    /// Fully open (100%)
    Full,
}

impl DrawerSnapPoint {
    /// Convert snap point to ratio (0.0-1.0)
    #[allow(dead_code)]
    pub fn to_ratio(&self) -> f32 {
        match self {
            DrawerSnapPoint::Closed => 0.0,
            DrawerSnapPoint::Partial(ratio) => ratio.clamp(0.0, 1.0),
            DrawerSnapPoint::Full => 1.0,
        }
    }
}

/// Drawer component (vaul-style bottom sheet)
///
/// A bottom drawer with drag handle that can be dismissed by dragging down.
/// For side panels, use [`Sheet`](super::Sheet).
pub struct Drawer {
    id: egui::Id,
    title: Option<String>,
    description: Option<String>,
    show_handle: bool,
    show_backdrop: bool,
    is_open: bool,
    snap_points: Vec<DrawerSnapPoint>,
    height: Option<f32>,
}

impl Drawer {
    /// Create a new drawer
    pub fn new(id: impl Into<egui::Id>) -> Self {
        Self {
            id: id.into(),
            title: None,
            description: None,
            show_handle: true,
            show_backdrop: true,
            is_open: false,
            snap_points: vec![DrawerSnapPoint::Full],
            height: None,
        }
    }

    /// Set the drawer open state
    pub fn open(mut self, open: bool) -> Self {
        self.is_open = open;
        self
    }

    /// Set the title (DrawerTitle equivalent)
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the description (DrawerDescription equivalent)
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Show or hide the drag handle (default: true)
    pub fn show_handle(mut self, show: bool) -> Self {
        self.show_handle = show;
        self
    }

    /// Show or hide the backdrop overlay (default: true)
    pub fn show_backdrop(mut self, show: bool) -> Self {
        self.show_backdrop = show;
        self
    }

    /// Set snap points for partial open states
    pub fn snap_points(mut self, points: Vec<DrawerSnapPoint>) -> Self {
        self.snap_points = points;
        self
    }

    /// Set a fixed height for the drawer
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Show the drawer and render content
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        theme: &Theme,
        content: impl FnOnce(&mut Ui),
    ) -> DrawerResponse {
        let mut response = DrawerResponse {
            closed: false,
            snap_point: DrawerSnapPoint::Full,
        };

        if !self.is_open {
            return response;
        }

        #[allow(deprecated)]
        let screen_rect = ctx.screen_rect();

        // Get or initialize drag state
        let drag_state_id = self.id.with("drag_state");
        let mut drag_offset: f32 = ctx.data_mut(|d| d.get_temp(drag_state_id).unwrap_or(0.0));
        let velocity_id = self.id.with("velocity");
        let mut last_drag_delta: f32 = ctx.data_mut(|d| d.get_temp(velocity_id).unwrap_or(0.0));

        // Calculate drawer dimensions
        let max_height = screen_rect.height() * DRAWER_MAX_HEIGHT_RATIO;
        let base_height = self.height.unwrap_or(DRAWER_DEFAULT_HEIGHT).min(max_height);
        let current_height = (base_height - drag_offset).max(0.0);

        // Draw backdrop
        if self.show_backdrop {
            let backdrop_alpha = (BACKDROP_ALPHA * (current_height / base_height)).max(0.0);
            let backdrop_color = Color32::from_black_alpha((255.0 * backdrop_alpha) as u8);

            egui::Area::new(self.id.with("backdrop"))
                .order(egui::Order::Middle)
                .interactable(true)
                .fixed_pos(screen_rect.min)
                .show(ctx, |ui| {
                    let backdrop_response =
                        ui.allocate_response(screen_rect.size(), Sense::click());
                    ui.painter().rect_filled(screen_rect, 0.0, backdrop_color);

                    if backdrop_response.clicked() {
                        response.closed = true;
                    }
                });
        }

        // Calculate drawer rect (from bottom)
        let drawer_rect = Rect::from_min_size(
            Pos2::new(screen_rect.left(), screen_rect.bottom() - current_height),
            vec2(screen_rect.width(), current_height),
        );

        // Draw the drawer panel
        egui::Area::new(self.id)
            .order(egui::Order::Foreground)
            .fixed_pos(drawer_rect.min)
            .show(ctx, |ui| {
                ui.set_clip_rect(drawer_rect);

                // Background with rounded top corners
                ui.painter()
                    .rect_filled(drawer_rect, BORDER_ROUNDING, theme.background());

                // Top border
                ui.painter().hline(
                    drawer_rect.x_range(),
                    drawer_rect.top(),
                    Stroke::new(1.0, theme.border()),
                );

                // Drag handle
                let mut handle_response = None;
                if self.show_handle {
                    let handle_rect = Rect::from_center_size(
                        Pos2::new(
                            drawer_rect.center().x,
                            drawer_rect.top() + HANDLE_TOP_MARGIN + HANDLE_HEIGHT / 2.0,
                        ),
                        vec2(HANDLE_WIDTH, HANDLE_HEIGHT),
                    );

                    // Larger drag area for easier grabbing
                    let drag_area = Rect::from_center_size(
                        handle_rect.center(),
                        vec2(screen_rect.width(), HANDLE_TOP_MARGIN * 2.0 + HANDLE_HEIGHT),
                    );

                    let drag_response =
                        ui.interact(drag_area, self.id.with("handle"), Sense::drag());

                    // Draw handle
                    ui.painter()
                        .rect_filled(handle_rect, HANDLE_ROUNDING, theme.muted());

                    handle_response = Some(drag_response);
                }

                // Handle dragging
                if let Some(drag_resp) = handle_response {
                    if drag_resp.dragged() {
                        let delta = drag_resp.drag_delta().y;
                        drag_offset += delta;
                        drag_offset = drag_offset.max(0.0); // Can't drag up past full height
                        last_drag_delta = delta;
                        ctx.request_repaint();
                    }

                    if drag_resp.drag_stopped() {
                        // Check if we should close based on position or velocity
                        let drag_ratio = drag_offset / base_height;
                        let velocity = last_drag_delta * 60.0; // Approximate velocity

                        if drag_ratio > DRAG_CLOSE_THRESHOLD || velocity > DRAG_VELOCITY_THRESHOLD {
                            response.closed = true;
                            drag_offset = 0.0;
                        } else {
                            // Snap back to nearest snap point
                            drag_offset = 0.0; // For now, just snap to full
                        }
                        last_drag_delta = 0.0;
                    }
                }

                // Content area starts after handle
                let content_top = if self.show_handle {
                    drawer_rect.top() + HANDLE_TOP_MARGIN * 2.0 + HANDLE_HEIGHT
                } else {
                    drawer_rect.top()
                };

                let content_rect =
                    Rect::from_min_max(Pos2::new(drawer_rect.left(), content_top), drawer_rect.max);

                let mut content_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(content_rect)
                        .layout(egui::Layout::top_down(egui::Align::Min)),
                );

                content_ui.set_clip_rect(content_rect);

                // Header section (title + description)
                if self.title.is_some() || self.description.is_some() {
                    content_ui.horizontal(|ui| {
                        ui.add_space(HEADER_PADDING);
                        ui.vertical(|ui| {
                            ui.set_width(drawer_rect.width() - HEADER_PADDING * 2.0);

                            if let Some(title) = &self.title {
                                ui.label(
                                    egui::RichText::new(title)
                                        .size(18.0)
                                        .strong()
                                        .color(theme.foreground()),
                                );
                            }

                            if let Some(desc) = &self.description {
                                ui.add_space(GAP_Y);
                                ui.label(
                                    egui::RichText::new(desc)
                                        .size(14.0)
                                        .color(theme.muted_foreground()),
                                );
                            }
                        });
                    });
                    content_ui.add_space(HEADER_PADDING);
                }

                // Main content with padding
                content_ui.horizontal(|ui| {
                    ui.add_space(CONTENT_PADDING);
                    ui.vertical(|ui| {
                        ui.set_width(drawer_rect.width() - CONTENT_PADDING * 2.0);
                        content(ui);
                    });
                    ui.add_space(CONTENT_PADDING);
                });
            });

        // Save drag state
        ctx.data_mut(|d| {
            d.insert_temp(drag_state_id, drag_offset);
            d.insert_temp(velocity_id, last_drag_delta);
        });

        // Handle ESC key
        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            response.closed = true;
        }

        // Reset drag state if closing
        if response.closed {
            ctx.data_mut(|d| {
                d.insert_temp(drag_state_id, 0.0f32);
                d.insert_temp(velocity_id, 0.0f32);
            });
        }

        response
    }
}

/// Response from showing a drawer
#[derive(Debug, Clone, Copy)]
pub struct DrawerResponse {
    /// Whether the drawer was closed this frame
    pub closed: bool,
    /// Current snap point (for partial open states)
    pub snap_point: DrawerSnapPoint,
}

impl DrawerResponse {
    /// Check if the drawer was closed
    pub fn closed(&self) -> bool {
        self.closed
    }
}
