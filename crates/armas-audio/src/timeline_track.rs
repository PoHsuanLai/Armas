//! Timeline Track Component
//!
//! A horizontal track row for DAW timelines that displays audio/MIDI regions.

use armas::components::cards::{Card, CardVariant};
use armas::theme::Theme;
use egui::{Color32, Pos2, Rect, Response, Sense, StrokeKind, Ui, Vec2};

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
    #[must_use]
    pub fn empty() -> Self {
        Self { notes: Vec::new() }
    }

    /// Create MIDI data from notes
    #[must_use]
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
    #[must_use]
    pub fn empty() -> Self {
        Self { points: Vec::new() }
    }

    /// Create automation data from points
    ///
    /// For display on timeline, pass pre-interpolated points.
    /// If integrating with `audio-automation`, sample the envelope
    /// at regular intervals and pass the results here.
    #[must_use]
    pub fn from_points(points: Vec<AutomationPoint>) -> Self {
        Self { points }
    }
}

/// Region type with associated data
#[derive(Debug, Clone, Default)]
pub enum RegionType {
    /// Audio region (placeholder for waveform data)
    #[default]
    Audio,
    /// MIDI region with optional MIDI notes
    Midi(MidiData),
    /// Automation region with optional automation points
    Automation(AutomationData),
}

/// Fade curve types for region fades
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FadeCurve {
    /// Linear fade
    #[default]
    Linear,
    /// Exponential fade (fast start, slow end)
    Exponential,
    /// Logarithmic fade (slow start, fast end)
    Logarithmic,
    /// S-curve fade (smooth acceleration/deceleration)
    SCurve,
}

impl FadeCurve {
    /// Apply the fade curve to a normalized position (0.0 to 1.0)
    /// Returns gain value (0.0 to 1.0)
    #[must_use]
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::Exponential => t * t,
            Self::Logarithmic => t.sqrt(),
            Self::SCurve => {
                // Smoothstep function
                t * t * (3.0 - 2.0 * t)
            }
        }
    }
}

/// Fade settings for region
#[derive(Debug, Clone)]
pub struct FadeSettings {
    /// Fade in duration in beats
    pub fade_in: f32,
    /// Fade out duration in beats
    pub fade_out: f32,
    /// Fade in curve type
    pub fade_in_curve: FadeCurve,
    /// Fade out curve type
    pub fade_out_curve: FadeCurve,
}

impl Default for FadeSettings {
    fn default() -> Self {
        Self {
            fade_in: 0.0,
            fade_out: 0.0,
            fade_in_curve: FadeCurve::Linear,
            fade_out_curve: FadeCurve::Linear,
        }
    }
}

impl FadeSettings {
    /// Create new fade settings with specified durations
    #[must_use]
    pub fn new(fade_in: f32, fade_out: f32) -> Self {
        Self {
            fade_in,
            fade_out,
            ..Default::default()
        }
    }

    /// Set fade in curve type
    #[must_use]
    pub fn fade_in_curve(mut self, curve: FadeCurve) -> Self {
        self.fade_in_curve = curve;
        self
    }

    /// Set fade out curve type
    #[must_use]
    pub fn fade_out_curve(mut self, curve: FadeCurve) -> Self {
        self.fade_out_curve = curve;
        self
    }
}

/// Playback settings for region
#[derive(Debug, Clone)]
pub struct PlaybackSettings {
    /// Clip gain in linear scale (1.0 = 0dB, 2.0 = +6dB, 0.5 = -6dB)
    pub gain: f32,
    /// Time stretch ratio (1.0 = normal, 0.5 = half speed, 2.0 = double speed)
    pub time_stretch: f32,
    /// Pitch shift in semitones (-12 to +12)
    pub pitch_shift: i32,
    /// Play region in reverse
    pub reversed: bool,
    /// Offset into source audio/MIDI file in beats
    /// (where in the original file this region starts)
    pub source_offset: f32,
}

impl Default for PlaybackSettings {
    fn default() -> Self {
        Self {
            gain: 1.0,
            time_stretch: 1.0,
            pitch_shift: 0,
            reversed: false,
            source_offset: 0.0,
        }
    }
}

