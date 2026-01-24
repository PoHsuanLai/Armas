//! Marker/Cue Point Component
//!
//! Visual markers for important positions in the timeline (verse, chorus, bridge, etc.).
//! Shows a vertical line with a labeled badge for timeline navigation.

use armas::ext::ArmasContextExt;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Marker/Cue point component
///
/// Displays a vertical line at a specific beat position with a labeled badge.
/// Useful for marking song sections, important positions, or cue points in DAW timelines.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::components::audio::Marker;
///
/// let mut position = 16.0; // beats
///
/// let response = Marker::new(&mut position, "Chorus")
///     .beat_width(60.0)
///     .measures(16)
///     .show(ui);
///
/// if response.position_changed {
///     println!("Marker moved to: {}", position);
/// }
/// # }
/// ```
pub struct Marker<'a> {
    position: &'a mut f32,
    label: String,
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
    show_tooltip: bool,
    tooltip_text: Option<String>,
    id: Option<egui::Id>,
    vertical_range: (f32, f32), // (top_percent, bottom_percent) from 0.0 to 1.0
}

/// Response from marker interaction
#[derive(Debug, Clone)]
pub struct MarkerResponse {
    /// The egui response
    pub response: Response,
    /// Marker position was changed by dragging
    pub position_changed: bool,
    /// Marker was clicked
    pub clicked: bool,
    /// Marker is hovered
    pub hovered: bool,
}

impl<'a> Marker<'a> {
    /// Create a new marker at a position with a label
    pub fn new(position: &'a mut f32, label: impl Into<String>) -> Self {
        Self {
            position,
            label: label.into(),
            beat_width: 60.0,
            measures: 16,
            beats_per_measure: 4,
            height: 80.0,
            enabled: true,
            draggable: true,
            snap_to_grid: true,
            grid_division: 1.0,
            color: None,
            show_line: true,
            show_tooltip: true,
            tooltip_text: None,
            id: None,
            vertical_range: (0.0, 1.0), // Full height by default
        }
    }

    /// Set unique ID for state persistence
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set pixels per beat (must match Timeline/TimelineTrack)
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

    /// Set grid division for snapping (e.g., 1.0 = whole beats, 0.25 = 16th notes)
    pub fn grid_division(mut self, division: f32) -> Self {
        self.grid_division = division.max(0.0625);
        self
    }

    /// Set custom color for the marker
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Show or hide the vertical line
    pub fn show_line(mut self, show: bool) -> Self {
        self.show_line = show;
        self
    }

    /// Show or hide tooltip on hover
    pub fn show_tooltip(mut self, show: bool) -> Self {
        self.show_tooltip = show;
        self
    }

    /// Set custom tooltip text (if None, shows position)
    pub fn tooltip(mut self, text: impl Into<String>) -> Self {
        self.tooltip_text = Some(text.into());
        self
    }

    /// Set vertical range as percentages (0.0 to 1.0)
    /// For example: (0.0, 0.5) = top half, (0.5, 1.0) = bottom half, (0.33, 0.66) = middle third
    pub fn vertical_range(mut self, top_percent: f32, bottom_percent: f32) -> Self {
        self.vertical_range = (top_percent.clamp(0.0, 1.0), bottom_percent.clamp(0.0, 1.0));
        self
    }

