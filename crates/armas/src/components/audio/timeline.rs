//! Timeline Component
//!
//! Complete scrollable timeline view combining ruler, playhead, track headers, and tracks.

use crate::components::audio::{
    LoopRegionMarker, Marker, Playhead, PunchMarker, Region, SelectionRange, SnapGrid,
    TimeRuler, TimelineTrack, TrackControls, TrackHeader,
};
use crate::theme::Theme;
use egui::{pos2, vec2, Color32, Rect, Response, Sense, Ui, Vec2};

/// Data for a cue point marker
#[derive(Debug, Clone)]
pub struct MarkerData {
    /// Position in beats
    pub position: f32,
    /// Marker content (label, tempo, time signature, etc.)
    pub content: String,
    /// Optional custom color
    pub color: Option<Color32>,
}

impl MarkerData {
    /// Create a new marker with label
    pub fn new(position: f32, label: impl Into<String>) -> Self {
        Self {
            position,
            content: label.into(),
            color: None,
        }
    }

    /// Create a tempo marker
    pub fn tempo(position: f32, bpm: f32) -> Self {
        Self {
            position,
            content: format!("{:.0} BPM", bpm),
            color: Some(Color32::from_rgb(0, 180, 180)), // Teal
        }
    }

    /// Create a time signature marker
    pub fn time_signature(position: f32, numerator: u32, denominator: u32) -> Self {
        Self {
            position,
            content: format!("{}/{}", numerator, denominator),
            color: Some(Color32::from_rgb(150, 100, 200)), // Purple
        }
    }

    /// Set custom color
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
}


/// Data for a loop region
#[derive(Debug, Clone)]
pub struct LoopRegionData {
    /// Start position in beats
    pub start: f32,
    /// End position in beats
    pub end: f32,
}

impl LoopRegionData {
    /// Create a new loop region
    pub fn new(start: f32, end: f32) -> Self {
        Self { start, end }
    }
}

/// Data for a selection range
#[derive(Debug, Clone)]
pub struct SelectionRangeData {
    /// Start position in beats
    pub start: f32,
    /// End position in beats
    pub end: f32,
}

impl SelectionRangeData {
    /// Create a new selection range
    pub fn new(start: f32, end: f32) -> Self {
        Self { start, end }
    }
}

/// Data for a punch in/out region
#[derive(Debug, Clone)]
pub struct PunchRegionData {
    /// Punch in position in beats
    pub punch_in: f32,
    /// Punch out position in beats
    pub punch_out: f32,
}

impl PunchRegionData {
    /// Create a new punch region
    pub fn new(punch_in: f32, punch_out: f32) -> Self {
        Self { punch_in, punch_out }
    }
}

/// Track data for DAW timeline
#[derive(Debug, Clone)]
pub struct Track {
    /// Track name
    pub name: String,
    /// Track controls (mute, solo, arm)
    pub controls: TrackControls,
    /// Track color
    pub color: Color32,
    /// Regions on this track
    pub regions: Vec<Region>,
    /// Is this a folder track?
    pub is_folder: bool,
    /// Is this folder collapsed? (only applies if is_folder is true)
    pub collapsed: bool,
    /// Child tracks (for folder tracks)
    pub children: Vec<Track>,
    /// Whether this track is selected
    pub selected: bool,
}

impl Track {
    /// Create a new track
    pub fn new(name: impl Into<String>, color: Color32) -> Self {
        Self {
            name: name.into(),
            controls: TrackControls::default(),
            color,
            regions: Vec::new(),
            is_folder: false,
            collapsed: false,
            children: Vec::new(),
            selected: false,
        }
    }

    /// Create a new folder track
    pub fn new_folder(name: impl Into<String>, color: Color32) -> Self {
        Self {
            name: name.into(),
            controls: TrackControls::default(),
            color,
            regions: Vec::new(),
            is_folder: true,
            collapsed: false,
            children: Vec::new(),
            selected: false,
        }
    }

    /// Add a region to the track
    pub fn region(mut self, region: Region) -> Self {
        self.regions.push(region);
        self
    }

    /// Add multiple regions to the track
    pub fn regions(mut self, regions: Vec<Region>) -> Self {
        self.regions = regions;
        self
    }

    /// Add a child track (for folder tracks)
    pub fn child(mut self, child: Track) -> Self {
        self.children.push(child);
        self
    }

    /// Add multiple child tracks (for folder tracks)
    pub fn children(mut self, children: Vec<Track>) -> Self {
        self.children = children;
        self
    }

