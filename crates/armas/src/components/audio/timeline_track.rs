//! Timeline Track Component
//!
//! A horizontal track row for DAW timelines that displays audio/MIDI regions.

use crate::components::cards::{Card, CardVariant};
use crate::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, StrokeKind, Ui, Vec2};

/// Waveform data for audio regions
#[derive(Debug, Clone)]
pub struct WaveformData {
    /// Peak values (min, max pairs) for each sample window
    /// If empty, a simulated waveform will be drawn
    pub peaks: Vec<(f32, f32)>,
}

impl WaveformData {
    /// Create empty waveform data (will show simulated waveform)
    pub fn empty() -> Self {
        Self { peaks: Vec::new() }
    }

    /// Create waveform data from peak values
    pub fn from_peaks(peaks: Vec<(f32, f32)>) -> Self {
        Self { peaks }
    }
}

/// MIDI note for MIDI regions
#[derive(Debug, Clone, Copy)]
pub struct MidiNote {
    /// Note number (0-127, where 60 = middle C)
    pub note: u8,
    /// Start time in beats (relative to region start)
    pub start: f32,
    /// Duration in beats
    pub duration: f32,
    /// Velocity (0-127)
    pub velocity: u8,
}

/// MIDI data for MIDI regions
#[derive(Debug, Clone)]
pub struct MidiData {
    /// MIDI notes to display
    /// If empty, a simulated pattern will be drawn
    pub notes: Vec<MidiNote>,
}

impl MidiData {
    /// Create empty MIDI data (will show simulated pattern)
    pub fn empty() -> Self {
        Self { notes: Vec::new() }
    }

    /// Create MIDI data from notes
    pub fn from_notes(notes: Vec<MidiNote>) -> Self {
        Self { notes }
    }
}

/// Simplified automation point for timeline display
///
/// For the timeline track, we only need time and value for display.
/// The actual automation engine (audio-automation crate) handles curves.
#[derive(Debug, Clone, Copy)]
pub struct AutomationPoint {
    /// Time in beats (relative to region start)
    pub time: f32,
    /// Value (normalized 0.0 to 1.0)
    pub value: f32,
}

/// Automation data for automation regions
///
/// This is a simplified view for timeline display. For full automation editing
/// with curves (Linear, Exponential, Bezier, etc.), use the `audio-automation`
/// crate and convert to this format for display.
#[derive(Debug, Clone)]
pub struct AutomationData {
    /// Automation points (already interpolated for display)
    /// If empty, a simulated curve will be drawn
    pub points: Vec<AutomationPoint>,
}

impl AutomationData {
    /// Create empty automation data (will show simulated curve)
    pub fn empty() -> Self {
        Self { points: Vec::new() }
    }

    /// Create automation data from points
    ///
    /// For display on timeline, pass pre-interpolated points.
    /// If integrating with `audio-automation`, sample the envelope
    /// at regular intervals and pass the results here.
    pub fn from_points(points: Vec<AutomationPoint>) -> Self {
        Self { points }
    }
}

/// Region type with associated data
#[derive(Debug, Clone)]
pub enum RegionType {
    /// Audio region with optional waveform data
    Audio(WaveformData),
    /// MIDI region with optional MIDI notes
    Midi(MidiData),
    /// Automation region with optional automation points
    Automation(AutomationData),
}

impl Default for RegionType {
    fn default() -> Self {
        Self::Audio(WaveformData::empty())
    }
}

/// A region (audio clip or MIDI clip) on the timeline
#[derive(Debug, Clone)]
pub struct Region {
    /// Region name
    pub name: String,
    /// Start position in beats
    pub start: f32,
    /// Duration in beats
    pub duration: f32,
    /// Region type with data
    pub region_type: RegionType,
    /// Region color
    pub color: Option<Color32>,
    /// Whether the region is selected
    pub selected: bool,
    /// Whether the region is muted
    pub muted: bool,
}

impl Region {
    /// Create a new audio region (with simulated waveform)
    pub fn new(name: impl Into<String>, start: f32, duration: f32) -> Self {
        Self {
            name: name.into(),
            start,
            duration,
            region_type: RegionType::Audio(WaveformData::empty()),
            color: None,
            selected: false,
            muted: false,
        }
    }