    /// Show the marker
    pub fn show(self, ui: &mut Ui) -> MarkerResponse {
        let theme = ui.ctx().armas_theme();

        // Calculate timeline width
        let total_beats = self.measures * self.beats_per_measure;
        let timeline_width = total_beats as f32 * self.beat_width;

        // Load previous state if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("marker_position");
            let stored_position: f32 = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id).unwrap_or(*self.position));
            *self.position = stored_position;
        }

        let mut position_changed = false;
        let mut clicked = false;

        if !self.enabled {
            // Return early if disabled
            let (_rect, response) = ui.allocate_exact_size(
                Vec2::new(timeline_width, 0.0),
                Sense::hover(),
            );
            return MarkerResponse {
                response,
                position_changed: false,
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

            // Marker color - default to a distinctive blue
            let marker_color = self.color.unwrap_or_else(|| {
                Color32::from_rgb(70, 140, 230) // Distinctive blue
            });

            // Calculate marker position
            let marker_x = rect.min.x + (*self.position * self.beat_width).max(0.0).min(timeline_width);

            // Draw vertical line
            if self.show_line {
                let line_start = Pos2::new(marker_x, rect.min.y + 20.0); // Start below badge
                let line_end = Pos2::new(marker_x, rect.max.y);

                painter.line_segment(
                    [line_start, line_end],
                    egui::Stroke::new(2.0, marker_color),
                );
            }

            // Draw badge at top
            let badge_height = 18.0;
            let badge_padding = 8.0;
            let label_galley = painter.layout_no_wrap(
                self.label.clone(),
                egui::FontId::proportional(11.0),
                Color32::WHITE,
            );
            let badge_width = label_galley.size().x + badge_padding * 2.0;

            let badge_rect = Rect::from_min_size(
                Pos2::new(marker_x - badge_width / 2.0, rect.min.y),
                Vec2::new(badge_width, badge_height),
            );

            // Make badge draggable if enabled
            let sense = if self.draggable && self.enabled {
                Sense::click_and_drag()
            } else {
                Sense::click()
            };

            let badge_response = ui.interact(badge_rect, ui.auto_id_with("marker_badge"), sense);

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
                marker_color.gamma_multiply(1.2)
            } else {
                marker_color
            };

            painter.rect_filled(
                badge_rect,
                theme.spacing.corner_radius_small as f32,
                badge_bg,
            );

            // Draw badge border
            painter.rect_stroke(
                badge_rect,
                theme.spacing.corner_radius_small as f32,
                egui::Stroke::new(1.0, marker_color.gamma_multiply(1.3)),
                egui::StrokeKind::Outside,
            );

            // Draw label text
            painter.galley(
                Pos2::new(
                    badge_rect.min.x + badge_padding,
                    badge_rect.min.y + (badge_height - label_galley.size().y) / 2.0,
                ),
                label_galley,
                Color32::WHITE,
            );

            // Draw tooltip on hover
            if is_hovered && self.show_tooltip {
                let tooltip_text = self.tooltip_text.unwrap_or_else(|| {
                    format!("Position: {:.2} beats\n({:.2} bars)",
                        *self.position,
                        *self.position / self.beats_per_measure as f32)
                });

                final_response.on_hover_text(tooltip_text);
            }

            // Glow effect when hovered
            if is_hovered {
                for i in 0..3 {
                    let offset = (i + 1) as f32 * 1.5;
                    let alpha = ((1.0 - i as f32 / 3.0) * 40.0) as u8;
                    let glow_color = Color32::from_rgba_unmultiplied(
                        marker_color.r(),
                        marker_color.g(),
                        marker_color.b(),
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

        // Save state to memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("marker_position");
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id, *self.position);
            });
        }

        MarkerResponse {
            response: final_response,
            position_changed,
            clicked,
            hovered: is_hovered,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_creation() {
        let mut position = 16.0;
        let _marker = Marker::new(&mut position, "Chorus");
        assert_eq!(position, 16.0);
    }

    #[test]
    fn test_marker_builder() {
        let mut position = 8.0;
        let marker = Marker::new(&mut position, "Verse")
            .draggable(false)
            .snap_to_grid(true)
            .grid_division(0.25)
            .show_line(false);

        assert!(!marker.draggable);
        assert!(marker.snap_to_grid);
        assert_eq!(marker.grid_division, 0.25);
        assert!(!marker.show_line);
    }

    #[test]
    fn test_marker_label() {
        let mut position = 0.0;
        let marker = Marker::new(&mut position, "Intro");
        assert_eq!(marker.label, "Intro");
    }
}
