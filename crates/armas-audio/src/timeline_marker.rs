//! Timeline Marker Component
//!
//! Unified component for timeline markers (cue points, tempo, time signature).
//! Shows a vertical line with a badge displaying marker-specific information.

use armas::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Timeline marker variant type
#[derive(Debug, Clone, PartialEq)]
pub enum MarkerVariant {
    /// Generic cue point with custom label
    Cue(String),
    /// Tempo marker with BPM value
    Tempo(f32),
    /// Time signature marker
    TimeSignature {
        /// Top number in time signature (beats per measure)
        numerator: u32,
        /// Bottom number in time signature (note value per beat)
        denominator: u32,
    },
}

impl MarkerVariant {
    /// Get default color for the variant
    pub fn default_color(&self, theme: &Theme) -> Color32 {
        match self {
            MarkerVariant::Cue(_) => theme.primary(),
            MarkerVariant::Tempo(_) => Color32::from_rgb(50, 200, 150), // Teal
            MarkerVariant::TimeSignature { .. } => Color32::from_rgb(180, 100, 220), // Purple
        }
    }

    /// Get badge text to display
    pub fn badge_text(&self) -> String {
        match self {
            MarkerVariant::Cue(label) => label.clone(),
            MarkerVariant::Tempo(bpm) => format!("{:.0} BPM", bpm),
            MarkerVariant::TimeSignature {
                numerator,
                denominator,
            } => {
                format!("{}/{}", numerator, denominator)
            }
        }
    }

    /// Get tooltip text
    pub fn tooltip_text(&self, position: f32) -> String {
        match self {
            MarkerVariant::Cue(label) => format!("{} at {:.1} beats", label, position),
            MarkerVariant::Tempo(bpm) => format!("{:.0} BPM at {:.1} beats", bpm, position),
            MarkerVariant::TimeSignature {
                numerator,
                denominator,
            } => {
                format!("{}/{} at {:.1} beats", numerator, denominator, position)
            }
        }
    }
}

/// Timeline marker component
///
/// Displays a marker at a specific beat position with variant-specific content.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas_audio::{TimelineMarker, MarkerVariant};
///
/// let mut position = 16.0; // beats
/// let mut variant = MarkerVariant::Cue("Chorus".to_string());
///
/// let response = TimelineMarker::new(&mut position, &mut variant)
///     .beat_width(60.0)
///     .measures(16)
///     .show(ui);
/// # }
/// ```
pub struct TimelineMarker<'a> {
    position: &'a mut f32,
    variant: &'a mut MarkerVariant,
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
    id: Option<egui::Id>,
    vertical_range: (f32, f32),
}

/// Response from timeline marker interaction
#[derive(Debug, Clone)]
pub struct TimelineMarkerResponse {
    /// The egui response
    pub response: Response,
    /// Marker position was changed
    pub position_changed: bool,
    /// Marker variant data was changed (BPM, time signature, etc.)
    pub variant_changed: bool,
    /// Marker was clicked
    pub clicked: bool,
    /// Marker is hovered
    pub hovered: bool,
}