impl PlaybackSettings {
    /// Create new playback settings with specified gain
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set clip gain (linear, 1.0 = 0dB)
    #[must_use]
    pub fn gain(mut self, gain: f32) -> Self {
        self.gain = gain.max(0.0);
        self
    }

    /// Set time stretch ratio
    #[must_use]
    pub fn time_stretch(mut self, ratio: f32) -> Self {
        self.time_stretch = ratio.clamp(0.25, 4.0);
        self
    }

    /// Set pitch shift in semitones
    #[must_use]
    pub fn pitch_shift(mut self, semitones: i32) -> Self {
        self.pitch_shift = semitones.clamp(-24, 24);
        self
    }

    /// Set reversed playback
    #[must_use]
    pub fn reversed(mut self, reversed: bool) -> Self {
        self.reversed = reversed;
        self
    }

    /// Set source offset
    #[must_use]
    pub fn source_offset(mut self, offset: f32) -> Self {
        self.source_offset = offset.max(0.0);
        self
    }

    /// Convert gain to decibels
    #[must_use]
    pub fn gain_db(&self) -> f32 {
        if self.gain > 0.0 {
            20.0 * self.gain.log10()
        } else {
            -f32::INFINITY
        }
    }

    /// Set gain from decibels
    pub fn set_gain_db(&mut self, db: f32) {
        self.gain = 10.0_f32.powf(db / 20.0);
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
    /// Fade settings
    pub fades: FadeSettings,
    /// Playback settings
    pub playback: PlaybackSettings,
}

impl Region {
    /// Create a new audio region
    pub fn new(name: impl Into<String>, start: f32, duration: f32) -> Self {
        Self {
            name: name.into(),
            start,
            duration,
            region_type: RegionType::Audio,
            color: None,
            selected: false,
            muted: false,
            fades: FadeSettings::default(),
            playback: PlaybackSettings::default(),
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
            fades: FadeSettings::default(),
            playback: PlaybackSettings::default(),
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
            fades: FadeSettings::default(),
            playback: PlaybackSettings::default(),
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
            fades: FadeSettings::default(),
            playback: PlaybackSettings::default(),
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
            fades: FadeSettings::default(),
            playback: PlaybackSettings::default(),
        }
    }

    /// Set region color
    #[must_use]
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Set selected state
    #[must_use]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set muted state
    #[must_use]
    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    /// Set fade settings
    #[must_use]
    pub fn fades(mut self, fades: FadeSettings) -> Self {
        self.fades = fades;
        self
    }

    /// Set fade in duration
    #[must_use]
    pub fn fade_in(mut self, duration: f32) -> Self {
        self.fades.fade_in = duration.max(0.0);
        self
    }

    /// Set fade out duration
    #[must_use]
    pub fn fade_out(mut self, duration: f32) -> Self {
        self.fades.fade_out = duration.max(0.0);
        self
    }

    /// Set playback settings
    #[must_use]
    pub fn playback(mut self, playback: PlaybackSettings) -> Self {
        self.playback = playback;
        self
    }

    /// Set clip gain (linear, 1.0 = 0dB)
    #[must_use]
    pub fn gain(mut self, gain: f32) -> Self {
        self.playback.gain = gain.max(0.0);
        self
    }

    /// Set clip gain in decibels
    #[must_use]
    pub fn gain_db(mut self, db: f32) -> Self {
        self.playback.set_gain_db(db);
        self
    }
}

/// Region edge handle for resizing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionEdge {
    /// Left edge of region
    Start,
    /// Right edge of region
    End,
}

/// Fade handle for adjusting fade curves
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FadeHandle {
    /// Fade in handle
    In,
    /// Fade out handle
    Out,
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
    /// Region edge being dragged (`region_idx`, edge, `new_position`)
    pub region_edge_dragged: Option<(usize, RegionEdge, f32)>,
    /// Fade handle being dragged (`region_idx`, handle, `new_duration`)
    pub fade_handle_dragged: Option<(usize, FadeHandle, f32)>,
    /// Region body dragged (`region_idx`, `new_start_position`)
    pub region_dragged: Option<(usize, f32)>,
    /// Region double-clicked for name editing
    pub region_double_clicked: Option<usize>,
}

