//! Timeline Region Component
//!
//! Unified component for timeline regions with draggable handles.
//! Supports multiple region types: Selection, Loop, and Punch.

use armas_basic::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Timeline region variant type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionVariant {
    /// Selection region for editing operations (copy/cut/paste)
    Selection,
    /// Loop region for playback looping
    Loop,
    /// Punch region for recording punch-in/out
    Punch,
}

impl RegionVariant {
    /// Get default color for the variant
    #[must_use]
    pub fn default_color(&self, theme: &Theme) -> Color32 {
        match self {
            Self::Selection => Color32::from_rgb(150, 150, 150), // Neutral gray
            Self::Loop => theme.secondary(),
            Self::Punch => Color32::from_rgb(220, 50, 50), // Recording red
        }
    }

    /// Get default height for the variant
    #[must_use]
    pub const fn default_height(&self) -> f32 {
        match self {
            Self::Selection => 60.0,
            Self::Loop => 50.0,
            Self::Punch => 70.0,
        }
    }

    /// Get region opacity
    #[must_use]
    pub const fn region_opacity(&self) -> u8 {
        match self {
            Self::Selection | Self::Loop | Self::Punch => 80,
        }
    }
}

/// Timeline region marker component
///
/// Displays region markers with draggable handles and highlighted background.
/// Supports multiple variants for different use cases.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas_audio::{TimelineRegion, RegionVariant};
/// use armas_basic::ext::ArmasContextExt;
///
/// let theme = ui.ctx().armas_theme();
/// let mut start = 4.0;  // beats
/// let mut end = 16.0;   // beats
///
/// let response = TimelineRegion::new(&mut start, &mut end)
///     .variant(RegionVariant::Selection)
///     .beat_width(60.0)
///     .measures(16)
///     .snap_to_grid(true)
///     .show(ui, &theme);
///
/// if response.start_changed {
///     println!("Region start: {}", start);
/// }
/// # }
/// ```
pub struct TimelineRegion<'a> {
    start: &'a mut f32,
    end: &'a mut f32,
    variant: RegionVariant,
    beat_width: f32,
    measures: u32,
    beats_per_measure: u32,
    height: Option<f32>,
    enabled: bool,
    snap_to_grid: bool,
    grid_division: f32,
    color: Option<Color32>,
    handle_width: f32,
    show_labels: bool,
    id: Option<egui::Id>,
    clip_rect: Option<Rect>,
    vertical_range: (f32, f32),
}

/// Response from timeline region interaction
#[derive(Debug, Clone)]
pub struct TimelineRegionResponse {
    /// The egui response
    pub response: Response,
    /// Start handle was dragged
    pub start_changed: bool,
    /// End handle was dragged
    pub end_changed: bool,
    /// Region was clicked
    pub region_clicked: bool,
}

/// Helper struct for tracking interaction state
struct RegionInteraction {
    start_changed: bool,
    end_changed: bool,
    region_clicked: bool,
}

impl<'a> TimelineRegion<'a> {
    /// Create a new timeline region with default Selection variant
    pub const fn new(start: &'a mut f32, end: &'a mut f32) -> Self {
        Self {
            start,
            end,
            variant: RegionVariant::Selection,
            beat_width: 60.0,
            measures: 16,
            beats_per_measure: 4,
            height: None,
            enabled: true,
            snap_to_grid: false,
            grid_division: 1.0,
            color: None,
            handle_width: 8.0,
            show_labels: true,
            id: None,
            clip_rect: None,
            vertical_range: (0.0, 1.0),
        }
    }