impl<'a> TimelineMarker<'a> {
    /// Create a new timeline marker
    pub fn new(position: &'a mut f32, variant: &'a mut MarkerVariant) -> Self {
        Self {
            position,
            variant,
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
            show_tooltip: true,
            id: None,
            vertical_range: (0.0, 1.0),
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

    /// Set height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(20.0);
        self
    }

    /// Enable or disable
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set draggable
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

    /// Show or hide vertical line
    pub fn show_line(mut self, show: bool) -> Self {
        self.show_line = show;
        self
    }

    /// Show or hide tooltip
    pub fn show_tooltip(mut self, show: bool) -> Self {
        self.show_tooltip = show;
        self
    }

    /// Set vertical range as percentages
    pub fn vertical_range(mut self, top_percent: f32, bottom_percent: f32) -> Self {
        self.vertical_range = (top_percent.clamp(0.0, 1.0), bottom_percent.clamp(0.0, 1.0));
        self
    }

    /// Show the timeline marker
    pub fn show(mut self, ui: &mut Ui, theme: &armas::Theme) -> TimelineMarkerResponse {
        let total_beats = self.measures * self.beats_per_measure;
        let timeline_width = total_beats as f32 * self.beat_width;

        self.load_state(ui);

        if !self.enabled {
            return self.create_disabled_response(ui, timeline_width);
        }

        let (rect, actual_height) = self.calculate_rect(ui, timeline_width);
        let marker_color = self
            .color
            .unwrap_or_else(|| self.variant.default_color(&theme));

        let mut interaction = MarkerInteraction {
            position_changed: false,
            variant_changed: false,
            clicked: false,
            hovered: false,
        };

        if ui.is_rect_visible(rect) {
            let x_pos = self.calculate_x_position(&rect, timeline_width);
            let painter = ui.painter().clone();

            if self.show_line {
                self.draw_vertical_line(&painter, x_pos, &rect, actual_height, marker_color);
            }

            interaction =
                self.draw_and_interact_badge(ui, &painter, &theme, x_pos, &rect, marker_color);
        }

        self.save_state(ui);

        let response = ui.allocate_response(Vec2::ZERO, Sense::hover());

        TimelineMarkerResponse {
            response,
            position_changed: interaction.position_changed,
            variant_changed: interaction.variant_changed,
            clicked: interaction.clicked,
            hovered: interaction.hovered,
        }
    }

    fn load_state(&mut self, ui: &mut Ui) {
        if let Some(id) = self.id {
            let state_id = id.with("position");
            let stored_position: f32 = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id).unwrap_or(*self.position));
            *self.position = stored_position;
        }
    }

    fn save_state(&self, ui: &mut Ui) {
        if let Some(id) = self.id {
            let state_id = id.with("position");
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id, *self.position);
            });
        }
    }

    fn create_disabled_response(&self, ui: &mut Ui, timeline_width: f32) -> TimelineMarkerResponse {
        let (_rect, response) =
            ui.allocate_exact_size(Vec2::new(timeline_width, 0.0), Sense::hover());
        TimelineMarkerResponse {
            response,
            position_changed: false,
            variant_changed: false,
            clicked: false,
            hovered: false,
        }
    }

    fn calculate_rect(&self, ui: &mut Ui, timeline_width: f32) -> (Rect, f32) {
        let full_rect = ui.available_rect_before_wrap();
        let (top_percent, bottom_percent) = self.vertical_range;
        let y_offset = self.height * top_percent;
        let actual_height = self.height * (bottom_percent - top_percent);

        let rect = Rect::from_min_size(
            Pos2::new(full_rect.min.x, full_rect.min.y + y_offset),
            Vec2::new(timeline_width, actual_height),
        );

        (rect, actual_height)
    }

    fn calculate_x_position(&self, rect: &Rect, timeline_width: f32) -> f32 {
        rect.min.x
            + (*self.position * self.beat_width)
                .max(0.0)
                .min(timeline_width)
    }

    fn draw_vertical_line(
        &self,
        painter: &egui::Painter,
        x_pos: f32,
        rect: &Rect,
        height: f32,
        color: Color32,
    ) {
        let line_start = Pos2::new(x_pos, rect.min.y + 24.0);
        let line_end = Pos2::new(x_pos, rect.min.y + height);
        painter.line_segment([line_start, line_end], egui::Stroke::new(2.0, color));
    }

    fn draw_and_interact_badge(
        &mut self,
        ui: &mut Ui,
        painter: &egui::Painter,
        theme: &Theme,
        x_pos: f32,
        rect: &Rect,
        color: Color32,
    ) -> MarkerInteraction {
        let badge_text = self.variant.badge_text();
        let font_id = egui::FontId::proportional(11.0);
        let galley =
            painter.layout_no_wrap(badge_text.clone(), font_id.clone(), theme.foreground());

        let badge_width = galley.size().x + 12.0;
        let badge_height = 20.0;

        let badge_rect = Rect::from_min_size(
            Pos2::new(x_pos - badge_width / 2.0, rect.min.y),
            Vec2::new(badge_width, badge_height),
        );

        let badge_id = self.id.unwrap_or_else(|| ui.id()).with("badge");
        let sense = if self.draggable {
            Sense::click_and_drag()
        } else {
            Sense::click()
        };
        let badge_response = ui.interact(badge_rect, badge_id, sense);

        let mut interaction = MarkerInteraction {
            position_changed: false,
            variant_changed: false,
            clicked: badge_response.clicked(),
            hovered: badge_response.hovered(),
        };

        // Handle dragging
        if self.draggable && badge_response.dragged() {
            if let Some(pos) = badge_response.interact_pointer_pos() {
                let new_beat = ((pos.x - rect.min.x) / self.beat_width).max(0.0);
                *self.position = if self.snap_to_grid {
                    (new_beat / self.grid_division).round() * self.grid_division
                } else {
                    new_beat
                };
                interaction.position_changed = true;
            }
        }

        // Draw badge
        let bg_color = if badge_response.hovered() {
            color.gamma_multiply(1.2)
        } else {
            color
        };

        painter.rect_filled(
            badge_rect,
            theme.spacing.corner_radius_small as f32,
            bg_color,
        );

        painter.rect_stroke(
            badge_rect,
            theme.spacing.corner_radius_small as f32,
            egui::Stroke::new(1.0, theme.foreground().gamma_multiply(0.5)),
            egui::StrokeKind::Outside,
        );

        // Draw badge text
        painter.galley(
            Pos2::new(x_pos - galley.size().x / 2.0, rect.min.y + 4.0),
            galley,
            theme.foreground(),
        );

        // Draw triangle flag below badge
        self.draw_triangle_flag(painter, x_pos, rect.min.y + badge_height, color);

        // Tooltip
        if self.show_tooltip {
            badge_response.on_hover_text(self.variant.tooltip_text(*self.position));
        }

        interaction
    }

    fn draw_triangle_flag(&self, painter: &egui::Painter, x_pos: f32, y_pos: f32, color: Color32) {
        let flag_size = 6.0;
        let points = [
            Pos2::new(x_pos - flag_size / 2.0, y_pos),
            Pos2::new(x_pos + flag_size / 2.0, y_pos),
            Pos2::new(x_pos, y_pos + flag_size),
        ];
        painter.add(egui::Shape::convex_polygon(
            points.to_vec(),
            color,
            egui::Stroke::NONE,
        ));
    }
}

/// Helper struct for tracking interaction state
struct MarkerInteraction {
    position_changed: bool,
    variant_changed: bool,
    clicked: bool,
    hovered: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_variants() {
        let cue = MarkerVariant::Cue("Chorus".to_string());
        assert_eq!(cue.badge_text(), "Chorus");

        let tempo = MarkerVariant::Tempo(120.0);
        assert_eq!(tempo.badge_text(), "120 BPM");

        let time_sig = MarkerVariant::TimeSignature {
            numerator: 4,
            denominator: 4,
        };
        assert_eq!(time_sig.badge_text(), "4/4");
    }
}