    /// Set collapsed state
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Set selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

/// Response from timeline interaction
#[derive(Debug, Clone)]
pub struct TimelineResponse {
    /// The egui response for the entire timeline
    pub response: Response,
    /// Track index that was clicked (if any)
    pub track_clicked: Option<usize>,
    /// Track mute button clicked (track_idx)
    pub track_mute_clicked: Option<usize>,
    /// Track solo button clicked (track_idx)
    pub track_solo_clicked: Option<usize>,
    /// Track record arm button clicked (track_idx)
    pub track_arm_clicked: Option<usize>,
    /// Track collapse/expand button clicked (track_idx)
    pub track_collapse_clicked: Option<usize>,
    /// Region that was clicked (track_idx, region_idx)
    pub region_clicked: Option<(usize, usize)>,
    /// Empty area clicked (track_idx, beat_position)
    pub empty_clicked: Option<(usize, f32)>,
    /// Playhead was moved
    pub playhead_moved: bool,
    /// Playhead was clicked (not just moved)
    pub playhead_clicked: bool,
    /// Current playhead position in beats
    pub playhead_position: f32,
    /// Which marker was moved (if any)
    pub marker_moved: Option<usize>,
}

/// Timeline component
///
/// Complete scrollable timeline view with synchronized ruler, playhead, headers, and tracks.
///
/// # Example
///
/// ```rust,no_run
/// use armas::components::audio::{Timeline, Track, Region};
///
/// fn ui(ui: &mut egui::Ui, theme: &armas::Theme) {
///     let mut tracks = vec![
///         Track::new("Vocals", egui::Color32::from_rgb(255, 100, 100))
///             .with_region(Region::new("Verse", 0.0, 4.0))
///             .with_region(Region::new("Chorus", 8.0, 4.0)),
///         Track::new("Guitar", egui::Color32::from_rgb(100, 255, 100))
///             .with_region(Region::new("Riff", 0.0, 8.0)),
///     ];
///
///     let mut playhead_pos = 0.0;
///
///     Timeline::new()
///         .track_header_width(200.0)
///         .track_height(80.0)
///         .beat_width(60.0)
///         .measures(16)
///         .show(ui, &mut tracks, &mut playhead_pos, theme);
/// }
/// ```
pub struct Timeline<'a> {
    /// Optional ID for the timeline
    id: Option<egui::Id>,
    /// Width of track header column
    track_header_width: f32,
    /// Height of each track
    track_height: f32,
    /// Width per beat in pixels
    beat_width: f32,
    /// Number of measures to display
    measures: u32,
    /// Beats per measure
    beats_per_measure: u32,
    /// Height of ruler at top
    ruler_height: f32,
    /// Show playhead
    show_playhead: bool,
    /// Playhead color
    playhead_color: Option<Color32>,
    /// Beat position to scroll to (if Some)
    scroll_to_beat: Option<f32>,
    /// All markers (cue points, tempo, time signature, etc.)
    markers: Option<&'a mut Vec<MarkerData>>,
    /// Loop region
    loop_region: Option<&'a mut LoopRegionData>,
    /// Selection range
    selection_range: Option<&'a mut SelectionRangeData>,
    /// Punch in/out region
    punch_region: Option<&'a mut PunchRegionData>,
    /// Show snap grid
    show_snap_grid: bool,
    /// Snap grid subdivision
    snap_grid_subdivision: u32,
    /// Minimum zoom level (beat_width multiplier)
    min_zoom: f32,
    /// Maximum zoom level (beat_width multiplier)
    max_zoom: f32,
    /// Auto-follow playhead during playback
    auto_follow_playhead: bool,
    /// Margin percent for auto-follow (0.0-1.0)
    auto_follow_margin: f32,
    /// Render margin in beats (how far outside viewport to render)
    visible_render_margin: f32,
    /// Empty state message
    empty_message: Option<String>,
}

/// Info about a track in the flattened hierarchy
#[derive(Debug, Clone)]
struct TrackInfo {
    /// Path to this track (indices at each level)
    path: Vec<usize>,
    /// Unique track index for interaction
    track_idx: usize,
    /// Indentation level
    indent_level: usize,
    /// Parent track color (for gradient)
    parent_color: Option<Color32>,
}

impl<'a> Timeline<'a> {
    /// Create a new timeline
    pub fn new() -> Self {
        Self {
            id: None,
            track_header_width: 200.0,
            track_height: 80.0,
            beat_width: 60.0,
            measures: 16,
            beats_per_measure: 4,
            ruler_height: 40.0,
            show_playhead: true,
            playhead_color: None,
            scroll_to_beat: None,
            markers: None,
            loop_region: None,
            selection_range: None,
            punch_region: None,
            show_snap_grid: false,
            snap_grid_subdivision: 4,
            min_zoom: 0.5,
            max_zoom: 2.0,
            auto_follow_playhead: false,
            auto_follow_margin: 0.25,
            visible_render_margin: 2.0,
            empty_message: None,
        }
    }

