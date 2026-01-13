//! Timeline Component
//!
//! Complete scrollable timeline view combining ruler, playhead, track headers, and tracks.

use crate::components::audio::{
    Playhead, Region, TimeRuler, TimelineTrack, TrackControls, TrackHeader,
};
use crate::theme::Theme;
use egui::{Color32, Rect, Response, ScrollArea, Sense, Ui, Vec2};

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
pub struct Timeline {
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

impl Timeline {
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
    fn get_track_by_path_mut<'a>(tracks: &'a mut [Track], path: &[usize]) -> Option<&'a mut Track> {
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

        // Apply scroll_to_beat if requested
        if let Some(beat) = self.scroll_to_beat {
            scroll_offset.x = beat * self.beat_width;
        }

        let vertical_response = ui.vertical(|ui| {
            // Calculate the full available width before Grid constrains it
            let full_width = ui.available_width();
            let timeline_width = full_width - self.track_header_width; // Width for ruler/tracks column

            // Use Grid for perfect alignment between ruler row and tracks row
            egui::Grid::new(self.id.unwrap_or_else(|| ui.id()).with("timeline_grid"))
                .spacing([0.0, ui.spacing().item_spacing.y]) // Use theme's natural item spacing
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

                    ruler_rect = ruler_response.rect;
                    ui.end_row(); // End first grid row

                    // Build flattened track list (includes hierarchy info and path to each track)
                    let mut flat_list = Vec::new();
                    Self::build_flat_track_list(tracks, Vec::new(), 0, 0, None, &mut flat_list);

                    // Row 2: Track Headers + Timeline Tracks
                    // Left cell: Track headers (wrapped in fixed-width container)
                    let (header_rects, header_viewport) = ui
                        .vertical(|ui| {
                            ui.set_width(self.track_header_width); // Force exact width to match row 1

                            let headers_scroll_id =
                                self.id.unwrap_or_else(|| ui.id()).with("headers_scroll");
                            let scroll_response = ScrollArea::vertical()
                                .id_salt(headers_scroll_id)
                                .scroll_offset(Vec2::new(0.0, scroll_offset.y))
                                .scroll_bar_visibility(
                                    egui::scroll_area::ScrollBarVisibility::AlwaysHidden,
                                )
                                .show(ui, |ui| {
                                    ui.vertical(|ui| {
                                        // Use default item_spacing for automatic consistent spacing
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
                                    })
                                    .inner
                                });
                            (scroll_response.inner, scroll_response.inner_rect)
                        })
                        .inner;

                    // Right cell: Timeline tracks
                    let (timeline_rects, timeline_viewport) = ui
                        .vertical(|ui| {
                            ui.set_width(timeline_width); // Match ruler width

                            let tracks_scroll_id =
                                self.id.unwrap_or_else(|| ui.id()).with("tracks_scroll");
                            let tracks_response = ScrollArea::both()
                                .id_salt(tracks_scroll_id)
                                .scroll_offset(scroll_offset)
                                .show(ui, |ui| {
                                    ui.vertical(|ui| {
                                        // Use default item_spacing for automatic consistent spacing
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
                                    })
                                    .inner
                                });

                            tracks_rect = tracks_response.inner_rect;
                            scroll_offset = tracks_response.state.offset;
                            (tracks_response.inner, tracks_response.inner_rect)
                        })
                        .inner;

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
                        let combined_rect = egui::Rect::from_min_max(
                            header_rect.min,
                            egui::Pos2::new(
                                timeline_rect.max.x,
                                timeline_rect.max.y.max(header_rect.max.y),
                            ),
                        );

                        painter.rect_stroke(
                            combined_rect,
                            4.0,
                            egui::Stroke::new(1.0, theme.outline()),
                            egui::StrokeKind::Inside,
                        );
                    }
                });

            scroll_offset
        });

        // Clip tracks_rect to the actual visible viewport
        tracks_rect = tracks_rect.intersect(vertical_response.response.rect);

        // Persist the shared scroll offset for next frame
        ui.ctx().data_mut(|d| {
            d.insert_persisted(scroll_id, scroll_offset);
        });

        // Render playhead overlay if enabled
        if self.show_playhead && !ruler_rect.is_negative() && !tracks_rect.is_negative() {
            // Create a combined rect spanning from ruler to end of tracks
            let playhead_rect = Rect::from_min_max(ruler_rect.min, tracks_rect.max);

            // Calculate total height (ruler + spacing + tracks)
            let total_height = playhead_rect.height();

            let playhead_id = self.id.unwrap_or_else(|| ui.id()).with("playhead");
            let mut playhead = Playhead::new(self.beat_width, total_height).id(playhead_id);

            if let Some(color) = self.playhead_color {
                playhead = playhead.color(color);
            }

            let playhead_response =
                playhead.show_in_rect(ui, playhead_rect, playhead_position, theme);

            if playhead_response.changed() {
                playhead_moved = true;
            }
        }

        // Create response
        let response = ui.allocate_response(Vec2::ZERO, Sense::hover());

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

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}
