# Timeline

DAW-style horizontal time ruler with playhead, regions, markers, and snap grid.

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Drums", egui::Color32::from_rgb(255, 100, 100)).region(Region::new("Kick", 0.0, 4.0)),
    Track::new("Bass", egui::Color32::from_rgb(100, 255, 100)).region(Region::new("Bassline", 0.0, 4.0)),
];
let mut playhead_pos = 0.0;
Timeline::new().id(ui.id().with("timeline")).track_header_width(150.0).track_height(60.0).beat_width(50.0).measures(4).show(ui, &mut tracks, &mut playhead_pos, &theme);
```

## With Markers

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Vocals", egui::Color32::from_rgb(255, 100, 100))
        .region(Region::new("Verse", 0.0, 2.0))
        .region(Region::new("Chorus", 3.0, 2.0)),
];
let mut playhead_pos = 0.0;
let mut markers = vec![
    MarkerData::new(0.0, "Intro"),
    MarkerData::tempo(1.0, 120.0),
    MarkerData::new(2.0, "Verse"),
    MarkerData::new(3.0, "Chorus"),
];
let mut loop_region = LoopRegionData::new(2.0, 4.0);
Timeline::new().id(ui.id().with("markers")).markers(&mut markers).loop_region(&mut loop_region).beat_width(50.0).measures(4).show(ui, &mut tracks, &mut playhead_pos, &theme);
```

## All Marker Types

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![Track::new("Track 1", egui::Color32::from_rgb(100, 180, 255)).region(Region::new("Clip", 0.0, 8.0))];
let mut playhead_pos = 0.0;
let mut markers = vec![
    MarkerData::new(0.0, "Intro"),
    MarkerData::tempo(0.0, 120.0),
    MarkerData::time_signature(0.0, 4, 4),
    MarkerData::new(4.0, "Verse"),
    MarkerData::tempo(4.0, 140.0),
];
let mut loop_region = LoopRegionData::new(2.0, 6.0);
let mut selection_range = SelectionRangeData::new(4.0, 8.0);
let mut punch_region = PunchRegionData::new(3.0, 7.0);
Timeline::new().id(ui.id().with("all_markers")).markers(&mut markers).loop_region(&mut loop_region).selection_range(&mut selection_range).punch_region(&mut punch_region).beat_width(40.0).measures(8).show(ui, &mut tracks, &mut playhead_pos, &theme);
```

## Snap Grid

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![Track::new("Track 1", egui::Color32::from_rgb(255, 150, 100)).region(Region::new("Clip", 0.0, 4.0))];
let mut playhead_pos = 0.0;
Timeline::new().id(ui.id().with("snap_grid")).show_snap_grid(true).snap_grid_subdivision(4).beat_width(60.0).measures(4).show(ui, &mut tracks, &mut playhead_pos, &theme);
```

## Folder Tracks

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new_folder("Vocals", egui::Color32::from_rgb(255, 100, 100))
        .child(Track::new("Lead", egui::Color32::from_rgb(255, 120, 120)).region(Region::new("Verse 1", 0.0, 4.0)))
        .child(Track::new("Backing", egui::Color32::from_rgb(255, 140, 140)).region(Region::new("Harmonies", 4.0, 4.0))),
    Track::new("Bass", egui::Color32::from_rgb(100, 150, 255)).region(Region::new("Bassline", 0.0, 8.0)),
];
let mut playhead_pos = 0.0;
Timeline::new().id(ui.id().with("folders")).track_header_width(160.0).beat_width(45.0).measures(8).show(ui, &mut tracks, &mut playhead_pos, &theme);
```

## Zoom Control

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![Track::new("Track 1", egui::Color32::from_rgb(100, 180, 255)).region(Region::new("Clip", 0.0, 4.0))];
let mut playhead_pos = 0.0;
let mut zoom = 1.0;
ZoomControl::new(&mut zoom).id("zoom").min_zoom(0.5).max_zoom(2.0).show(ui, &theme);
ui.add_space(8.0);
let zoomed_beat_width = 50.0 * zoom;
Timeline::new().id(ui.id().with("zoom")).beat_width(zoomed_beat_width).measures(4).show(ui, &mut tracks, &mut playhead_pos, &theme);
```