/// Timeline track component for DAW
///
/// Displays a horizontal track with regions that can be clicked and selected.
///
/// # Example
///
/// ```rust,no_run
/// use armas_audio::{TimelineTrack, Region};
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
    /// Width per beat in pixels (must match `TimeRuler`)
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
    #[must_use]
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
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set region height (None = auto-calculate with padding)
    #[must_use]
    pub fn region_height(mut self, height: f32) -> Self {
        self.region_height = Some(height);
        self
    }

    /// Set pixels per beat (must match `TimeRuler`)
    #[must_use]
    pub fn beat_width(mut self, width: f32) -> Self {
        self.beat_width = width;
        self
    }

    /// Set number of measures
    #[must_use]
    pub fn measures(mut self, measures: u32) -> Self {
        self.measures = measures;
        self
    }

    /// Set beats per measure
    #[must_use]
    pub fn beats_per_measure(mut self, beats: u32) -> Self {
        self.beats_per_measure = beats;
        self
    }

    /// Set track color (used for regions if not specified)
    #[must_use]
    pub fn track_color(mut self, color: Color32) -> Self {
        self.track_color = Some(color);
        self
    }

    /// Set background color
    #[must_use]
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
        let mut region_edge_dragged = None;
        let mut fade_handle_dragged = None;
        let mut region_dragged = None;
        let mut region_double_clicked = None;

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
            let (rect, response) = ui.allocate_exact_size(
                Vec2::new(total_width, content_height),
                Sense::click_and_drag(),
            );

            if ui.is_rect_visible(rect) {
                let painter = ui.painter();

                // Draw subtle grid lines for beats
                for beat in 0..=total_beats {
                    let x = rect.min.x + beat as f32 * self.beat_width;
                    let is_measure = beat % self.beats_per_measure == 0;

                    let line_color = if is_measure {
                        Color32::from_rgba_unmultiplied(
                            theme.border().r(),
                            theme.border().g(),
                            theme.border().b(),
                            30,
                        )
                    } else {
                        Color32::from_rgba_unmultiplied(
                            theme.border().r(),
                            theme.border().g(),
                            theme.border().b(),
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

                // Handle zones (in pixels)
                const EDGE_HANDLE_WIDTH: f32 = 8.0;
                const FADE_HANDLE_WIDTH: f32 = 12.0;

                for (i, region) in regions.iter().enumerate() {
                    let region_x = rect.min.x + region.start * self.beat_width;
                    let region_width = region.duration * self.beat_width;
                    let region_rect = Rect::from_min_size(
                        Pos2::new(region_x, rect.min.y + region_y_offset),
                        Vec2::new(region_width, region_h),
                    );

                    // Check interactions
                    if let Some(pos) = response.interact_pointer_pos() {
                        if region_rect.contains(pos) {
                            let rel_x = pos.x - region_rect.min.x;

                            // Check for double click
                            if response.double_clicked() {
                                region_double_clicked = Some(i);
                            }
                            // Check for dragging edge handles
                            else if response.dragged() {
                                let mut handled = false;

                                // Left edge resize handle
                                if rel_x <= EDGE_HANDLE_WIDTH && region.selected {
                                    let new_start = (pos.x - rect.min.x) / self.beat_width;
                                    region_edge_dragged = Some((i, RegionEdge::Start, new_start));
                                    handled = true;
                                }
                                // Right edge resize handle
                                else if rel_x >= region_width - EDGE_HANDLE_WIDTH
                                    && region.selected
                                {
                                    let new_end = (pos.x - rect.min.x) / self.beat_width;
                                    region_edge_dragged = Some((i, RegionEdge::End, new_end));
                                    handled = true;
                                }

                                // Check fade handles (only for selected regions)
                                if !handled && region.selected {
                                    // Fade in handle (if fade exists)
                                    if region.fades.fade_in > 0.0 {
                                        let fade_in_x = region.fades.fade_in * self.beat_width;
                                        if rel_x >= fade_in_x - FADE_HANDLE_WIDTH / 2.0
                                            && rel_x <= fade_in_x + FADE_HANDLE_WIDTH / 2.0
                                        {
                                            let new_fade = rel_x / self.beat_width;
                                            fade_handle_dragged =
                                                Some((i, FadeHandle::In, new_fade));
                                            handled = true;
                                        }
                                    }

                                    // Fade out handle (if fade exists)
                                    if !handled && region.fades.fade_out > 0.0 {
                                        let fade_out_x = region_width
                                            - (region.fades.fade_out * self.beat_width);
                                        if rel_x >= fade_out_x - FADE_HANDLE_WIDTH / 2.0
                                            && rel_x <= fade_out_x + FADE_HANDLE_WIDTH / 2.0
                                        {
                                            let new_fade = (region_width - rel_x) / self.beat_width;
                                            fade_handle_dragged =
                                                Some((i, FadeHandle::Out, new_fade));
                                            handled = true;
                                        }
                                    }
                                }

                                // Region body drag (move entire region) - only if no handle was grabbed
                                if !handled {
                                    let new_start =
                                        (pos.x - region_width / 2.0 - rect.min.x) / self.beat_width;
                                    region_dragged = Some((i, new_start));
                                }
                            }
                            // Single click (select)
                            else if response.clicked() {
                                region_clicked = Some(i);
                            }
                        }
                    }

                    // Draw region with handles
                    self.draw_region(painter, region_rect, region, theme);

                    // Draw interactive handles if region is selected
                    if region.selected {
                        self.draw_region_handles(
                            painter,
                            region_rect,
                            region,
                            theme,
                            self.beat_width,
                        );
                    }
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
            region_edge_dragged,
            fade_handle_dragged,
            region_dragged,
            region_double_clicked,
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
            theme.muted_foreground()
        } else {
            Color32::WHITE
        };

        let name_galley = painter.layout_no_wrap(
            region.name.clone(),
            egui::FontId::proportional(12.0),
            text_color,
        );

        let name_pos = Pos2::new(rect.min.x + 6.0, rect.min.y + 6.0);
        painter.galley(name_pos, name_galley.clone(), text_color);

        // Draw clip gain indicator right after name (if not unity)
        if (region.playback.gain - 1.0).abs() > 0.01 {
            let gain_db = region.playback.gain_db();
            let gain_text = if gain_db > 0.0 {
                format!(" +{gain_db:.1}dB")
            } else {
                format!(" {gain_db:.1}dB")
            };

            let gain_pos = Pos2::new(name_pos.x + name_galley.rect.width() + 4.0, name_pos.y);

            painter.text(
                gain_pos,
                egui::Align2::LEFT_TOP,
                gain_text,
                egui::FontId::proportional(10.0),
                theme.secondary(),
            );
        }

        // Draw visualization based on region type
        if !region.muted {
            match &region.region_type {
                RegionType::Audio => self.draw_waveform_peaks(painter, rect, region_color, &[]),
                RegionType::Midi(data) => self.draw_midi_pattern(painter, rect, region_color, data),
                RegionType::Automation(data) => {
                    self.draw_automation_curve(painter, rect, region_color, data);
                }
            }
        }
    }

    /// Draw audio waveform visualization from peaks
    fn draw_waveform_peaks(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        color: Color32,
        peaks: &[(f32, f32)],
    ) {
        let center_y = rect.center().y;
        let waveform_color = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 100);

        let content_rect = rect.shrink2(Vec2::new(6.0, 12.0));
        let available_height = content_rect.height();

        if peaks.is_empty() {
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
            let x_step = content_rect.width() / peaks.len().max(1) as f32;

            for (i, (min_peak, max_peak)) in peaks.iter().enumerate() {
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

    /// Draw interactive handles for selected regions
    fn draw_region_handles(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        region: &Region,
        theme: &Theme,
        beat_width: f32,
    ) {
        const HANDLE_WIDTH: f32 = 3.0;
        let handle_color = theme.primary();
        let handle_highlight = theme.secondary();

        // Draw edge resize handles
        // Left edge handle
        let left_handle = Rect::from_min_max(
            Pos2::new(rect.min.x, rect.min.y),
            Pos2::new(rect.min.x + HANDLE_WIDTH, rect.max.y),
        );
        painter.rect_filled(left_handle, 0.0, handle_color);

        // Right edge handle
        let right_handle = Rect::from_min_max(
            Pos2::new(rect.max.x - HANDLE_WIDTH, rect.min.y),
            Pos2::new(rect.max.x, rect.max.y),
        );
        painter.rect_filled(right_handle, 0.0, handle_color);

        // Draw fade curves if they exist
        if region.fades.fade_in > 0.0 {
            self.draw_fade_curve(
                painter,
                rect,
                region.fades.fade_in,
                region.fades.fade_in_curve,
                true,
                theme,
                beat_width,
            );

            // Draw fade in handle (hollow circle at fade end point)
            let fade_in_x = rect.min.x + region.fades.fade_in * beat_width;
            let fade_handle_y = rect.min.y + rect.height() * 0.25;
            painter.circle_stroke(
                Pos2::new(fade_in_x, fade_handle_y),
                3.0,
                egui::Stroke::new(1.5, handle_highlight),
            );
        }

        if region.fades.fade_out > 0.0 {
            self.draw_fade_curve(
                painter,
                rect,
                region.fades.fade_out,
                region.fades.fade_out_curve,
                false,
                theme,
                beat_width,
            );

            // Draw fade out handle (hollow circle at fade start point)
            let fade_out_x = rect.max.x - region.fades.fade_out * beat_width;
            let fade_handle_y = rect.min.y + rect.height() * 0.75;
            painter.circle_stroke(
                Pos2::new(fade_out_x, fade_handle_y),
                3.0,
                egui::Stroke::new(1.5, handle_highlight),
            );
        }
    }

    /// Draw fade curve overlay
    fn draw_fade_curve(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        fade_duration: f32,
        fade_curve: FadeCurve,
        is_fade_in: bool,
        theme: &Theme,
        beat_width: f32,
    ) {
        let fade_width = fade_duration * beat_width;
        if fade_width < 1.0 {
            return;
        }

        let num_points = (fade_width / 2.0).max(10.0) as usize;
        let mut points = Vec::with_capacity(num_points);

        let fade_color = theme.primary().gamma_multiply(0.4);

        for i in 0..num_points {
            let t = i as f32 / (num_points - 1) as f32;

            let x = if is_fade_in {
                rect.min.x + t * fade_width
            } else {
                rect.max.x - fade_width + t * fade_width
            };

            // Apply fade curve to get gain (0.0 to 1.0)
            let gain = if is_fade_in {
                fade_curve.apply(t)
            } else {
                fade_curve.apply(1.0 - t)
            };

            // Convert gain to y position (fade from top to bottom for fade in, or vice versa)
            let y = if is_fade_in {
                rect.max.y - (gain * rect.height())
            } else {
                rect.min.y + ((1.0 - gain) * rect.height())
            };

            points.push(Pos2::new(x, y));
        }

        // Draw the fade curve line
        for i in 0..points.len().saturating_sub(1) {
            painter.line_segment(
                [points[i], points[i + 1]],
                egui::Stroke::new(2.0, fade_color),
            );
        }

        // Draw filled area under/over the curve for visual emphasis
        if is_fade_in {
            // For fade in, fill from bottom to curve
            let mut fill_points = vec![Pos2::new(rect.min.x, rect.max.y)];
            fill_points.extend(points.iter());
            fill_points.push(Pos2::new(rect.min.x + fade_width, rect.max.y));

            painter.add(egui::Shape::convex_polygon(
                fill_points,
                Color32::from_rgba_unmultiplied(fade_color.r(), fade_color.g(), fade_color.b(), 30),
                egui::Stroke::NONE,
            ));
        } else {
            // For fade out, fill from top to curve
            let mut fill_points = vec![Pos2::new(rect.max.x - fade_width, rect.min.y)];
            fill_points.extend(points.iter());
            fill_points.push(Pos2::new(rect.max.x, rect.min.y));

            painter.add(egui::Shape::convex_polygon(
                fill_points,
                Color32::from_rgba_unmultiplied(fade_color.r(), fade_color.g(), fade_color.b(), 30),
                egui::Stroke::NONE,
            ));
        }
    }
}

impl Default for TimelineTrack {
    fn default() -> Self {
        Self::new()
    }
}