    /// Set the region variant
    #[must_use]
    pub const fn variant(mut self, variant: RegionVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set unique ID for state persistence
    #[must_use]
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set pixels per beat (must match Timeline)
    #[must_use]
    pub const fn beat_width(mut self, width: f32) -> Self {
        self.beat_width = width.max(1.0);
        self
    }

    /// Set number of measures
    #[must_use]
    pub const fn measures(mut self, measures: u32) -> Self {
        self.measures = measures;
        self
    }

    /// Set beats per measure
    #[must_use]
    pub const fn beats_per_measure(mut self, beats: u32) -> Self {
        self.beats_per_measure = beats;
        self
    }

    /// Set height of the marker component
    #[must_use]
    pub const fn height(mut self, height: f32) -> Self {
        self.height = Some(height.max(20.0));
        self
    }

    /// Enable or disable the region
    #[must_use]
    pub const fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Enable snap to grid
    #[must_use]
    pub const fn snap_to_grid(mut self, snap: bool) -> Self {
        self.snap_to_grid = snap;
        self
    }

    /// Set grid division for snapping (e.g., 1.0 = whole beats, 0.25 = 16th notes)
    #[must_use]
    pub const fn grid_division(mut self, division: f32) -> Self {
        self.grid_division = division.max(0.0625);
        self
    }

    /// Set custom color for the region
    #[must_use]
    pub const fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Set handle width
    #[must_use]
    pub const fn handle_width(mut self, width: f32) -> Self {
        self.handle_width = width.max(4.0);
        self
    }

    /// Show or hide time labels on handles
    #[must_use]
    pub const fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    /// Set clip rect for rendering
    #[must_use]
    pub const fn clip_rect(mut self, clip_rect: Rect) -> Self {
        self.clip_rect = Some(clip_rect);
        self
    }

    /// Set vertical range as percentages (0.0 to 1.0)
    #[must_use]
    pub const fn vertical_range(mut self, top_percent: f32, bottom_percent: f32) -> Self {
        self.vertical_range = (top_percent.clamp(0.0, 1.0), bottom_percent.clamp(0.0, 1.0));
        self
    }

    /// Show the timeline region
    pub fn show(mut self, ui: &mut Ui, theme: &armas_basic::Theme) -> TimelineRegionResponse {
        let total_beats = self.measures * self.beats_per_measure;
        let timeline_width = total_beats as f32 * self.beat_width;

        self.load_state(ui);

        if *self.start > *self.end {
            std::mem::swap(self.start, self.end);
        }

        if !self.enabled {
            return self.create_disabled_response(ui, timeline_width);
        }

        let (rect, actual_height) = self.calculate_rect(ui, timeline_width);

        let mut interaction = RegionInteraction {
            start_changed: false,
            end_changed: false,
            region_clicked: false,
        };

        if ui.is_rect_visible(rect) {
            let region_color = self
                .color
                .unwrap_or_else(|| self.variant.default_color(theme));
            let painter = self.get_painter(ui);
            let (start_x, end_x) = self.calculate_handle_positions(&rect, timeline_width);

            interaction.region_clicked =
                self.draw_region_background(ui, &painter, &rect, region_color, start_x, end_x);

            interaction.start_changed = self.draw_and_interact_handle(
                ui,
                &painter,
                theme,
                &rect,
                actual_height,
                start_x,
                region_color,
                true,
            );

            interaction.end_changed = self.draw_and_interact_handle(
                ui,
                &painter,
                theme,
                &rect,
                actual_height,
                end_x,
                region_color,
                false,
            );

            self.draw_labels(&painter, theme, &rect, start_x, end_x);
        }

        self.save_state(ui);

        let response = ui.allocate_response(Vec2::ZERO, Sense::hover());

        TimelineRegionResponse {
            response,
            start_changed: interaction.start_changed,
            end_changed: interaction.end_changed,
            region_clicked: interaction.region_clicked,
        }
    }

    fn load_state(&mut self, ui: &mut Ui) {
        if let Some(id) = self.id {
            let state_id_start = id.with("start");
            let state_id_end = id.with("end");

            let stored_start: f32 = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id_start).unwrap_or(*self.start));
            let stored_end: f32 = ui
                .ctx()
                .data_mut(|d| d.get_temp(state_id_end).unwrap_or(*self.end));

            *self.start = stored_start;
            *self.end = stored_end;
        }
    }

    fn save_state(&self, ui: &mut Ui) {
        if let Some(id) = self.id {
            let state_id_start = id.with("start");
            let state_id_end = id.with("end");
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id_start, *self.start);
                d.insert_temp(state_id_end, *self.end);
            });
        }
    }

    fn create_disabled_response(&self, ui: &mut Ui, timeline_width: f32) -> TimelineRegionResponse {
        let (_rect, response) =
            ui.allocate_exact_size(Vec2::new(timeline_width, 0.0), Sense::hover());
        TimelineRegionResponse {
            response,
            start_changed: false,
            end_changed: false,
            region_clicked: false,
        }
    }

    fn calculate_rect(&self, ui: &mut Ui, timeline_width: f32) -> (Rect, f32) {
        let full_rect = ui.available_rect_before_wrap();
        let height = self.height.unwrap_or_else(|| self.variant.default_height());
        let (top_percent, bottom_percent) = self.vertical_range;
        let y_offset = height * top_percent;
        let actual_height = height * (bottom_percent - top_percent);

        let rect = Rect::from_min_size(
            Pos2::new(full_rect.min.x, full_rect.min.y + y_offset),
            Vec2::new(timeline_width, actual_height),
        );

        (rect, actual_height)
    }

    fn get_painter(&self, ui: &mut Ui) -> egui::Painter {
        let base_painter = ui.painter();
        self.clip_rect.map_or_else(
            || base_painter.clone(),
            |clip| base_painter.with_clip_rect(clip),
        )
    }

    fn calculate_handle_positions(&self, rect: &Rect, timeline_width: f32) -> (f32, f32) {
        let start_x = rect.min.x + (*self.start * self.beat_width).max(0.0).min(timeline_width);
        let end_x = rect.min.x + (*self.end * self.beat_width).max(0.0).min(timeline_width);
        (start_x, end_x)
    }

    fn draw_region_background(
        &self,
        ui: &mut Ui,
        painter: &egui::Painter,
        rect: &Rect,
        color: Color32,
        start_x: f32,
        end_x: f32,
    ) -> bool {
        if end_x <= start_x {
            return false;
        }

        let region_rect =
            Rect::from_min_max(Pos2::new(start_x, rect.min.y), Pos2::new(end_x, rect.max.y));

        painter.rect_filled(
            region_rect,
            0.0,
            Color32::from_rgba_unmultiplied(
                color.r(),
                color.g(),
                color.b(),
                self.variant.region_opacity(),
            ),
        );

        match self.variant {
            RegionVariant::Punch => {
                self.draw_dashed_border(painter, region_rect, color);
            }
            _ => {
                painter.rect_stroke(
                    region_rect,
                    0.0,
                    egui::Stroke::new(1.0, color),
                    egui::StrokeKind::Inside,
                );
            }
        }

        let region_id = self.id.unwrap_or_else(|| ui.id()).with("region_area");
        let region_response = ui.interact(region_rect, region_id, Sense::click());
        region_response.clicked()
    }

    fn draw_and_interact_handle(
        &mut self,
        ui: &mut Ui,
        painter: &egui::Painter,
        theme: &Theme,
        rect: &Rect,
        actual_height: f32,
        handle_x: f32,
        color: Color32,
        is_start: bool,
    ) -> bool {
        let handle_rect = Rect::from_min_size(
            Pos2::new(handle_x - self.handle_width / 2.0, rect.min.y),
            Vec2::new(self.handle_width, actual_height),
        );

        let handle_id = self.id.unwrap_or_else(|| ui.id()).with(if is_start {
            "start_handle"
        } else {
            "end_handle"
        });
        let handle_response = ui.interact(handle_rect, handle_id, Sense::click_and_drag());

        let mut changed = false;
        if handle_response.dragged() {
            if let Some(pos) = handle_response.interact_pointer_pos() {
                let new_beat = ((pos.x - rect.min.x) / self.beat_width).max(0.0);
                let snapped_beat = if self.snap_to_grid {
                    (new_beat / self.grid_division).round() * self.grid_division
                } else {
                    new_beat
                };

                if is_start {
                    *self.start = snapped_beat;
                } else {
                    *self.end = snapped_beat;
                }
                changed = true;
            }
        }

        self.draw_handle(
            painter,
            theme,
            handle_rect,
            color,
            handle_response.hovered() || handle_response.dragged(),
            is_start,
        );

        changed
    }

    fn draw_labels(
        &self,
        painter: &egui::Painter,
        theme: &Theme,
        rect: &Rect,
        start_x: f32,
        end_x: f32,
    ) {
        if !self.show_labels {
            return;
        }

        painter.text(
            Pos2::new(start_x, rect.min.y - theme.spacing.xs),
            egui::Align2::CENTER_BOTTOM,
            format!("{:.1}", self.start),
            egui::FontId::proportional(10.0),
            theme.foreground(),
        );

        painter.text(
            Pos2::new(end_x, rect.min.y - theme.spacing.xs),
            egui::Align2::CENTER_BOTTOM,
            format!("{:.1}", self.end),
            egui::FontId::proportional(10.0),
            theme.foreground(),
        );
    }

    fn draw_dashed_border(&self, painter: &egui::Painter, rect: Rect, color: Color32) {
        let dash_len = 4.0;
        let gap_len = 3.0;
        let stroke_width = 1.5;

        // Top edge
        let width = rect.width();
        let dash_cycle = dash_len + gap_len;
        let num_dashes = (width / dash_cycle).ceil() as usize;
        for i in 0..num_dashes {
            let x = (i as f32).mul_add(dash_cycle, rect.min.x);
            if x >= rect.max.x {
                break;
            }
            let end_x = (x + dash_len).min(rect.max.x);
            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(end_x, rect.min.y)],
                egui::Stroke::new(stroke_width, color),
            );
        }

        // Bottom edge
        for i in 0..num_dashes {
            let x = (i as f32).mul_add(dash_cycle, rect.min.x);
            if x >= rect.max.x {
                break;
            }
            let end_x = (x + dash_len).min(rect.max.x);
            painter.line_segment(
                [Pos2::new(x, rect.max.y), Pos2::new(end_x, rect.max.y)],
                egui::Stroke::new(stroke_width, color),
            );
        }
    }

    fn draw_handle(
        &self,
        painter: &egui::Painter,
        theme: &Theme,
        rect: Rect,
        color: Color32,
        is_hovered: bool,
        is_start: bool,
    ) {
        let bg_color = if is_hovered {
            color.gamma_multiply(1.3)
        } else {
            match self.variant {
                RegionVariant::Punch => color.gamma_multiply(0.9),
                _ => color,
            }
        };

        painter.rect_filled(rect, f32::from(theme.spacing.corner_radius_small), bg_color);

        // Handle border
        let border_width = if self.variant == RegionVariant::Punch {
            2.0
        } else {
            1.0
        };
        let border_color = match self.variant {
            RegionVariant::Punch => Color32::from_rgb(180, 40, 40),
            _ => theme.foreground().gamma_multiply(0.8),
        };

        painter.rect_stroke(
            rect,
            f32::from(theme.spacing.corner_radius_small),
            egui::Stroke::new(border_width, border_color),
            egui::StrokeKind::Outside,
        );

        // Draw variant-specific indicators
        let center = rect.center();
        match self.variant {
            RegionVariant::Selection => {
                // Brackets [ ]
                let bracket_color = theme.foreground();
                let bracket_width = 6.0;
                let bracket_height = 8.0;

                if is_start {
                    // [
                    painter.line_segment(
                        [
                            Pos2::new(
                                center.x + bracket_width / 2.0,
                                center.y - bracket_height / 2.0,
                            ),
                            Pos2::new(
                                center.x - bracket_width / 2.0,
                                center.y - bracket_height / 2.0,
                            ),
                        ],
                        egui::Stroke::new(1.5, bracket_color),
                    );
                    painter.line_segment(
                        [
                            Pos2::new(
                                center.x - bracket_width / 2.0,
                                center.y - bracket_height / 2.0,
                            ),
                            Pos2::new(
                                center.x - bracket_width / 2.0,
                                center.y + bracket_height / 2.0,
                            ),
                        ],
                        egui::Stroke::new(1.5, bracket_color),
                    );
                    painter.line_segment(
                        [
                            Pos2::new(
                                center.x - bracket_width / 2.0,
                                center.y + bracket_height / 2.0,
                            ),
                            Pos2::new(
                                center.x + bracket_width / 2.0,
                                center.y + bracket_height / 2.0,
                            ),
                        ],
                        egui::Stroke::new(1.5, bracket_color),
                    );
                } else {
                    // ]
                    painter.line_segment(
                        [
                            Pos2::new(
                                center.x - bracket_width / 2.0,
                                center.y - bracket_height / 2.0,
                            ),
                            Pos2::new(
                                center.x + bracket_width / 2.0,
                                center.y - bracket_height / 2.0,
                            ),
                        ],
                        egui::Stroke::new(1.5, bracket_color),
                    );
                    painter.line_segment(
                        [
                            Pos2::new(
                                center.x + bracket_width / 2.0,
                                center.y - bracket_height / 2.0,
                            ),
                            Pos2::new(
                                center.x + bracket_width / 2.0,
                                center.y + bracket_height / 2.0,
                            ),
                        ],
                        egui::Stroke::new(1.5, bracket_color),
                    );
                    painter.line_segment(
                        [
                            Pos2::new(
                                center.x + bracket_width / 2.0,
                                center.y + bracket_height / 2.0,
                            ),
                            Pos2::new(
                                center.x - bracket_width / 2.0,
                                center.y + bracket_height / 2.0,
                            ),
                        ],
                        egui::Stroke::new(1.5, bracket_color),
                    );
                }
            }
            RegionVariant::Loop => {
                // Arrows < >
                let arrow_size = 4.0;
                let arrow_color = theme.foreground();

                if is_start {
                    let points = [
                        Pos2::new(center.x + arrow_size / 2.0, center.y - arrow_size),
                        Pos2::new(center.x - arrow_size / 2.0, center.y),
                        Pos2::new(center.x + arrow_size / 2.0, center.y + arrow_size),
                    ];
                    painter.add(egui::Shape::convex_polygon(
                        points.to_vec(),
                        arrow_color,
                        egui::Stroke::NONE,
                    ));
                } else {
                    let points = [
                        Pos2::new(center.x - arrow_size / 2.0, center.y - arrow_size),
                        Pos2::new(center.x + arrow_size / 2.0, center.y),
                        Pos2::new(center.x - arrow_size / 2.0, center.y + arrow_size),
                    ];
                    painter.add(egui::Shape::convex_polygon(
                        points.to_vec(),
                        arrow_color,
                        egui::Stroke::NONE,
                    ));
                }
            }
            RegionVariant::Punch => {
                // Record button
                painter.circle_filled(center, 5.0, Color32::WHITE);
                painter.circle_stroke(
                    center,
                    5.0,
                    egui::Stroke::new(1.0, Color32::from_rgb(200, 200, 200)),
                );

                let label_y = center.y + 12.0;
                let text = if is_start { "IN" } else { "OUT" };
                painter.text(
                    Pos2::new(center.x, label_y),
                    egui::Align2::CENTER_CENTER,
                    text,
                    egui::FontId::proportional(7.0),
                    Color32::WHITE,
                );
            }
        }

        // Glow effect
        if is_hovered {
            let glow_layers = if self.variant == RegionVariant::Punch {
                4
            } else {
                3
            };
            let glow_offset_mult = if self.variant == RegionVariant::Punch {
                2.0
            } else {
                1.5
            };
            let glow_alpha_mult = if self.variant == RegionVariant::Punch {
                60.0
            } else {
                40.0
            };

            for i in 0..glow_layers {
                let offset = (i + 1) as f32 * glow_offset_mult;
                let alpha = ((1.0 - i as f32 / glow_layers as f32) * glow_alpha_mult) as u8;
                let glow_color =
                    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);
                painter.rect_stroke(
                    rect.expand(offset),
                    f32::from(theme.spacing.corner_radius_small),
                    egui::Stroke::new(
                        if self.variant == RegionVariant::Punch {
                            2.0
                        } else {
                            1.5
                        },
                        glow_color,
                    ),
                    egui::StrokeKind::Outside,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_region_creation() {
        let mut start = 0.0;
        let mut end = 16.0;
        let _region = TimelineRegion::new(&mut start, &mut end);
        assert_eq!(start, 0.0);
        assert_eq!(end, 16.0);
    }

    #[test]
    fn test_region_variants() {
        let mut start = 4.0;
        let mut end = 12.0;

        let selection = TimelineRegion::new(&mut start, &mut end).variant(RegionVariant::Selection);
        assert_eq!(selection.variant, RegionVariant::Selection);

        let loop_region = TimelineRegion::new(&mut start, &mut end).variant(RegionVariant::Loop);
        assert_eq!(loop_region.variant, RegionVariant::Loop);

        let punch = TimelineRegion::new(&mut start, &mut end).variant(RegionVariant::Punch);
        assert_eq!(punch.variant, RegionVariant::Punch);
    }
}