    /// Create a new audio region with waveform data
    pub fn audio(name: impl Into<String>, start: f32, duration: f32, data: WaveformData) -> Self {
        Self {
            name: name.into(),
            start,
            duration,
            region_type: RegionType::Audio(data),
            color: None,
            selected: false,
            muted: false,
        }
    }

    /// Create a new MIDI region (with simulated pattern)
    pub fn midi(name: impl Into<String>, start: f32, duration: f32) -> Self {
        Self {
            name: name.into(),
            start,
            duration,
            region_type: RegionType::Midi(MidiData::empty()),
            color: None,
            selected: false,
            muted: false,
        }
    }

    /// Create a new MIDI region with MIDI data
    pub fn midi_with_data(
        name: impl Into<String>,
        start: f32,
        duration: f32,
        data: MidiData,
    ) -> Self {
        Self {
            name: name.into(),
            start,
            duration,
            region_type: RegionType::Midi(data),
            color: None,
            selected: false,
            muted: false,
        }
    }

    /// Create a new automation region (with simulated curve)
    pub fn automation(name: impl Into<String>, start: f32, duration: f32) -> Self {
        Self {
            name: name.into(),
            start,
            duration,
            region_type: RegionType::Automation(AutomationData::empty()),
            color: None,
            selected: false,
            muted: false,
        }
    }

    /// Create a new automation region with automation data
    pub fn automation_with_data(
        name: impl Into<String>,
        start: f32,
        duration: f32,
        data: AutomationData,
    ) -> Self {
        Self {
            name: name.into(),
            start,
            duration,
            region_type: RegionType::Automation(data),
            color: None,
            selected: false,
            muted: false,
        }
    }

    /// Set region color
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Set selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set muted state
    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }
}

/// Response from timeline track interaction
#[derive(Debug, Clone)]
pub struct TimelineTrackResponse {
    /// The egui response for the entire track
    pub response: Response,
    /// Index of clicked region (if any)
    pub region_clicked: Option<usize>,
    /// Empty area clicked (position in beats)
    pub empty_clicked: Option<f32>,
}

/// Timeline track component for DAW
///
/// Displays a horizontal track with regions that can be clicked and selected.
///
/// # Example
///
/// ```rust,no_run
/// use armas::components::audio::{TimelineTrack, Region};
///
/// fn ui(ui: &mut egui::Ui, theme: &armas::Theme) {
///     let mut regions = vec![
///         Region::new("Clip 1", 0.0, 4.0),
///         Region::new("Clip 2", 8.0, 4.0),
///     ];
///
///     TimelineTrack::new()
///         .height(80.0)
///         .beat_width(60.0)
///         .track_color(egui::Color32::from_rgb(100, 150, 255))
///         .show(ui, &mut regions, theme);
/// }
/// ```
pub struct TimelineTrack {
    /// Optional ID for the track
    id: Option<egui::Id>,
    /// Height of the track
    height: f32,
    /// Height of regions within the track (None = track height - padding)
    region_height: Option<f32>,
    /// Width per beat in pixels (must match TimeRuler)
    beat_width: f32,
    /// Number of measures to display
    measures: u32,
    /// Beats per measure
    beats_per_measure: u32,
    /// Track color (used for regions if not specified)
    track_color: Option<Color32>,
    /// Background color
    background_color: Option<Color32>,
}

impl TimelineTrack {
    /// Create a new timeline track
    pub fn new() -> Self {
        Self {
            id: None,
            height: 80.0,
            region_height: None,
            beat_width: 60.0,
            measures: 8,
            beats_per_measure: 4,
            track_color: None,
            background_color: None,
        }
    }

    /// Set custom ID (important when using multiple timeline tracks)
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set track height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set region height (None = auto-calculate with padding)
    pub fn region_height(mut self, height: f32) -> Self {
        self.region_height = Some(height);
        self
    }

