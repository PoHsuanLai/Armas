//! Time Signature Marker Component
//!
//! Visual markers for time signature changes.
//! Shows a vertical line with a time signature badge (e.g., "4/4", "3/4", "7/8").

use armas::ext::ArmasContextExt;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Time signature marker component
///
/// Displays a time signature change marker at a specific beat position.
/// Useful for marking meter changes in DAW timelines.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::components::audio::TimeSignatureMarker;
///
/// let mut position = 32.0; // beats
/// let mut numerator = 3;
/// let mut denominator = 4;
///
/// TimeSignatureMarker::new(&mut position, &mut numerator, &mut denominator)
///     .beat_width(60.0)
///     .measures(16)
///     .show(ui);
/// # }
/// ```
pub struct TimeSignatureMarker<'a> {
    position: &'a mut f32,
    numerator: &'a mut u32,
    denominator: &'a mut u32,
    beat_width: f32,
    measures: u32,
    beats_per_measure: u32,
    height: f32,
    enabled: bool,
    draggable: bool,
    snap_to_grid: bool,
    grid_division: f32,
    color: Option<Color32>,
    show_line: bool,
    id: Option<egui::Id>,
    vertical_range: (f32, f32), // (top_percent, bottom_percent) from 0.0 to 1.0
}

/// Response from time signature marker interaction
#[derive(Debug, Clone)]
pub struct TimeSignatureMarkerResponse {
    /// The egui response
    pub response: Response,
    /// Marker position was changed
    pub position_changed: bool,
    /// Time signature was changed
    pub time_sig_changed: bool,
    /// Marker was clicked
    pub clicked: bool,
    /// Marker is hovered
    pub hovered: bool,
}

impl<'a> TimeSignatureMarker<'a> {
    /// Create a new time signature marker
    pub fn new(position: &'a mut f32, numerator: &'a mut u32, denominator: &'a mut u32) -> Self {
        Self {
            position,
            numerator,
            denominator,
            beat_width: 60.0,
            measures: 16,
            beats_per_measure: 4,
            height: 70.0,
            enabled: true,
            draggable: true,
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

    /// Set whether the marker can be dragged
    pub fn draggable(mut self, draggable: bool) -> Self {
        self.draggable = draggable;
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

    /// Show the time signature marker
    pub fn show(self, ui: &mut Ui) -> TimeSignatureMarkerResponse {
        let theme = ui.ctx().armas_theme();

        let total_beats = self.measures * self.beats_per_measure;
        let timeline_width = total_beats as f32 * self.beat_width;

        // Load state
        if let Some(id) = self.id {
            let pos_id = id.with("timesig_position");
            let num_id = id.with("timesig_num");
            let den_id = id.with("timesig_den");

            let stored_pos: f32 = ui.ctx().data_mut(|d| d.get_temp(pos_id).unwrap_or(*self.position));
            let stored_num: u32 = ui.ctx().data_mut(|d| d.get_temp(num_id).unwrap_or(*self.numerator));
            let stored_den: u32 = ui.ctx().data_mut(|d| d.get_temp(den_id).unwrap_or(*self.denominator));

            *self.position = stored_pos;
            *self.numerator = stored_num;
            *self.denominator = stored_den;
        }

        let mut position_changed = false;
        let time_sig_changed = false;
        let mut clicked = false;

        if !self.enabled {
            let (_rect, response) = ui.allocate_exact_size(Vec2::new(timeline_width, 0.0), Sense::hover());
            return TimeSignatureMarkerResponse {
                response,
                position_changed: false,
                time_sig_changed: false,
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

            // Time signature markers use purple color
            let timesig_color = self.color.unwrap_or_else(|| {
                Color32::from_rgb(180, 100, 220)
            });

            let marker_x = rect.min.x + (*self.position * self.beat_width).max(0.0).min(timeline_width);

            // Draw vertical line
            if self.show_line {
                let line_start = Pos2::new(marker_x, rect.min.y + 24.0);
                let line_end = Pos2::new(marker_x, rect.max.y);
                painter.line_segment([line_start, line_end], egui::Stroke::new(2.0, timesig_color));
            }

            // Draw time signature badge (vertically stacked)
            let badge_width = 28.0;
            let badge_height = 24.0;
            let badge_rect = Rect::from_min_size(
                Pos2::new(marker_x - badge_width / 2.0, rect.min.y),
                Vec2::new(badge_width, badge_height),
            );

            let sense = if self.draggable && self.enabled {
                Sense::click_and_drag()
            } else {
                Sense::click()
            };

            let badge_response = ui.interact(badge_rect, ui.auto_id_with("timesig_badge"), sense);
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

            // Draw badge background
            let badge_bg = if is_hovered {
                timesig_color.gamma_multiply(1.2)
            } else {
                timesig_color
            };

            painter.rect_filled(badge_rect, theme.spacing.corner_radius_small as f32, badge_bg);
            painter.rect_stroke(
                badge_rect,
                theme.spacing.corner_radius_small as f32,
                egui::Stroke::new(1.0, timesig_color.gamma_multiply(1.3)),
                egui::StrokeKind::Outside,
            );

            // Draw time signature text (stacked)
            let center_x = badge_rect.center().x;
            let num_text = format!("{}", self.numerator);
            let den_text = format!("{}", self.denominator);

            // Numerator (top)
            painter.text(
                Pos2::new(center_x, badge_rect.min.y + 6.0),
                egui::Align2::CENTER_TOP,
                num_text,
                egui::FontId::proportional(10.0),
                Color32::WHITE,
            );

            // Divider line
            painter.line_segment(
                [
                    Pos2::new(badge_rect.min.x + 4.0, badge_rect.center().y),
                    Pos2::new(badge_rect.max.x - 4.0, badge_rect.center().y),
                ],
                egui::Stroke::new(1.0, Color32::WHITE),
            );

            // Denominator (bottom)
            painter.text(
                Pos2::new(center_x, badge_rect.max.y - 6.0),
                egui::Align2::CENTER_BOTTOM,
                den_text,
                egui::FontId::proportional(10.0),
                Color32::WHITE,
            );

            // Tooltip
            if is_hovered {
                final_response.on_hover_text(format!(
                    "Time Signature: {}/{}\nPosition: {:.2} beats ({:.2} bars)",
                    self.numerator,
                    self.denominator,
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
                        timesig_color.r(),
                        timesig_color.g(),
                        timesig_color.b(),
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
            let pos_id = id.with("timesig_position");
            let num_id = id.with("timesig_num");
            let den_id = id.with("timesig_den");

            ui.ctx().data_mut(|d| {
                d.insert_temp(pos_id, *self.position);
                d.insert_temp(num_id, *self.numerator);
                d.insert_temp(den_id, *self.denominator);
            });
        }

        TimeSignatureMarkerResponse {
            response: final_response,
            position_changed,
            time_sig_changed,
            clicked,
            hovered: is_hovered,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timesig_marker_creation() {
        let mut position = 32.0;
        let mut num = 3;
        let mut den = 4;
        let _marker = TimeSignatureMarker::new(&mut position, &mut num, &mut den);
        assert_eq!(num, 3);
        assert_eq!(den, 4);
    }
}