## Scrolling

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Track 1", egui::Color32::from_rgb(255, 100, 100))
        .region(Region::new("Clip A", 0.0, 2.0))
        .region(Region::new("Clip B", 8.0, 2.0)),
];
let mut playhead_pos = ui.ctx().data_mut(|d| d.get_persisted(egui::Id::new("playhead")).unwrap_or(10.0));
ui.horizontal(|ui| {
    if ui.button("Jump to Start").clicked() { playhead_pos = 0.0; }
    if ui.button("Jump to Beat 8").clicked() { playhead_pos = 8.0; }
});
ui.add_space(8.0);
Timeline::new().id(ui.id().with("scroll")).beat_width(50.0).measures(16).scroll_to_beat(playhead_pos).show(ui, &mut tracks, &mut playhead_pos, &theme);
ui.ctx().data_mut(|d| d.insert_persisted(egui::Id::new("playhead"), playhead_pos));
```

## Interactions

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Track 1", egui::Color32::from_rgb(255, 150, 100)).region(Region::new("Clip A", 0.0, 2.0)),
    Track::new("Track 2", egui::Color32::from_rgb(100, 200, 255)).region(Region::new("Clip B", 1.0, 3.0)),
];
let mut playhead_pos = 0.0;
let last_id = ui.id().with("last_interaction");
let mut last_interaction: String = ui.ctx().data_mut(|d| d.get_persisted(last_id).unwrap_or_default());
let response = Timeline::new().id(ui.id().with("interactions")).beat_width(50.0).measures(4).show(ui, &mut tracks, &mut playhead_pos, &theme);
if let Some(idx) = response.track_clicked {
    last_interaction = format!("Track clicked: {}", tracks[idx].name);
}
if let Some(idx) = response.track_mute_clicked {
    last_interaction = format!("Mute toggled: {}", tracks[idx].name);
}
if let Some((track_idx, region_idx)) = response.region_clicked {
    last_interaction = format!("Region clicked: {}", tracks[track_idx].regions[region_idx].name);
}
if response.playhead_moved {
    last_interaction = format!("Playhead: {:.2}", playhead_pos);
}
ui.ctx().data_mut(|d| d.insert_persisted(last_id, last_interaction.clone()));
ui.add_space(8.0);
if !last_interaction.is_empty() {
    ui.label(format!("Last: {}", last_interaction));
} else {
    ui.colored_label(egui::Color32::GRAY, "Last: None yet");
}
```

## Clip Editing

```demo
let theme = ui.ctx().armas_theme();
let s_curve = FadeSettings::new(0.4, 0.4).fade_in_curve(FadeCurve::SCurve).fade_out_curve(FadeCurve::SCurve);
let mut tracks = vec![
    Track::new("Audio", egui::Color32::from_rgb(100, 180, 255))
        .regions(vec![
            Region::new("With Fades", 0.0, 3.0).fade_in(0.5).fade_out(0.5).selected(true),
            Region::new("Boosted", 4.0, 2.0).gain_db(6.0).selected(true),
        ]),
    Track::new("MIDI", egui::Color32::from_rgb(255, 150, 100))
        .region(Region::midi("Piano", 0.0, 6.0).fades(s_curve).selected(true)),
];
let mut playhead_pos = 0.0;
Timeline::new().id(ui.id().with("clip_editing")).track_height(80.0).beat_width(60.0).measures(8).show(ui, &mut tracks, &mut playhead_pos, &theme);
```

## Complete Example

```demo
let theme = ui.ctx().armas_theme();
let mut tracks = vec![
    Track::new("Lead Vocals", egui::Color32::from_rgb(255, 100, 100))
        .regions(vec![
            Region::new("Verse 1", 0.0, 4.0),
            Region::new("Chorus", 4.0, 3.0),
            Region::new("Verse 2", 8.0, 4.0),
        ]),
    Track::new("Guitar", egui::Color32::from_rgb(255, 200, 50)).region(Region::new("Strumming", 0.0, 8.0)),
    Track::new("Bass", egui::Color32::from_rgb(100, 150, 255)).region(Region::new("Bassline", 0.0, 8.0)),
];
let mut playhead_pos = 0.0;
let mut markers = vec![
    MarkerData::new(0.0, "Verse 1"),
    MarkerData::new(4.0, "Chorus"),
];
let mut loop_region = LoopRegionData::new(4.0, 7.0);
Timeline::new().id(ui.id().with("complete")).markers(&mut markers).loop_region(&mut loop_region).show_snap_grid(true).track_header_width(140.0).track_height(50.0).beat_width(45.0).measures(8).show(ui, &mut tracks, &mut playhead_pos, &theme);
```