    /// Set pixels per beat (must match TimeRuler)
    pub fn beat_width(mut self, width: f32) -> Self {
        self.beat_width = width;
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

    /// Set track color (used for regions if not specified)
    pub fn track_color(mut self, color: Color32) -> Self {
        self.track_color = Some(color);
        self
    }

    /// Set background color
    pub fn background_color(mut self, color: Color32) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Show the timeline track
    pub fn show(
        self,
        ui: &mut Ui,
        #[allow(clippy::ptr_arg)] regions: &mut Vec<Region>,
        theme: &Theme,
    ) -> TimelineTrackResponse {
        let total_beats = self.measures * self.beats_per_measure;
        let total_width = total_beats as f32 * self.beat_width;

        let mut region_clicked = None;
        let mut empty_clicked = None;

        // Don't add any padding - allocate full height to match TrackHeader
        let content_height = self.height;
        let region_h = self.region_height.unwrap_or((self.height * 0.7).max(20.0));

        let card = Card::new()
            .variant(CardVariant::Filled)
            .width(total_width)
            .height(self.height)
            .inner_margin(0.0) // No card padding
            .fill(self.background_color.unwrap_or(Color32::TRANSPARENT)); // Transparent by default to show grid

        let card_response = card.show(ui, theme, |ui| {
            // Allocate full height without any top padding

            // Allocate space for the track content
            let (rect, response) =
                ui.allocate_exact_size(Vec2::new(total_width, content_height), Sense::click());

            if ui.is_rect_visible(rect) {
                let painter = ui.painter();

                // Draw subtle grid lines for beats
                for beat in 0..=total_beats {
                    let x = rect.min.x + beat as f32 * self.beat_width;
                    let is_measure = beat % self.beats_per_measure == 0;

                    let line_color = if is_measure {
                        Color32::from_rgba_unmultiplied(
                            theme.outline().r(),
                            theme.outline().g(),
                            theme.outline().b(),
                            30,
                        )
                    } else {
                        Color32::from_rgba_unmultiplied(
                            theme.outline_variant().r(),
                            theme.outline_variant().g(),
                            theme.outline_variant().b(),
                            15,
                        )
                    };

                    painter.line_segment(
                        [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                        egui::Stroke::new(if is_measure { 1.0 } else { 0.5 }, line_color),
                    );
                }

                // Draw regions (centered vertically within the allocated rect)
                let rect_height = rect.height();
                let region_y_offset = (rect_height - region_h) / 2.0;
                for (i, region) in regions.iter().enumerate() {
                    let region_x = rect.min.x + region.start * self.beat_width;
                    let region_width = region.duration * self.beat_width;
                    let region_rect = Rect::from_min_size(
                        Pos2::new(region_x, rect.min.y + region_y_offset),
                        Vec2::new(region_width, region_h),
                    );

                    // Check if this region was clicked
                    if response.clicked() {
                        if let Some(pos) = response.interact_pointer_pos() {
                            if region_rect.contains(pos) {
                                region_clicked = Some(i);
                            }
                        }
                    }

                    // Draw region
                    self.draw_region(painter, region_rect, region, theme);
                }

                // Check for empty area click
                if response.clicked() && region_clicked.is_none() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        let beat_pos = (pos.x - rect.min.x) / self.beat_width;
                        empty_clicked = Some(beat_pos.max(0.0));
                    }
                }
            }

            response
        });

        TimelineTrackResponse {
            response: card_response.response,
            region_clicked,
            empty_clicked,
        }
    }

