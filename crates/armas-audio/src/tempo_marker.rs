//! Tempo Marker Component
//!
//! Visual markers for tempo changes with BPM display.
//! Shows a vertical line with a BPM badge for marking tempo automation points.

use armas::ext::ArmasContextExt;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Tempo marker component
///
/// Displays a tempo change marker at a specific beat position with BPM value.
/// Useful for tempo automation and marking tempo changes in DAW timelines.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::components::audio::TempoMarker;
///
/// let mut position = 16.0; // beats
/// let mut bpm = 120.0;
///
/// let response = TempoMarker::new(&mut position, &mut bpm)
///     .beat_width(60.0)
///     .measures(16)
///     .show(ui);
///
/// if response.bpm_changed {
///     println!("New BPM: {}", bpm);
/// }
/// # }
/// ```
pub struct TempoMarker<'a> {
    position: &'a mut f32,
    bpm: &'a mut f32,
    beat_width: f32,
    measures: u32,
    beats_per_measure: u32,
    height: f32,
    enabled: bool,
    draggable: bool,
    editable_bpm: bool,
    snap_to_grid: bool,
    grid_division: f32,
    color: Option<Color32>,
    show_line: bool,
    id: Option<egui::Id>,
    vertical_range: (f32, f32), // (top_percent, bottom_percent) from 0.0 to 1.0
}

/// Response from tempo marker interaction
#[derive(Debug, Clone)]
pub struct TempoMarkerResponse {
    /// The egui response
    pub response: Response,
    /// Marker position was changed by dragging
    pub position_changed: bool,
    /// BPM value was changed
    pub bpm_changed: bool,
    /// Marker was clicked
    pub clicked: bool,
    /// Marker is hovered
    pub hovered: bool,
}

impl<'a> TempoMarker<'a> {
    /// Create a new tempo marker
    pub fn new(position: &'a mut f32, bpm: &'a mut f32) -> Self {
        Self {
            position,
            bpm,
            beat_width: 60.0,
            measures: 16,
            beats_per_measure: 4,
            height: 70.0,
            enabled: true,
            draggable: true,
            editable_bpm: true,
            snap_to_grid: true,
            grid_division: 1.0,
            color: None,
            show_line: true,
            id: None,
            vertical_range: (0.0, 1.0), // Full height by default
        }
    }

    /// Set unique ID for state persistence
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set pixels per beat
    pub fn beat_width(mut self, width: f32) -> Self {
        self.beat_width = width.max(1.0);
        self
    }

    /// Set number of measures
    pub fn measures(mut self, measures: u32) -> Self {
        self.measures = measures;
        self
    }

    /// Set beats per measure
    pub fn beats_per_measure(mut self, beats: u32) -> Self {
        self.beats_per_measure = beats;
        self
    }

