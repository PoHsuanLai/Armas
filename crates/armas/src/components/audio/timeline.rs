//! Timeline Component
//!
//! Complete scrollable timeline view combining ruler, playhead, track headers, and tracks.

use crate::components::audio::{
    LoopRegionMarker, Marker, Playhead, PunchMarker, Region, SelectionRange, SnapGrid, TimeRuler,
    TimelineTrack, TrackControls, TrackHeader,
};
use crate::theme::Theme;
use egui::{Color32, Rect, Response, ScrollArea, Sense, Ui, Vec2};

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
}

/// Response from timeline interaction
#[derive(Debug, Clone)]
pub struct TimelineResponse {
    /// The egui response for the entire timeline
    pub response: Response,
    /// Track index that was clicked (if any)
    pub track_clicked: Option<usize>,
    /// Region that was clicked (track_idx, region_idx)
    pub region_clicked: Option<(usize, usize)>,
    /// Empty area clicked (track_idx, beat_position)
    pub empty_clicked: Option<(usize, f32)>,
    /// Playhead was moved
    pub playhead_moved: bool,
    /// Current playhead position in beats
    pub playhead_position: f32,
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

        if header_response.response.clicked() {
            *track_clicked = Some(track_idx);
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
        let mut region_clicked = None;
        let mut empty_clicked = None;
        let mut playhead_moved = false;
        let mut ruler_rect = Rect::NOTHING;
        let mut tracks_rect = Rect::NOTHING;

        // Get or create shared scroll offset for synchronization
        let scroll_id = self.id.unwrap_or_else(|| ui.id()).with("shared_scroll");
        let mut scroll_offset = ui
            .ctx()
            .data_mut(|d| d.get_persisted::<Vec2>(scroll_id).unwrap_or(Vec2::ZERO));

        // Track previous scroll_to_beat value to detect changes
        let prev_beat_id = self.id.unwrap_or_else(|| ui.id()).with("prev_scroll_to_beat");
        let prev_beat = ui.ctx().data(|d| d.get_temp::<f32>(prev_beat_id));

        // Apply scroll_to_beat only if it changed from the previous frame
        if let Some(beat) = self.scroll_to_beat {
            // Only update scroll if the beat position changed
            if prev_beat != Some(beat) {
                scroll_offset.x = beat * self.beat_width;
                // Store the new beat value for next frame comparison
                ui.ctx().data_mut(|d| d.insert_temp(prev_beat_id, beat));
            }
        } else {
            // Clear the stored beat if scroll_to_beat is not set
            ui.ctx().data_mut(|d| d.remove::<f32>(prev_beat_id));
        }

        let mut timeline_rects_for_markers = Vec::new();
        let mut tracks_clip_rect = Rect::NOTHING;
        let mut header_cell_width = 0.0;
        let expected_width = self.track_header_width;

        let vertical_response = ui.vertical(|ui| {
            // Calculate the full available width before Grid constrains it
            let full_width = ui.available_width();
            let timeline_width = full_width - self.track_header_width; // Width for ruler/tracks column

            // Use Grid for perfect alignment between ruler row and tracks row
            egui::Grid::new(self.id.unwrap_or_else(|| ui.id()).with("timeline_grid"))
                .spacing([0.0, 0.0]) // No spacing between ruler and tracks
                .show(ui, |ui| {
                    // Row 1: Empty space + Ruler
                    // Wrap first column in fixed-width container
                    ui.allocate_exact_size(
                        Vec2::new(self.track_header_width, self.ruler_height),
                        Sense::hover(),
                    );

                    // Fixed time ruler area that shows offset content
                    let ruler_id = self.id.unwrap_or_else(|| ui.id()).with("ruler");

                    // Allocate the calculated timeline width for second column
                    let (ruler_response, _painter) = ui.allocate_painter(
                        egui::Vec2::new(timeline_width, self.ruler_height),
                        Sense::hover(),
                    );

                    // Store ruler rect for playhead positioning
                    ruler_rect = ruler_response.rect;

                    // Push a child UI with negative offset to simulate scrolling
                    let mut child_ui = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(ruler_response.rect)
                            .layout(egui::Layout::left_to_right(egui::Align::Min)),
                    );

                    // Offset the child UI to simulate scrolling
                    child_ui.set_clip_rect(ruler_response.rect);
                    let offset_rect = ruler_response
                        .rect
                        .translate(egui::Vec2::new(-scroll_offset.x, 0.0));
                    let mut offset_ui = child_ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(offset_rect)
                            .layout(egui::Layout::left_to_right(egui::Align::Min)),
                    );

                    TimeRuler::new()
                        .id(ruler_id)
                        .height(self.ruler_height)
                        .beat_width(self.beat_width)
                        .measures(self.measures)
                        .beats_per_measure(self.beats_per_measure)
                        .show(&mut offset_ui, theme);

                    ui.end_row(); // End first grid row

                    // Build flattened track list (includes hierarchy info and path to each track)
                    let mut flat_list = Vec::new();
                    Self::build_flat_track_list(tracks, Vec::new(), 0, 0, None, &mut flat_list);

                    // Row 2: Track Headers + Timeline Tracks
                    // Left cell: Track headers (match Row 1's allocation style)
                    let header_cell_response = ui.vertical(|ui| {
                        ui.set_width(self.track_header_width);
                        ui.set_min_height(0.0); // Prevent minimum height
                        ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO; // Remove all spacing

                        let headers_scroll_id =
                            self.id.unwrap_or_else(|| ui.id()).with("headers_scroll");
                        let scroll_response = ScrollArea::vertical()
                            .id_salt(headers_scroll_id)
                            .scroll_offset(Vec2::new(0.0, scroll_offset.y))
                            .scroll_bar_visibility(
                                egui::scroll_area::ScrollBarVisibility::AlwaysHidden,
                            )
                            .auto_shrink([false, false]) // Don't shrink to content
                            .show(ui, |ui| {
                                // Remove item spacing - Cards have their own margins
                                ui.spacing_mut().item_spacing.y = 0.0;
                                // Set max width to prevent headers from extending beyond the cell
                                ui.set_max_width(self.track_header_width);

                                // Set clip rect to hard-clip any content that exceeds the width
                                let clip_rect = Rect::from_min_size(
                                    ui.cursor().min,
                                    Vec2::new(self.track_header_width, f32::INFINITY)
                                );
                                ui.set_clip_rect(clip_rect);

                                let mut rects = Vec::new();

                                // Render each track in flattened order
                                for info in &flat_list {
                                    if let Some(track) =
                                        Self::get_track_by_path_mut(tracks, &info.path)
                                    {
                                        let rect = self.render_track_header_flat(
                                            ui,
                                            track,
                                            info.track_idx,
                                            info.indent_level,
                                            info.parent_color,
                                            &mut track_clicked,
                                            theme,
                                        );
                                        rects.push(rect);
                                    }
                                }
                                rects
                            });
                            (scroll_response.inner, scroll_response.inner_rect)
                    });

                    let (header_rects, header_viewport) = header_cell_response.inner;

                    // Debug: Check header cell width
                    header_cell_width = header_cell_response.response.rect.width();

                    // Right cell: Timeline tracks (match Row 1's allocation style)
                    let tracks_cell_response = ui.vertical(|ui| {
                        ui.set_width(timeline_width);
                        ui.set_min_height(0.0); // Prevent minimum height
                        ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO; // Remove all spacing

                        let tracks_scroll_id =
                            self.id.unwrap_or_else(|| ui.id()).with("tracks_scroll");

                        // Build the scroll area with viewport
                        // Always pass the scroll_offset - it will be used if scroll_to_beat was set
                        let scroll_area = ScrollArea::both()
                            .id_salt(tracks_scroll_id)
                            .scroll_offset(scroll_offset)
                            .auto_shrink([false, false]); // Don't shrink to content

                        let tracks_response = scroll_area.show(ui, |ui| {
                            // Remove item spacing - Cards have their own margins
                            ui.spacing_mut().item_spacing.y = 0.0;
                            let mut rects = Vec::new();

                            // Render each track in flattened order
                            for info in &flat_list {
                                if let Some(track) =
                                    Self::get_track_by_path_mut(tracks, &info.path)
                                {
                                    let rect = self.render_track_timeline_flat(
                                        ui,
                                        track,
                                        info.track_idx,
                                        &mut region_clicked,
                                        &mut empty_clicked,
                                        theme,
                                    );
                                    rects.push(rect);
                                }
                            }
                            rects
                        });

                        tracks_rect = tracks_response.inner_rect;
                        scroll_offset = tracks_response.state.offset;
                        (tracks_response.inner, tracks_response.inner_rect)
                    });

                    let (timeline_rects, timeline_viewport) = tracks_cell_response.inner;

                    ui.end_row(); // End second grid row

                    // Draw borders around track rows (header + timeline combined)
                    // Create a combined clip rect from both scroll viewports
                    let clip_rect =
                        egui::Rect::from_min_max(header_viewport.min, timeline_viewport.max);

                    let painter = ui.painter().with_clip_rect(clip_rect);
                    for (header_rect, timeline_rect) in
                        header_rects.iter().zip(timeline_rects.iter())
                    {
                        // Combine header and timeline rects into one
                        // Use actual rect heights to ensure perfect alignment
                        let combined_rect = egui::Rect::from_min_max(
                            header_rect.min,
                            egui::Pos2::new(
                                timeline_rect.max.x,
                                header_rect.max.y.max(timeline_rect.max.y), // Use max of both to ensure full coverage
                            ),
                        );

                        painter.rect_stroke(
                            combined_rect,
                            4.0,
                            egui::Stroke::new(1.0, theme.outline()),
                            egui::StrokeKind::Inside,
                        );
                    }

                    // Store timeline_rects for region marker rendering
                    timeline_rects_for_markers = timeline_rects;
                    tracks_clip_rect = timeline_viewport; // Only clip to timeline area, not headers
                });

            scroll_offset
        });

        // Get the visible viewport rect before clipping
        let visible_viewport = vertical_response.response.rect;

        // Clip tracks_rect to the actual visible viewport
        tracks_rect = tracks_rect.intersect(visible_viewport);

        // Persist the shared scroll offset for next frame
        ui.ctx().data_mut(|d| {
            d.insert_persisted(scroll_id, scroll_offset);
        });

        // Calculate marker heights early, before any closures

        // Render point markers (cue, tempo, time signature) as overlays in the ruler area
        if !ruler_rect.is_negative() {
            // Create a layer for markers that sits above the ruler
            let marker_layer_id = egui::LayerId::new(egui::Order::Foreground, self.id.unwrap_or_else(|| ui.id()).with("markers"));

            // Point markers use full ruler height
            let marker_rect = Rect::from_min_size(
                egui::Pos2::new(ruler_rect.min.x - scroll_offset.x, ruler_rect.min.y),
                egui::Vec2::new(self.measures as f32 * self.beats_per_measure as f32 * self.beat_width, ruler_rect.height())
            );

            ui.scope_builder(egui::UiBuilder::new().layer_id(marker_layer_id).max_rect(marker_rect), |ui| {
                ui.set_clip_rect(ruler_rect); // Clip to visible ruler area

                // Render all markers - vertical position determined by content
                if let Some(markers) = self.markers {
                    for (i, marker_data) in markers.iter_mut().enumerate() {
                        // Determine vertical range based on marker type (heuristic: check content)
                        let vertical_range = if marker_data.content.contains("BPM") {
                            (0.33, 0.67) // Tempo markers in middle third
                        } else if marker_data.content.contains('/') && marker_data.content.len() < 6 {
                            (0.67, 1.0) // Time signature markers in bottom third
                        } else {
                            (0.0, 0.33) // Cue point markers in top third
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

                        marker.show(ui);
                    }
                }
            });
        }

        // Render snap grid as overlay over tracks
        if self.show_snap_grid && !tracks_rect.is_negative() {
            let grid_layer_id = egui::LayerId::new(egui::Order::Middle, self.id.unwrap_or_else(|| ui.id()).with("snap_grid"));

            let grid_rect = Rect::from_min_size(
                egui::Pos2::new(tracks_rect.min.x - scroll_offset.x, tracks_rect.min.y),
                egui::Vec2::new(
                    self.measures as f32 * self.beats_per_measure as f32 * self.beat_width,
                    tracks_rect.height()
                )
            );

            ui.scope_builder(egui::UiBuilder::new().layer_id(grid_layer_id).max_rect(grid_rect), |ui| {
                ui.set_clip_rect(tracks_rect);

                SnapGrid::new()
                    .beat_width(self.beat_width)
                    .measures(self.measures)
                    .beats_per_measure(self.beats_per_measure)
                    .subdivision(self.snap_grid_subdivision)
                    .show(ui);
            });
        }

        // Render playhead overlay if enabled (using same approach as region markers)
        if self.show_playhead && !ruler_rect.is_negative() {
            // Calculate total height from ruler to bottom of visible viewport
            let total_height = visible_viewport.max.y - ruler_rect.min.y;

            // Create playhead rect aligned with timeline content area
            // Offset by scroll to account for horizontal scrolling
            let playhead_rect = Rect::from_min_size(
                egui::Pos2::new(ruler_rect.min.x - scroll_offset.x, ruler_rect.min.y),
                egui::Vec2::new(ruler_rect.width(), total_height)
            );

            // Use a layer like region markers do, with proper clipping
            let playhead_layer_id = egui::LayerId::new(
                egui::Order::Foreground,
                self.id.unwrap_or_else(|| ui.id()).with("playhead_layer")
            );

            // Clip rect is the visible viewport (ruler area)
            let playhead_clip_rect = Rect::from_min_max(
                ruler_rect.min,
                egui::Pos2::new(ruler_rect.max.x, visible_viewport.max.y)
            );

            let playhead_id = self.id.unwrap_or_else(|| ui.id()).with("playhead");
            let mut playhead = Playhead::new(self.beat_width, total_height).id(playhead_id);

            if let Some(color) = self.playhead_color {
                playhead = playhead.color(color);
            }

            let playhead_response = ui.scope_builder(
                egui::UiBuilder::new()
                    .layer_id(playhead_layer_id)
                    .max_rect(playhead_rect),
                |ui| {
                    ui.set_clip_rect(playhead_clip_rect);
                    playhead.show_in_rect(ui, playhead_rect, playhead_position, theme)
                }
            ).inner;

            if playhead_response.changed() {
                playhead_moved = true;
            }
        }

        // Render region markers as overlays across all tracks
        // Position them starting at the first track and spanning down to cover all tracks
        if !timeline_rects_for_markers.is_empty() && !tracks_clip_rect.is_negative() {
            let first_track_rect = &timeline_rects_for_markers[0];
            let last_track_rect = &timeline_rects_for_markers[timeline_rects_for_markers.len() - 1];

            // Calculate total height from first track to last track
            let total_tracks_height = last_track_rect.max.y - first_track_rect.min.y;
            let timeline_width = self.measures as f32 * self.beats_per_measure as f32 * self.beat_width;

            // All markers share the same rect (overlaying like a zstack)
            // Position markers to start at tracks_rect.min.x (after the header column)
            let markers_rect = Rect::from_min_size(
                egui::Pos2::new(tracks_rect.min.x - scroll_offset.x, first_track_rect.min.y),
                egui::Vec2::new(timeline_width, total_tracks_height)
            );

            // Load z-order state (which marker is on top)
            // 0 = loop, 1 = selection, 2 = punch
            let z_order_id = self.id.unwrap_or_else(|| ui.id()).with("marker_z_order");
            let mut top_marker: u8 = ui.ctx().data_mut(|d| d.get_temp(z_order_id).unwrap_or(2)); // punch on top by default

            // Render loop region - base layer or brought to front
            if let Some(loop_data) = self.loop_region {
                let loop_order = if top_marker == 0 {
                    egui::Order::Foreground  // On top
                } else {
                    egui::Order::Middle      // Behind
                };

                let loop_layer_id = egui::LayerId::new(loop_order, self.id.unwrap_or_else(|| ui.id()).with("loop_region_layer"));

                let loop_response = ui.scope_builder(egui::UiBuilder::new().layer_id(loop_layer_id).max_rect(markers_rect), |ui| {
                    LoopRegionMarker::new(&mut loop_data.start, &mut loop_data.end)
                        .beat_width(self.beat_width)
                        .measures(self.measures)
                        .beats_per_measure(self.beats_per_measure)
                        .height(total_tracks_height)
                        .vertical_range(0.0, 0.5)  // Top half
                        .clip_rect(tracks_clip_rect)
                        .id(self.id.unwrap_or_else(|| ui.id()).with("loop_region"))
                        .show(ui)
                }).inner;

                // Bring to front if clicked
                if loop_response.region_clicked || loop_response.loop_start_changed || loop_response.loop_end_changed {
                    top_marker = 0;
                }
            }

            // Render selection range - middle layer or brought to front
            if let Some(selection_data) = self.selection_range {
                let selection_order = if top_marker == 1 {
                    egui::Order::Foreground
                } else {
                    egui::Order::Middle
                };

                let selection_layer_id = egui::LayerId::new(selection_order, self.id.unwrap_or_else(|| ui.id()).with("selection_range_layer"));

                let selection_response = ui.scope_builder(egui::UiBuilder::new().layer_id(selection_layer_id).max_rect(markers_rect), |ui| {
                    SelectionRange::new(&mut selection_data.start, &mut selection_data.end)
                        .beat_width(self.beat_width)
                        .measures(self.measures)
                        .beats_per_measure(self.beats_per_measure)
                        .height(total_tracks_height)
                        .vertical_range(0.33, 0.67)  // Middle third
                        .clip_rect(tracks_clip_rect)
                        .id(self.id.unwrap_or_else(|| ui.id()).with("selection_range"))
                        .show(ui)
                }).inner;

                if selection_response.region_clicked || selection_response.selection_start_changed || selection_response.selection_end_changed {
                    top_marker = 1;
                }
            }

            // Render punch region - top layer or brought to front
            if let Some(punch_data) = self.punch_region {
                let punch_order = if top_marker == 2 {
                    egui::Order::Foreground
                } else {
                    egui::Order::Middle
                };

                let punch_layer_id = egui::LayerId::new(punch_order, self.id.unwrap_or_else(|| ui.id()).with("punch_region_layer"));

                let punch_response = ui.scope_builder(egui::UiBuilder::new().layer_id(punch_layer_id).max_rect(markers_rect), |ui| {
                    PunchMarker::new(&mut punch_data.punch_in, &mut punch_data.punch_out)
                        .beat_width(self.beat_width)
                        .measures(self.measures)
                        .beats_per_measure(self.beats_per_measure)
                        .height(total_tracks_height)
                        .vertical_range(0.5, 1.0)  // Bottom half
                        .clip_rect(tracks_clip_rect)
                        .id(self.id.unwrap_or_else(|| ui.id()).with("punch_region"))
                        .show(ui)
                }).inner;

                if punch_response.region_clicked || punch_response.punch_in_changed || punch_response.punch_out_changed {
                    top_marker = 2;
                }
            }

            // Save z-order state
            ui.ctx().data_mut(|d| d.insert_temp(z_order_id, top_marker));
        }

        // Create response
        let response = ui.allocate_response(Vec2::ZERO, Sense::hover());

        // Debug: Display header width info
        ui.label(
            egui::RichText::new(format!(
                "Header width: {:.1} | Expected: {:.1} | Diff: {:.1}",
                header_cell_width,
                expected_width,
                header_cell_width - expected_width
            ))
            .small()
            .color(egui::Color32::RED)
        );

        TimelineResponse {
            response,
            track_clicked,
            region_clicked,
            empty_clicked,
            playhead_moved,
            playhead_position: *playhead_position,
        }
    }
}

impl<'a> Default for Timeline<'a> {
    fn default() -> Self {
        Self::new()
    }
}