    /// Draw a single region
    fn draw_region(&self, painter: &egui::Painter, rect: Rect, region: &Region, theme: &Theme) {
        let region_color = region.color.or(self.track_color).unwrap_or(theme.primary());

        // Adjust color for muted state
        let display_color = if region.muted {
            Color32::from_rgba_unmultiplied(
                region_color.r() / 3,
                region_color.g() / 3,
                region_color.b() / 3,
                150,
            )
        } else {
            Color32::from_rgba_unmultiplied(
                region_color.r(),
                region_color.g(),
                region_color.b(),
                180,
            )
        };

        // Draw region background with glassmorphism
        painter.rect_filled(
            rect,
            theme.spacing.corner_radius_small as f32,
            display_color,
        );

        // Draw selection highlight
        if region.selected {
            painter.rect_stroke(
                rect,
                theme.spacing.corner_radius_small as f32,
                egui::Stroke::new(2.0, theme.primary()),
                StrokeKind::Outside,
            );
        } else {
            // Subtle border
            painter.rect_stroke(
                rect,
                theme.spacing.corner_radius_small as f32,
                egui::Stroke::new(1.0, Color32::from_black_alpha(40)),
                StrokeKind::Outside,
            );
        }

        // Draw region name
        let text_color = if region.muted {
            theme.on_surface_variant()
        } else {
            Color32::WHITE
        };

        painter.text(
            Pos2::new(rect.min.x + 6.0, rect.min.y + 6.0),
            egui::Align2::LEFT_TOP,
            &region.name,
            egui::FontId::proportional(12.0),
            text_color,
        );

        // Draw visualization based on region type
        if !region.muted {
            match &region.region_type {
                RegionType::Audio(data) => self.draw_waveform(painter, rect, region_color, data),
                RegionType::Midi(data) => self.draw_midi_pattern(painter, rect, region_color, data),
                RegionType::Automation(data) => {
                    self.draw_automation_curve(painter, rect, region_color, data)
                }
            }
        }
    }

    /// Draw audio waveform visualization
    fn draw_waveform(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        color: Color32,
        data: &WaveformData,
    ) {
        let center_y = rect.center().y;
        let waveform_color = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 100);

        let content_rect = rect.shrink2(Vec2::new(6.0, 12.0));
        let available_height = content_rect.height();