    /// Flatten track hierarchy into a vec with path to each track
    /// Path is indices at each level (e.g., [0, 2, 1] = root[0].children[2].children[1])
    fn build_flat_track_list(
        tracks: &[Track],
        current_path: Vec<usize>,
        base_idx: usize,
        indent_level: usize,
        parent_color: Option<Color32>,
        out: &mut Vec<TrackInfo>,
    ) {
        for (i, track) in tracks.iter().enumerate() {
            let track_idx = base_idx + i;
            let current_color = track.color;
            let mut path = current_path.clone();
            path.push(i);

            // Add this track
            out.push(TrackInfo {
                path: path.clone(),
                track_idx,
                indent_level,
                parent_color,
            });

            // Add children if folder and not collapsed
            if track.is_folder && !track.collapsed {
                Self::build_flat_track_list(
                    &track.children,
                    path,
                    track_idx * 1000 + 1,
                    indent_level + 1,
                    Some(current_color),
                    out,
                );
            }
        }
    }

    /// Get a mutable reference to a track by path
    fn get_track_by_path<'b>(tracks: &'b [Track], path: &[usize]) -> Option<&'b Track> {
        if path.is_empty() {
            return None;
        }

        let mut current = &tracks[path[0]];
        for &idx in &path[1..] {
            current = &current.children[idx];
        }
        Some(current)
    }

    fn get_track_by_path_mut<'b>(tracks: &'b mut [Track], path: &[usize]) -> Option<&'b mut Track> {
        if path.is_empty() {
            return None;
        }

        let mut current = &mut tracks[path[0]];
        for &idx in &path[1..] {
            current = &mut current.children[idx];
        }
        Some(current)
    }

    /// Set custom ID (important when using multiple timelines)
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set track header width
    pub fn track_header_width(mut self, width: f32) -> Self {
        self.track_header_width = width;
        self
    }

    /// Set track height
    pub fn track_height(mut self, height: f32) -> Self {
        self.track_height = height;
        self
    }

    /// Set pixels per beat (zoom level)
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

    /// Set ruler height
    pub fn ruler_height(mut self, height: f32) -> Self {
        self.ruler_height = height;
        self
    }

    /// Set whether to show playhead
    pub fn show_playhead(mut self, show: bool) -> Self {
        self.show_playhead = show;
        self
    }

    /// Set playhead color
    pub fn playhead_color(mut self, color: Color32) -> Self {
        self.playhead_color = Some(color);
        self
    }

    /// Add cue point markers
    /// Set markers (cue points, tempo, time signature, etc.)
    pub fn markers(mut self, markers: &'a mut Vec<MarkerData>) -> Self {
        self.markers = Some(markers);
        self
    }

    /// Set loop region
    pub fn loop_region(mut self, loop_region: &'a mut LoopRegionData) -> Self {
        self.loop_region = Some(loop_region);
        self
    }

    /// Set selection range
    pub fn selection_range(mut self, selection_range: &'a mut SelectionRangeData) -> Self {
        self.selection_range = Some(selection_range);
        self
    }

    /// Set punch in/out region
    pub fn punch_region(mut self, punch_region: &'a mut PunchRegionData) -> Self {
        self.punch_region = Some(punch_region);
        self
    }

    /// Show snap grid
    pub fn show_snap_grid(mut self, show: bool) -> Self {
        self.show_snap_grid = show;
        self
    }

    /// Set snap grid subdivision (lines per beat)
    pub fn snap_grid_subdivision(mut self, subdivision: u32) -> Self {
        self.snap_grid_subdivision = subdivision;
        self
    }

    /// Scroll to show a specific beat position
    ///
    /// This is useful for:
    /// - Following playhead during playback
    /// - Jumping to a specific position after user action
    /// - Auto-scrolling to keep important content visible
    ///
    /// The timeline will smoothly scroll to center the specified beat position.
    pub fn scroll_to_beat(mut self, beat: f32) -> Self {
        self.scroll_to_beat = Some(beat);
        self
    }

    /// Set minimum zoom level (beat_width multiplier)
    ///
    /// Default is 0.5x (50% of normal beat_width).
    /// This prevents zooming out too far.
    ///
    /// # Example
    /// ```
    /// Timeline::new()
    ///     .min_zoom(0.25)  // Allow zooming to 1/4 size
    ///     .max_zoom(3.0)   // Allow zooming to 3x size
    /// ```
    pub fn min_zoom(mut self, min: f32) -> Self {
        self.min_zoom = min.max(0.1);
        self
    }

    /// Set maximum zoom level (beat_width multiplier)
    ///
    /// Default is 2.0x (200% of normal beat_width).
    /// This prevents zooming in too far.
    pub fn max_zoom(mut self, max: f32) -> Self {
        self.max_zoom = max.min(10.0);
        self
    }

    /// Enable auto-follow playhead during playback
    ///
    /// When enabled, the timeline automatically scrolls to keep the playhead
    /// visible within the specified margin during playback.
    ///
    /// Default is disabled. Use with `.auto_follow_margin()` to control behavior.
    pub fn auto_follow_playhead(mut self, follow: bool) -> Self {
        self.auto_follow_playhead = follow;
        self
    }

    /// Set the margin percent for auto-follow (0.0-1.0)
    ///
    /// Controls how far from the edge the playhead can travel before auto-scrolling.
    /// - 0.0: Playhead at left edge
    /// - 0.25: Playhead at 25% from left (default)
    /// - 0.5: Playhead in center
    /// - 1.0: Playhead at right edge
    pub fn auto_follow_margin(mut self, margin: f32) -> Self {
        self.auto_follow_margin = margin.clamp(0.0, 1.0);
        self
    }

    /// Set render margin outside viewport (in beats)
    ///
    /// Controls how far outside the visible area content is rendered.
    /// Larger values = smoother scrolling but more CPU cost.
    /// Smaller values = lower CPU cost but may see content pop in.
    ///
    /// Default is 2.0 beats.
    pub fn visible_render_margin(mut self, beats: f32) -> Self {
        self.visible_render_margin = beats.max(0.0);
        self
    }

    /// Set empty state message when no tracks exist
    ///
    /// Displays a message in the timeline area when there are no tracks.
    /// This provides helpful guidance to users about what to do next.
    ///
    /// # Example
    /// ```
    /// Timeline::new()
    ///     .empty_message("No tracks yet. Click '+' to add a track.")
    /// ```
    pub fn empty_message(mut self, message: impl Into<String>) -> Self {
        self.empty_message = Some(message.into());
        self
    }

    /// Render a single track and its children recursively
    /// Render a single track header (non-recursive, flat rendering)
    #[allow(clippy::too_many_arguments)]
    fn render_track_header_flat(
        &self,
        ui: &mut Ui,
        track: &mut Track,
        track_idx: usize,
        indent_level: usize,
        parent_color: Option<Color32>,
        track_clicked: &mut Option<usize>,
        track_mute_clicked: &mut Option<usize>,
        track_solo_clicked: &mut Option<usize>,
        track_arm_clicked: &mut Option<usize>,
        track_collapse_clicked: &mut Option<usize>,
        theme: &Theme,
    ) -> egui::Rect {
        let header_id = self
            .id
            .unwrap_or_else(|| ui.id())
            .with("track_header")
            .with(track_idx)
            .with(indent_level);

        let mut header = TrackHeader::new()
            .id(header_id)
            .width(self.track_header_width)
            .height(self.track_height)
            .color(track.color)
            .is_folder(track.is_folder)
            .indent_level(indent_level);

        // Set parent color for nested folders
        if let Some(parent_col) = parent_color {
            header = header.parent_color(parent_col);
        }

        let header_response = header.show(
            ui,
            &mut track.name,
            &mut track.controls,
            &mut track.collapsed,
            theme,
        );

        // Capture all track header interactions
        if header_response.response.clicked() {
            *track_clicked = Some(track_idx);
        }
        if header_response.mute_clicked {
            *track_mute_clicked = Some(track_idx);
        }
        if header_response.solo_clicked {
            *track_solo_clicked = Some(track_idx);
        }
        if header_response.arm_clicked {
            *track_arm_clicked = Some(track_idx);
        }
        if header_response.collapse_clicked {
            *track_collapse_clicked = Some(track_idx);
        }

        header_response.response.rect
    }

    /// Render a single track timeline (non-recursive, flat rendering)
    fn render_track_timeline_flat(
        &self,
        ui: &mut Ui,
        track: &mut Track,
        track_idx: usize,
        region_clicked: &mut Option<(usize, usize)>,
        empty_clicked: &mut Option<(usize, f32)>,
        theme: &Theme,
    ) -> egui::Rect {
        let track_id = self
            .id
            .unwrap_or_else(|| ui.id())
            .with("timeline_track")
            .with(track_idx);

        let track_response = TimelineTrack::new()
            .id(track_id)
            .height(self.track_height)
            .beat_width(self.beat_width)
            .measures(self.measures)
            .beats_per_measure(self.beats_per_measure)
            .track_color(track.color)
            .show(ui, &mut track.regions, theme);

        if let Some(region_idx) = track_response.region_clicked {
            *region_clicked = Some((track_idx, region_idx));
        }

        if let Some(beat_pos) = track_response.empty_clicked {
            *empty_clicked = Some((track_idx, beat_pos));
        }

        track_response.response.rect
    }

    /// Show the timeline
    #[allow(clippy::ptr_arg)]
    pub fn show(
        self,
        ui: &mut Ui,
        tracks: &mut Vec<Track>,
        playhead_position: &mut f32,
        theme: &Theme,
    ) -> TimelineResponse {
        let mut track_clicked = None;
        let mut track_mute_clicked = None;
        let mut track_solo_clicked = None;
        let mut track_arm_clicked = None;
        let mut track_collapse_clicked = None;
        let mut region_clicked = None;
        let mut empty_clicked = None;
        let mut playhead_moved = false;

        // Get or create scroll state
        let scroll_id = self.id.unwrap_or_else(|| ui.id()).with("timeline_scroll");
        let mut scroll_offset = ui
            .ctx()
            .data_mut(|d| d.get_persisted::<Vec2>(scroll_id).unwrap_or(Vec2::ZERO));

        // Build flattened track list
        let mut flat_list = Vec::new();
        Self::build_flat_track_list(tracks, Vec::new(), 0, 0, None, &mut flat_list);

        // Calculate content dimensions
        let timeline_content_width =
            self.measures as f32 * self.beats_per_measure as f32 * self.beat_width;
        let timeline_content_height = flat_list.len() as f32 * self.track_height;

        // Get available space
        let available_rect = ui.available_rect_before_wrap();
        // Ensure minimum dimensions for usability
        let timeline_width = (available_rect.width() - self.track_header_width).max(100.0);
        let timeline_height = (available_rect.height() - self.ruler_height).max(100.0);

        // Calculate max scroll
        let max_scroll_x = (timeline_content_width - timeline_width).max(0.0);
        let max_scroll_y = (timeline_content_height - timeline_height).max(0.0);

        // Handle scroll_to_beat
        if let Some(beat) = self.scroll_to_beat {
            scroll_offset.x = beat * self.beat_width;
        }

        // Handle auto-follow playhead
        if self.auto_follow_playhead {
            let playhead_x = *playhead_position * self.beat_width;
            let margin_width = timeline_width * self.auto_follow_margin;

            // If playhead is outside the visible area, auto-scroll to keep it within margin
            if playhead_x < scroll_offset.x + margin_width {
                scroll_offset.x = (playhead_x - margin_width).max(0.0);
            } else if playhead_x > scroll_offset.x + timeline_width - margin_width {
                scroll_offset.x = (playhead_x - timeline_width + margin_width).min(max_scroll_x);
            }
        }

        // We'll handle scroll input after rendering to ensure we capture events properly

        // === RENDER WITHOUT GRID (Manual Positioning) ===
        let response = ui.vertical(|ui| {
            // Row 1: Corner + Ruler
            ui.horizontal(|ui| {
                // Top-left corner
                ui.allocate_exact_size(Vec2::new(self.track_header_width, self.ruler_height), Sense::hover());

                // Ruler
                let ruler_rect = ui.allocate_exact_size(
                    Vec2::new(timeline_width, self.ruler_height),
                    Sense::hover(),
                ).0;

                // Draw ruler with horizontal scroll
                ui.painter().with_clip_rect(ruler_rect).rect_filled(
                    ruler_rect,
                    0.0,
                    theme.surface_variant()
                );

                // Render TimeRuler component with offset and clipping
                let mut ruler_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(ruler_rect.translate(Vec2::new(-scroll_offset.x, 0.0)))
                        .layout(egui::Layout::left_to_right(egui::Align::Min)),
                );
                ruler_ui.set_clip_rect(ruler_rect);
                TimeRuler::new()
                    .id(self.id.unwrap_or_else(|| ui.id()).with("ruler"))
                    .height(self.ruler_height)
                    .beat_width(self.beat_width)
                    .measures(self.measures)
                    .beats_per_measure(self.beats_per_measure)
                    .show_clipped(&mut ruler_ui, theme);
            });

            // Row 2: Headers + Tracks
            ui.horizontal(|ui| {
                // Left: Headers container
                let headers_rect = ui.allocate_exact_size(
                    Vec2::new(self.track_header_width, timeline_height),
                    Sense::hover(),
                ).0;

                // Clip and render headers with vertical scroll
                ui.painter().with_clip_rect(headers_rect);

                // Only render if we have tracks and there's space to render
                if !flat_list.is_empty() && timeline_height > 0.0 {
                    // Calculate which tracks are visible
                    let first_visible = (scroll_offset.y / self.track_height).floor() as usize;
                    let last_visible = ((scroll_offset.y + timeline_height) / self.track_height).ceil() as usize;

                    // Clamp to valid range
                    let first_visible = first_visible.min(flat_list.len().saturating_sub(1));
                    let last_visible = last_visible.min(flat_list.len().saturating_sub(1));

                    for idx in first_visible..=last_visible {
                        if let Some(info) = flat_list.get(idx) {
                            if let Some(track) = Self::get_track_by_path_mut(tracks, &info.path) {
                                let y_pos = idx as f32 * self.track_height - scroll_offset.y;
                                let header_rect = Rect::from_min_size(
                                    headers_rect.min + Vec2::new(0.0, y_pos),
                                    Vec2::new(self.track_header_width, self.track_height)
                                );

                                // Always render headers that are at least partially visible
                                if header_rect.max.y > headers_rect.min.y && header_rect.min.y < headers_rect.max.y {
                                    let mut header_ui = ui.new_child(
                                        egui::UiBuilder::new()
                                            .max_rect(header_rect)
                                            .layout(egui::Layout::top_down(egui::Align::Min)),
                                    );
                                    header_ui.set_clip_rect(headers_rect);

                                    self.render_track_header_flat(
                                        &mut header_ui,
                                        track,
                                        info.track_idx,
                                        info.indent_level,
                                        info.parent_color,
                                        &mut track_clicked,
                                        &mut track_mute_clicked,
                                        &mut track_solo_clicked,
                                        &mut track_arm_clicked,
                                        &mut track_collapse_clicked,
                                        theme,
                                    );

                                    // Update track selection state when clicked
                                    if let Some(clicked_idx) = track_clicked {
                                        if clicked_idx == info.track_idx {
                                            track.selected = !track.selected;
                                            track_clicked = None; // Reset so we don't process it multiple times
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Draw selection borders on selected track headers
                let painter = ui.painter();
                if !flat_list.is_empty() {
                    let first_visible = (scroll_offset.y / self.track_height).floor() as usize;
                    let last_visible = ((scroll_offset.y + timeline_height) / self.track_height).ceil() as usize;

                    for idx in first_visible..=last_visible.min(flat_list.len() - 1) {
                        if let Some(info) = flat_list.get(idx) {
                            if let Some(track) = Self::get_track_by_path(tracks, &info.path) {
                                if track.selected {
                                    let y_pos = idx as f32 * self.track_height - scroll_offset.y;
                                    let header_rect = egui::Rect::from_min_size(
                                        headers_rect.min + Vec2::new(0.0, y_pos),
                                        Vec2::new(self.track_header_width, self.track_height)
                                    );

                                    if header_rect.intersects(headers_rect) {
                                        painter.rect_stroke(
                                            header_rect,
                                            2.0,
                                            egui::Stroke::new(2.0, theme.primary()),
                                            egui::StrokeKind::Outside,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }

                // Right: Tracks container
                let tracks_rect = ui.allocate_exact_size(
                    Vec2::new(timeline_width, timeline_height),
                    Sense::hover(),
                ).0;

                // Clip and render tracks with both scrolls
                ui.painter().with_clip_rect(tracks_rect);

                // Only render if we have tracks and there's space to render
                if !flat_list.is_empty() && timeline_height > 0.0 {
                    // Calculate which tracks are visible (same as headers)
                    let first_visible = (scroll_offset.y / self.track_height).floor() as usize;
                    let last_visible = ((scroll_offset.y + timeline_height) / self.track_height).ceil() as usize;

                    // Clamp to valid range
                    let first_visible = first_visible.min(flat_list.len().saturating_sub(1));
                    let last_visible = last_visible.min(flat_list.len().saturating_sub(1));

                    for idx in first_visible..=last_visible {
                        if let Some(info) = flat_list.get(idx) {
                            if let Some(track) = Self::get_track_by_path_mut(tracks, &info.path) {
                                let y_pos = idx as f32 * self.track_height - scroll_offset.y;
                                let track_rect = Rect::from_min_size(
                                    tracks_rect.min + Vec2::new(-scroll_offset.x, y_pos),
                                    Vec2::new(timeline_content_width, self.track_height)
                                );

                                // Create child UI with clipping to prevent overflow into header area
                                let mut track_ui = ui.new_child(
                                    egui::UiBuilder::new()
                                        .max_rect(track_rect)
                                        .layout(egui::Layout::top_down(egui::Align::Min)),
                                );
                                track_ui.set_clip_rect(tracks_rect);

                                self.render_track_timeline_flat(
                                    &mut track_ui,
                                    track,
                                    info.track_idx,
                                    &mut region_clicked,
                                    &mut empty_clicked,
                                    theme,
                                );
                            }
                        }
                    }
                }

                // Empty state message
                if flat_list.is_empty() {
                    if let Some(msg) = &self.empty_message {
                        let empty_rect = Rect::from_min_size(
                            tracks_rect.min,
                            Vec2::new(timeline_width, timeline_height)
                        );

                        // Draw centered empty message
                        let painter = ui.painter();
                        painter.rect_filled(empty_rect, 0.0, theme.surface());

                        let text_center = empty_rect.center();
                        painter.text(
                            text_center,
                            egui::Align2::CENTER_CENTER,
                            msg,
                            egui::FontId::default(),
                            theme.on_surface(),
                        );
                    }
                }

                // Grid overlay
                if self.show_snap_grid {
                    let painter = ui.painter().with_clip_rect(tracks_rect);
                    for measure in 0..self.measures {
                        for beat in 0..self.beats_per_measure {
                            let x = tracks_rect.min.x - scroll_offset.x + (measure * self.beats_per_measure + beat) as f32 * self.beat_width;
                            if x >= tracks_rect.min.x && x <= tracks_rect.max.x {
                                let is_measure = beat == 0;
                                let color = if is_measure {
                                    theme.outline().linear_multiply(0.3)
                                } else {
                                    theme.outline_variant().linear_multiply(0.2)
                                };
                                painter.line_segment(
                                    [pos2(x, tracks_rect.min.y), pos2(x, tracks_rect.max.y)],
                                    (if is_measure { 1.0 } else { 0.5 }, color),
                                );
                            }
                        }
                    }
                }
            });

            // Handle scroll input (mouse wheel/trackpad)
            // Create an invisible interaction area over the timeline content
            let scroll_area_rect = Rect::from_min_size(
                available_rect.min + Vec2::new(0.0, self.ruler_height),
                Vec2::new(available_rect.width(), timeline_height)
            );

            let scroll_response = ui.interact(scroll_area_rect, scroll_id.with("scroll"), Sense::hover());
            if scroll_response.hovered() {
                ui.ctx().input(|i| {
                    let delta = i.smooth_scroll_delta;
                    if delta != Vec2::ZERO {
                        scroll_offset.x -= delta.x;
                        scroll_offset.y -= delta.y;
                        scroll_offset.x = scroll_offset.x.clamp(0.0, max_scroll_x);
                        scroll_offset.y = scroll_offset.y.clamp(0.0, max_scroll_y);
                    }
                });
            }
        }).response;

        // Persist scroll offset
        ui.ctx().data_mut(|d| {
            d.insert_persisted(scroll_id, scroll_offset);
        });

        // Get ruler rect for markers (we need to reference the actual allocated ruler rect)
        let ruler_rect = Rect::from_min_size(
            available_rect.min + Vec2::new(self.track_header_width, 0.0),
            Vec2::new(timeline_width, self.ruler_height)
        );

        // Render point markers in the ruler area
        // NOTE: We use a child UI with max_rect instead of a separate layer to respect parent z-order
        if self.markers.is_some() && !ruler_rect.is_negative() {
            let marker_rect = Rect::from_min_size(
                pos2(ruler_rect.min.x - scroll_offset.x, ruler_rect.min.y),
                vec2(timeline_content_width, ruler_rect.height())
            );

            let mut marker_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(marker_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Min)),
            );
            marker_ui.set_clip_rect(ruler_rect);

            if let Some(markers) = self.markers {
                for (i, marker_data) in markers.iter_mut().enumerate() {
                    let vertical_range = if marker_data.content.contains("BPM") {
                        (0.33, 0.67)
                    } else if marker_data.content.contains('/') && marker_data.content.len() < 6 {
                        (0.67, 1.0)
                    } else {
                        (0.0, 0.33)
                    };

                    let mut marker = Marker::new(&mut marker_data.position, &marker_data.content)
                        .beat_width(self.beat_width)
                        .measures(self.measures)
                        .beats_per_measure(self.beats_per_measure)
                        .height(self.ruler_height)
                        .vertical_range(vertical_range.0, vertical_range.1)
                        .id(self.id.unwrap_or_else(|| ui.id()).with("marker").with(i));

                    if let Some(color) = marker_data.color {
                        marker = marker.color(color);
                    }

                    marker.show(&mut marker_ui);
                }
            }
        }

        // Render snap grid overlay if enabled
        // NOTE: We use a child UI with max_rect instead of a separate layer to respect parent z-order
        if self.show_snap_grid && timeline_height > 0.0 {
            let tracks_rect = Rect::from_min_size(
                available_rect.min + Vec2::new(self.track_header_width, self.ruler_height),
                Vec2::new(timeline_width, timeline_height)
            );

            let grid_rect = Rect::from_min_size(
                pos2(tracks_rect.min.x - scroll_offset.x, tracks_rect.min.y - scroll_offset.y),
                vec2(timeline_content_width, timeline_content_height)
            );

            let mut grid_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(grid_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Min)),
            );
            grid_ui.set_clip_rect(tracks_rect);

            SnapGrid::new()
                .beat_width(self.beat_width)
                .measures(self.measures)
                .beats_per_measure(self.beats_per_measure)
                .subdivision(self.snap_grid_subdivision)
                .show_overlay(&mut grid_ui);
        }

        // Render region markers (loop, selection, punch) overlaying tracks
        // NOTE: We use child UIs instead of separate layers to respect parent z-order
        // The z_order_id is still used to track which marker was last interacted with
        // for internal ordering between the three markers
        if !flat_list.is_empty() && timeline_height > 0.0 {
            let tracks_rect = Rect::from_min_size(
                available_rect.min + Vec2::new(self.track_header_width, self.ruler_height),
                Vec2::new(timeline_width, timeline_height)
            );

            let markers_rect = Rect::from_min_size(
                pos2(tracks_rect.min.x - scroll_offset.x, tracks_rect.min.y - scroll_offset.y),
                vec2(timeline_content_width, timeline_content_height)
            );

            let z_order_id = self.id.unwrap_or_else(|| ui.id()).with("marker_z_order");
            let mut top_marker: u8 = ui.ctx().data_mut(|d| d.get_temp(z_order_id).unwrap_or(2));

            // Determine render order based on which marker is "on top"
            // We render in back-to-front order so the top marker is drawn last
            let render_order: [u8; 3] = match top_marker {
                0 => [1, 2, 0], // Loop on top: render selection, punch, then loop
                1 => [0, 2, 1], // Selection on top: render loop, punch, then selection
                _ => [0, 1, 2], // Punch on top (default): render loop, selection, then punch
            };

            // We need to render markers in a specific order, but we can only consume each Option once
            // So we extract them first, then render in order
            let mut loop_region = self.loop_region;
            let mut selection_range = self.selection_range;
            let mut punch_region = self.punch_region;

            for marker_type in render_order {
                match marker_type {
                    0 => {
                        // Render loop region
                        if let Some(loop_data) = loop_region.take() {
                            let mut loop_ui = ui.new_child(
                                egui::UiBuilder::new()
                                    .max_rect(markers_rect)
                                    .layout(egui::Layout::left_to_right(egui::Align::Min)),
                            );
                            loop_ui.set_clip_rect(tracks_rect);

                            let loop_response = LoopRegionMarker::new(&mut loop_data.start, &mut loop_data.end)
                                .beat_width(self.beat_width)
                                .measures(self.measures)
                                .beats_per_measure(self.beats_per_measure)
                                .height(timeline_content_height)
                                .vertical_range(0.0, 0.5)
                                .id(self.id.unwrap_or_else(|| ui.id()).with("loop_region"))
                                .show(&mut loop_ui);

                            if loop_response.region_clicked || loop_response.loop_start_changed || loop_response.loop_end_changed {
                                top_marker = 0;
                            }
                        }
                    }
                    1 => {
                        // Render selection range
                        if let Some(selection_data) = selection_range.take() {
                            let mut selection_ui = ui.new_child(
                                egui::UiBuilder::new()
                                    .max_rect(markers_rect)
                                    .layout(egui::Layout::left_to_right(egui::Align::Min)),
                            );
                            selection_ui.set_clip_rect(tracks_rect);

                            let selection_response = SelectionRange::new(&mut selection_data.start, &mut selection_data.end)
                                .beat_width(self.beat_width)
                                .measures(self.measures)
                                .beats_per_measure(self.beats_per_measure)
                                .height(timeline_content_height)
                                .vertical_range(0.33, 0.67)
                                .id(self.id.unwrap_or_else(|| ui.id()).with("selection_range"))
                                .show(&mut selection_ui);

                            if selection_response.region_clicked || selection_response.selection_start_changed || selection_response.selection_end_changed {
                                top_marker = 1;
                            }
                        }
                    }
                    _ => {
                        // Render punch region
                        if let Some(punch_data) = punch_region.take() {
                            let mut punch_ui = ui.new_child(
                                egui::UiBuilder::new()
                                    .max_rect(markers_rect)
                                    .layout(egui::Layout::left_to_right(egui::Align::Min)),
                            );
                            punch_ui.set_clip_rect(tracks_rect);

                            let punch_response = PunchMarker::new(&mut punch_data.punch_in, &mut punch_data.punch_out)
                                .beat_width(self.beat_width)
                                .measures(self.measures)
                                .beats_per_measure(self.beats_per_measure)
                                .height(timeline_content_height)
                                .vertical_range(0.5, 1.0)
                                .id(self.id.unwrap_or_else(|| ui.id()).with("punch_region"))
                                .show(&mut punch_ui);

                            if punch_response.region_clicked || punch_response.punch_in_changed || punch_response.punch_out_changed {
                                top_marker = 2;
                            }
                        }
                    }
                }
            }

            ui.ctx().data_mut(|d| d.insert_temp(z_order_id, top_marker));
        }

        // Playhead - now using Playhead component
        // NOTE: We use a child UI instead of a separate layer to respect parent z-order
        if self.show_playhead && timeline_height > 0.0 {
            let total_height = available_rect.height();

            let playhead_rect = Rect::from_min_size(
                pos2(available_rect.min.x + self.track_header_width - scroll_offset.x, available_rect.min.y),
                vec2(timeline_content_width, total_height)
            );

            let playhead_clip_rect = Rect::from_min_size(
                available_rect.min + Vec2::new(self.track_header_width, 0.0),
                Vec2::new(timeline_width, total_height)
            );

            let playhead_id = self.id.unwrap_or_else(|| ui.id()).with("playhead");
            let mut playhead = Playhead::new()
                .beat_width(self.beat_width)
                .height(total_height)
                .id(playhead_id);

            if let Some(color) = self.playhead_color {
                playhead = playhead.color(color);
            }

            let mut playhead_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(playhead_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Min)),
            );
            playhead_ui.set_clip_rect(playhead_clip_rect);

            let playhead_response = playhead.show_in_rect(&mut playhead_ui, playhead_rect, playhead_position, theme);

            if playhead_response.changed() {
                playhead_moved = true;
            }
        }

        TimelineResponse {
            response,
            track_clicked,
            track_mute_clicked,
            track_solo_clicked,
            track_arm_clicked,
            track_collapse_clicked,
            region_clicked,
            empty_clicked,
            playhead_moved,
            playhead_clicked: false, // Can be enhanced to detect actual clicks vs drags
            playhead_position: *playhead_position,
            marker_moved: None, // Can be enhanced to track marker movements
        }
    }
}

impl<'a> Default for Timeline<'a> {
    fn default() -> Self {
        Self::new()
    }
}