//! Playhead Indicator Component
//!
//! Vertical line showing current playback position in DAW timeline.
//! Draggable for scrubbing through the timeline.

use crate::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

/// Playhead indicator for DAW timeline
///
/// Shows the current playback position as a vertical line with a draggable handle.
/// Synchronized with the timeline's beat_width for accurate positioning.
///
/// # Example
///
/// ```rust,no_run
/// use armas::components::audio::Playhead;
///
/// fn ui(ui: &mut egui::Ui, theme: &armas::Theme) {
///     let mut position = 4.5; // Current beat position
///     let beat_width = 60.0;
///     let height = 400.0;
///
///     Playhead::new(beat_width, height)
///         .show(ui, &mut position, theme);
/// }
/// ```
pub struct Playhead {
    /// Unique ID for this playhead (required for multiple playheads)
    id: Option<egui::Id>,
    /// Width per beat in pixels (must match TimeRuler)
    beat_width: f32,
    /// Height of the playhead line
    height: f32,
    /// Playhead line color
    color: Option<Color32>,
    /// Line width in pixels
    line_width: f32,
    /// Show position handle at top
    show_handle: bool,
    /// Handle size (radius)
    handle_size: f32,
    /// Show glow effect around handle
    show_glow: bool,
    /// Glow intensity (0.0-1.0)
    glow_intensity: f32,
}

impl Playhead {
    /// Create a new playhead indicator
    pub fn new(beat_width: f32, height: f32) -> Self {
        Self {
            id: None,
            beat_width,
            height,
            color: None,
            line_width: 2.0,
            show_handle: true,
            handle_size: 6.0,
            show_glow: true,
            glow_intensity: 0.3,
        }
    }

    /// Set custom ID (important when using multiple playheads)
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set custom playhead color
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Set line width
    pub fn line_width(mut self, width: f32) -> Self {
        self.line_width = width;
        self
    }

    /// Set whether to show draggable handle
    pub fn show_handle(mut self, show: bool) -> Self {
        self.show_handle = show;
        self
    }

    /// Set handle size (radius)
    pub fn handle_size(mut self, size: f32) -> Self {
        self.handle_size = size;
        self
    }

    /// Set whether to show glow effect
    pub fn show_glow(mut self, show: bool) -> Self {
        self.show_glow = show;
        self
    }

    /// Set glow intensity (0.0-1.0)
    pub fn glow_intensity(mut self, intensity: f32) -> Self {
        self.glow_intensity = intensity.clamp(0.0, 1.0);
        self
    }

    /// Show the playhead indicator
    ///
    /// Renders a playhead overlay at the specified beat position within a given rect.
    /// Should be called with the timeline's rect after rendering timeline content.
    ///
    /// Returns Response with `changed()` indicating if position was modified by dragging.
    pub fn show_in_rect(
        self,
        ui: &mut Ui,
        timeline_rect: Rect,
        position: &mut f32,
        theme: &Theme,
    ) -> Response {
        // Calculate x position based on beat position
        let x = timeline_rect.min.x + (*position * self.beat_width);

        // Create interaction rect around the playhead
        let interact_width = self.handle_size * 4.0;
        let actual_height = self.height.min(timeline_rect.height());
        let interact_rect = Rect::from_center_size(
            Pos2::new(x, timeline_rect.min.y + actual_height / 2.0),
            Vec2::new(interact_width, actual_height),
        );

        let playhead_color = self.color.unwrap_or(Color32::WHITE);

        // Draw directly without Area - the caller should set up the layer
        let line_start = Pos2::new(x, timeline_rect.min.y);
        let line_end = Pos2::new(x, timeline_rect.min.y + actual_height);

        // Draw glow on entire playhead line if enabled
        if self.show_glow {
            let glow_alpha = (30.0 * self.glow_intensity) as u8; // Much more subtle (was 80)
            for i in 0..3 {
                let glow_width = self.line_width + (3 - i) as f32 * 1.5;
                let alpha = glow_alpha.saturating_sub(i * 8);
                ui.painter().line_segment(
                    [line_start, line_end],
                    Stroke::new(
                        glow_width,
                        Color32::from_rgba_unmultiplied(
                            playhead_color.r(),
                            playhead_color.g(),
                            playhead_color.b(),
                            alpha,
                        ),
                    ),
                );
            }
        }

        // Draw main playhead line
        ui.painter().line_segment(
            [line_start, line_end],
            Stroke::new(self.line_width, playhead_color),
        );

        // Draw handle at top (rounded triangle/teardrop shape)
        let mut response = if self.show_handle {
            let handle_top = Pos2::new(x, timeline_rect.min.y);
            let handle_height = self.handle_size * 2.0;
            let handle_width = self.handle_size * 1.2; // Narrower top

            // Create interactive handle
            let handle_id = ui.id().with("playhead_handle");
            let handle_response =
                ui.interact(interact_rect, handle_id, Sense::click_and_drag());

            // Handle color based on interaction
            let handle_color = if handle_response.hovered() || handle_response.dragged() {
                playhead_color.gamma_multiply(1.2)
            } else {
                playhead_color
            };

            // Draw very subtle glow effect if enabled
            if self.show_glow {
                let glow_alpha = (30.0 * self.glow_intensity) as u8;
                for i in 0..3 {
                    let glow_offset = (3 - i) as f32 * 1.5;
                    let alpha = glow_alpha.saturating_sub(i * 8);
                    let glow_color = Color32::from_rgba_unmultiplied(
                        playhead_color.r(),
                        playhead_color.g(),
                        playhead_color.b(),
                        alpha,
                    );

                    // Draw rounded triangle with glow
                    self.draw_rounded_triangle(
                        ui,
                        handle_top,
                        handle_width + glow_offset,
                        handle_height + glow_offset,
                        glow_color,
                    );
                }
            }

            // Handle shadow for depth
            self.draw_rounded_triangle(
                ui,
                handle_top + Vec2::new(0.0, 1.0),
                handle_width,
                handle_height,
                Color32::from_black_alpha(40),
            );

            // Main handle shape
            self.draw_rounded_triangle(
                ui,
                handle_top,
                handle_width,
                handle_height,
                handle_color,
            );

            // Handle border
            self.draw_rounded_triangle_outline(
                ui,
                handle_top,
                handle_width,
                handle_height,
                Stroke::new(1.5, theme.surface()),
            );

            // Inner highlight for 3D effect (small rounded triangle at top)
            self.draw_rounded_triangle(
                ui,
                handle_top + Vec2::new(0.0, 1.0),
                handle_width * 0.5,
                handle_height * 0.3,
                Color32::from_white_alpha(60),
            );

            handle_response
        } else {
            // No handle, just return a dummy response
            ui.interact(interact_rect, ui.id().with("playhead_line"), Sense::hover())
        };

        // Handle dragging
        if response.dragged() {
            let delta_x = response.drag_delta().x;
            let delta_beats = delta_x / self.beat_width;
            *position = (*position + delta_beats).max(0.0);
            response.mark_changed();
        }

        response
    }