    /// Set height of the marker component
    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(20.0);
        self
    }

    /// Enable or disable the marker
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set whether the marker position can be dragged
    pub fn draggable(mut self, draggable: bool) -> Self {
        self.draggable = draggable;
        self
    }

    /// Set whether BPM can be edited by clicking
    pub fn editable_bpm(mut self, editable: bool) -> Self {
        self.editable_bpm = editable;
        self
    }

    /// Enable snap to grid
    pub fn snap_to_grid(mut self, snap: bool) -> Self {
        self.snap_to_grid = snap;
        self
    }

    /// Set grid division for snapping
    pub fn grid_division(mut self, division: f32) -> Self {
        self.grid_division = division.max(0.0625);
        self
    }

    /// Set custom color
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Show or hide the vertical line
    pub fn show_line(mut self, show: bool) -> Self {
        self.show_line = show;
        self
    }

    /// Set vertical range as percentages (0.0 to 1.0)
    /// For example: (0.0, 0.5) = top half, (0.5, 1.0) = bottom half, (0.33, 0.66) = middle third
    pub fn vertical_range(mut self, top_percent: f32, bottom_percent: f32) -> Self {
        self.vertical_range = (top_percent.clamp(0.0, 1.0), bottom_percent.clamp(0.0, 1.0));
        self
    }

    /// Show the tempo marker
    pub fn show(self, ui: &mut Ui) -> TempoMarkerResponse {
        let theme = ui.ctx().armas_theme();

        let total_beats = self.measures * self.beats_per_measure;
        let timeline_width = total_beats as f32 * self.beat_width;

        // Load state
        if let Some(id) = self.id {
            let pos_id = id.with("tempo_position");
            let bpm_id = id.with("tempo_bpm");
            let stored_pos: f32 = ui.ctx().data_mut(|d| d.get_temp(pos_id).unwrap_or(*self.position));
            let stored_bpm: f32 = ui.ctx().data_mut(|d| d.get_temp(bpm_id).unwrap_or(*self.bpm));
            *self.position = stored_pos;
            *self.bpm = stored_bpm;
        }

        let mut position_changed = false;
        let bpm_changed = false;
        let mut clicked = false;

        if !self.enabled {
            let (_rect, response) = ui.allocate_exact_size(Vec2::new(timeline_width, 0.0), Sense::hover());
            return TempoMarkerResponse {
                response,
                position_changed: false,
                bpm_changed: false,
                clicked: false,
                hovered: false,
            };
        }

        // Apply vertical range to position within the available height
        let full_rect = ui.available_rect_before_wrap();
        let full_height = self.height;

        let (top_percent, bottom_percent) = self.vertical_range;
        let y_offset = full_height * top_percent;
        let actual_height = full_height * (bottom_percent - top_percent);

        let rect = Rect::from_min_size(
            Pos2::new(full_rect.min.x, full_rect.min.y + y_offset),
            Vec2::new(timeline_width, actual_height)
        );

        let mut final_response = ui.allocate_response(Vec2::ZERO, Sense::hover());
        let mut is_hovered = false;

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Tempo markers use green/teal color
            let tempo_color = self.color.unwrap_or_else(|| {
                Color32::from_rgb(50, 200, 150)
            });

            let marker_x = rect.min.x + (*self.position * self.beat_width).max(0.0).min(timeline_width);

            // Draw vertical line
            if self.show_line {
                let line_start = Pos2::new(marker_x, rect.min.y + 22.0);
                let line_end = Pos2::new(marker_x, rect.max.y);
                painter.line_segment([line_start, line_end], egui::Stroke::new(2.0, tempo_color));
            }

            // Draw BPM badge
            let bpm_text = format!("{:.0} BPM", self.bpm);
            let badge_padding = 6.0;
            let bpm_galley = painter.layout_no_wrap(
                bpm_text,
                egui::FontId::proportional(10.0),
                Color32::WHITE,
            );
            let badge_width = bpm_galley.size().x + badge_padding * 2.0;
            let badge_height = 20.0;

            let badge_rect = Rect::from_min_size(
                Pos2::new(marker_x - badge_width / 2.0, rect.min.y),
                Vec2::new(badge_width, badge_height),
            );

            let sense = if self.draggable && self.enabled {
                Sense::click_and_drag()
            } else {
                Sense::click()
            };

            let badge_response = ui.interact(badge_rect, ui.auto_id_with("tempo_badge"), sense);
            is_hovered = badge_response.hovered();

            // Handle dragging
            if badge_response.dragged() && self.draggable {
                if let Some(pos) = badge_response.interact_pointer_pos() {
                    let new_beat = ((pos.x - rect.min.x) / self.beat_width).max(0.0);
                    *self.position = if self.snap_to_grid {
                        (new_beat / self.grid_division).round() * self.grid_division
                    } else {
                        new_beat
                    };
                    position_changed = true;
                }
            }

            if badge_response.clicked() {
                clicked = true;
            }

            // Draw badge
            let badge_bg = if is_hovered {
                tempo_color.gamma_multiply(1.2)
            } else {
                tempo_color
            };

            painter.rect_filled(badge_rect, theme.spacing.corner_radius_small as f32, badge_bg);
            painter.rect_stroke(
                badge_rect,
                theme.spacing.corner_radius_small as f32,
                egui::Stroke::new(1.0, tempo_color.gamma_multiply(1.3)),
                egui::StrokeKind::Outside,
            );

            // Draw BPM text
            painter.galley(
                Pos2::new(
                    badge_rect.min.x + badge_padding,
                    badge_rect.min.y + (badge_height - bpm_galley.size().y) / 2.0,
                ),
                bpm_galley,
                Color32::WHITE,
            );

            // Draw small triangle flag indicator
            let flag_size = 6.0;
            let flag_top = Pos2::new(marker_x, rect.min.y + 22.0);
            let flag_bottom_left = Pos2::new(marker_x, flag_top.y + flag_size);
            let flag_bottom_right = Pos2::new(marker_x + flag_size, flag_top.y + flag_size / 2.0);

            painter.add(egui::Shape::convex_polygon(
                vec![flag_top, flag_bottom_left, flag_bottom_right],
                tempo_color,
                egui::Stroke::NONE,
            ));

            // Tooltip
            if is_hovered {
                final_response.on_hover_text(format!(
                    "Tempo: {:.1} BPM\nPosition: {:.2} beats ({:.2} bars)",
                    self.bpm,
                    *self.position,
                    *self.position / self.beats_per_measure as f32
                ));
            }

            // Glow effect
            if is_hovered {
                for i in 0..3 {
                    let offset = (i + 1) as f32 * 1.5;
                    let alpha = ((1.0 - i as f32 / 3.0) * 40.0) as u8;
                    let glow_color = Color32::from_rgba_unmultiplied(
                        tempo_color.r(),
                        tempo_color.g(),
                        tempo_color.b(),
                        alpha,
                    );
                    painter.rect_stroke(
                        badge_rect.expand(offset),
                        theme.spacing.corner_radius_small as f32,
                        egui::Stroke::new(1.5, glow_color),
                        egui::StrokeKind::Outside,
                    );
                }
            }

            final_response = badge_response;
        }

        // Save state
        if let Some(id) = self.id {
            let pos_id = id.with("tempo_position");
            let bpm_id = id.with("tempo_bpm");
            ui.ctx().data_mut(|d| {
                d.insert_temp(pos_id, *self.position);
                d.insert_temp(bpm_id, *self.bpm);
            });
        }

        TempoMarkerResponse {
            response: final_response,
            position_changed,
            bpm_changed,
            clicked,
            hovered: is_hovered,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tempo_marker_creation() {
        let mut position = 16.0;
        let mut bpm = 120.0;
        let _marker = TempoMarker::new(&mut position, &mut bpm);
        assert_eq!(position, 16.0);
        assert_eq!(bpm, 120.0);
    }
}