        if data.peaks.is_empty() {
            // Draw simulated waveform - use full vertical space
            let num_lines = (content_rect.width() / 4.0) as i32;
            for i in 0..num_lines {
                let x = content_rect.min.x + i as f32 * 4.0;
                // Range from 0.1 to 0.9 for more dynamic waveform (80% of space)
                let height_factor = ((i as f32 * 0.5).sin() * 0.4 + 0.5) * 0.9;
                let line_height = available_height * height_factor;

                painter.line_segment(
                    [
                        Pos2::new(x, center_y - line_height / 2.0),
                        Pos2::new(x, center_y + line_height / 2.0),
                    ],
                    egui::Stroke::new(1.5, waveform_color),
                );
            }
        } else {
            // Draw real waveform data
            let x_step = content_rect.width() / data.peaks.len().max(1) as f32;

            for (i, (min_peak, max_peak)) in data.peaks.iter().enumerate() {
                let x = content_rect.min.x + i as f32 * x_step;

                // Clamp peaks to [-1.0, 1.0] range
                let min = min_peak.clamp(-1.0, 1.0);
                let max = max_peak.clamp(-1.0, 1.0);

                // Convert to screen coordinates (inverted y-axis)
                let y_min = center_y - (max * available_height * 0.5);
                let y_max = center_y - (min * available_height * 0.5);

                painter.line_segment(
                    [Pos2::new(x, y_min), Pos2::new(x, y_max)],
                    egui::Stroke::new(1.5, waveform_color),
                );
            }
        }
    }

    /// Draw MIDI pattern visualization
    fn draw_midi_pattern(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        color: Color32,
        data: &MidiData,
    ) {
        let midi_color = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 120);

        let content_rect = rect.shrink2(Vec2::new(6.0, 12.0));

        if data.notes.is_empty() {
            // Draw simulated MIDI pattern (piano roll style blocks)
            // Spread notes across the full height of the track
            let num_note_lanes = 8; // Show 8 different note heights
            let note_height = (content_rect.height() / num_note_lanes as f32) * 0.6;
            let lane_height = content_rect.height() / num_note_lanes as f32;

            // Simulate a melodic pattern with normalized positions (0.0 to 1.0)
            // This makes the pattern fit any region duration
            let pattern = [
                (0, 0.0, 0.15),  // Lane 0, start at 0%, duration 15%
                (2, 0.15, 0.10), // Lane 2, start at 15%, duration 10%
                (4, 0.25, 0.10),
                (3, 0.35, 0.15),
                (5, 0.50, 0.10),
                (7, 0.60, 0.20),
                (4, 0.80, 0.15),
            ];

            let region_width = content_rect.width();

            for (lane, start_norm, duration_norm) in pattern {
                let y = content_rect.min.y
                    + (lane as f32 * lane_height)
                    + (lane_height - note_height) / 2.0;
                let x_start = content_rect.min.x + (start_norm * region_width);
                let block_width = duration_norm * region_width;

                painter.rect_filled(
                    Rect::from_min_size(Pos2::new(x_start, y), Vec2::new(block_width, note_height)),
                    1.0,
                    midi_color,
                );
            }
        } else {
            // Draw real MIDI notes
            // Calculate note range for vertical positioning
            let min_note = data.notes.iter().map(|n| n.note).min().unwrap_or(0);
            let max_note = data.notes.iter().map(|n| n.note).max().unwrap_or(127);
            let note_range = (max_note - min_note).max(12) as f32; // At least one octave

            for note in &data.notes {
                // Horizontal position based on start time (assuming beats)
                let x_start = content_rect.min.x
                    + (note.start / self.beats_per_measure as f32) * self.beat_width;
                let note_width = (note.duration / self.beats_per_measure as f32) * self.beat_width;

                // Vertical position based on note number (inverted: higher notes at top)
                let y_normalized = (note.note - min_note) as f32 / note_range;
                let y = content_rect.max.y - (y_normalized * content_rect.height());

                // Note height based on velocity
                let height = 3.0 + (note.velocity as f32 / 127.0) * 2.0;

                // Vary color slightly based on velocity
                let velocity_factor = note.velocity as f32 / 127.0;
                let note_color = Color32::from_rgba_unmultiplied(
                    (color.r() as f32 * velocity_factor) as u8,
                    (color.g() as f32 * velocity_factor) as u8,
                    (color.b() as f32 * velocity_factor) as u8,
                    120,
                );

                painter.rect_filled(
                    Rect::from_min_size(
                        Pos2::new(x_start, y - height / 2.0),
                        Vec2::new(note_width.max(2.0), height),
                    ),
                    1.0,
                    note_color,
                );
            }
        }
    }

    /// Draw automation curve visualization
    fn draw_automation_curve(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        color: Color32,
        data: &AutomationData,
    ) {
        let automation_color =
            Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 150);

        let content_rect = rect.shrink2(Vec2::new(6.0, 12.0));

        if data.points.is_empty() {
            // Draw simulated automation curve (sine wave) - use full vertical space
            let num_points = (content_rect.width() / 2.0) as usize;
            let mut points = Vec::with_capacity(num_points);

            for i in 0..num_points {
                let t = i as f32 / num_points as f32;
                let x = content_rect.min.x + t * content_rect.width();

                // Sine wave from 0.1 to 0.9 (80% of space)
                let value = 0.5 + 0.4 * (t * std::f32::consts::PI * 2.0).sin();
                let y = content_rect.max.y - (value * content_rect.height());

                points.push(Pos2::new(x, y));
            }

            // Draw line segments
            for i in 0..points.len() - 1 {
                painter.line_segment(
                    [points[i], points[i + 1]],
                    egui::Stroke::new(2.0, automation_color),
                );
            }

            // Draw point markers
            for point in points {
                painter.circle_filled(point, 2.0, automation_color);
            }
        } else {
            // Draw real automation data
            let mut screen_points = Vec::with_capacity(data.points.len());

            for point in &data.points {
                // Horizontal position based on time
                let x = content_rect.min.x
                    + (point.time / self.beats_per_measure as f32) * self.beat_width;

                // Vertical position based on value (0.0 at bottom, 1.0 at top)
                let y = content_rect.max.y - (point.value.clamp(0.0, 1.0) * content_rect.height());

                screen_points.push(Pos2::new(x, y));
            }

            // Draw line segments connecting points
            for i in 0..screen_points.len().saturating_sub(1) {
                painter.line_segment(
                    [screen_points[i], screen_points[i + 1]],
                    egui::Stroke::new(2.0, automation_color),
                );
            }

            // Draw point markers
            for point in &screen_points {
                painter.circle_filled(*point, 2.5, automation_color);
                painter.circle_stroke(
                    *point,
                    2.5,
                    egui::Stroke::new(1.0, Color32::from_black_alpha(60)),
                );
            }
        }
    }
}

impl Default for TimelineTrack {
    fn default() -> Self {
        Self::new()
    }
}