    /// Draw a rounded triangle (teardrop shape pointing down)
    fn draw_rounded_triangle(
        &self,
        ui: &mut Ui,
        top: Pos2,
        width: f32,
        height: f32,
        color: Color32,
    ) {
        use egui::epaint::{PathShape, Shape};

        let half_width = width / 2.0;
        let bottom = Pos2::new(top.x, top.y + height);

        // Create a path with rounded corners
        let mut path = Vec::new();

        // Top rounded section (smaller arc from left to right)
        let segments = 8;
        let top_roundness = 0.15; // Reduced from 0.3 for smaller top edge
        for i in 0..=segments {
            let angle = std::f32::consts::PI * (i as f32 / segments as f32);
            let x = top.x + half_width * angle.cos();
            let y = top.y + half_width * top_roundness * (1.0 - angle.sin());
            path.push(Pos2::new(x, y));
        }

        // Right side curve to point
        let right_ctrl = Pos2::new(top.x + half_width * 0.3, top.y + height * 0.5);
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let x = (1.0 - t).powi(2) * (top.x + half_width)
                + 2.0 * (1.0 - t) * t * right_ctrl.x
                + t.powi(2) * bottom.x;
            let y = (1.0 - t).powi(2) * (top.y + half_width * top_roundness)
                + 2.0 * (1.0 - t) * t * right_ctrl.y
                + t.powi(2) * bottom.y;
            path.push(Pos2::new(x, y));
        }

        // Left side curve from point
        let left_ctrl = Pos2::new(top.x - half_width * 0.3, top.y + height * 0.5);
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let x = (1.0 - t).powi(2) * bottom.x
                + 2.0 * (1.0 - t) * t * left_ctrl.x
                + t.powi(2) * (top.x - half_width);
            let y = (1.0 - t).powi(2) * bottom.y
                + 2.0 * (1.0 - t) * t * left_ctrl.y
                + t.powi(2) * (top.y + half_width * top_roundness);
            path.push(Pos2::new(x, y));
        }

        let shape = PathShape::convex_polygon(path, color, Stroke::NONE);
        ui.painter().add(Shape::Path(shape));
    }

    /// Draw outline of rounded triangle
    fn draw_rounded_triangle_outline(
        &self,
        ui: &mut Ui,
        top: Pos2,
        width: f32,
        height: f32,
        stroke: Stroke,
    ) {
        use egui::epaint::{PathShape, Shape};

        let half_width = width / 2.0;
        let bottom = Pos2::new(top.x, top.y + height);

        let mut path = Vec::new();

        // Top rounded section (smaller arc)
        let segments = 8;
        let top_roundness = 0.15; // Match the fill method
        for i in 0..=segments {
            let angle = std::f32::consts::PI * (i as f32 / segments as f32);
            let x = top.x + half_width * angle.cos();
            let y = top.y + half_width * top_roundness * (1.0 - angle.sin());
            path.push(Pos2::new(x, y));
        }

        // Right side curve
        let right_ctrl = Pos2::new(top.x + half_width * 0.3, top.y + height * 0.5);
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let x = (1.0 - t).powi(2) * (top.x + half_width)
                + 2.0 * (1.0 - t) * t * right_ctrl.x
                + t.powi(2) * bottom.x;
            let y = (1.0 - t).powi(2) * (top.y + half_width * top_roundness)
                + 2.0 * (1.0 - t) * t * right_ctrl.y
                + t.powi(2) * bottom.y;
            path.push(Pos2::new(x, y));
        }

        // Left side curve
        let left_ctrl = Pos2::new(top.x - half_width * 0.3, top.y + height * 0.5);
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let x = (1.0 - t).powi(2) * bottom.x
                + 2.0 * (1.0 - t) * t * left_ctrl.x
                + t.powi(2) * (top.x - half_width);
            let y = (1.0 - t).powi(2) * bottom.y
                + 2.0 * (1.0 - t) * t * left_ctrl.y
                + t.powi(2) * (top.y + half_width * top_roundness);
            path.push(Pos2::new(x, y));
        }

        let shape = PathShape::line(path, stroke);
        ui.painter().add(Shape::Path(shape));
    }
}

impl Default for Playhead {
    fn default() -> Self {
        Self::new(60.0, 400.0)
    }
}
