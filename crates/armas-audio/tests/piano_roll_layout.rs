//! Layout regression test for `PianoRoll`.
//!
//! Ensures the vertical piano strip does not overlap the first beat column of
//! the grid. Regression for a bug where `Piano`'s per-key `allocate_rect` calls
//! pulled the horizontal cursor backward, making the grid start inside the
//! piano area.

use armas_audio::{Piano, PianoOrientation, PianoRoll};
use armas_basic::ArmasContextExt;
use egui_kittest::Harness;
use std::sync::{Arc, Mutex};

#[test]
fn vertical_piano_does_not_overlap_following_widget() {
    let piano_rect: Arc<Mutex<Option<egui::Rect>>> = Arc::new(Mutex::new(None));
    let grid_rect: Arc<Mutex<Option<egui::Rect>>> = Arc::new(Mutex::new(None));
    let piano_cap = piano_rect.clone();
    let grid_cap = grid_rect.clone();

    let mut harness = Harness::new_ui(move |ui| {
        ui.set_min_size(egui::vec2(1200.0, 700.0));
        let theme = ui.ctx().armas_theme();

        ui.horizontal(|ui| {
            let p = Piano::new()
                .start_note(60)
                .octaves(2)
                .white_key_width(40.0)
                .white_key_height(120.0)
                .orientation(PianoOrientation::Vertical)
                .show(ui, &theme);
            *piano_cap.lock().unwrap() = Some(p.response.rect);

            let (rect, _) = ui.allocate_exact_size(egui::vec2(800.0, 560.0), egui::Sense::hover());
            *grid_cap.lock().unwrap() = Some(rect);
        });
    });

    harness.run();
    harness.run();

    let p = piano_rect.lock().unwrap().expect("piano rect captured");
    let g = grid_rect.lock().unwrap().expect("grid rect captured");

    assert!(
        g.min.x >= p.max.x,
        "piano.max.x={:.2} overlaps grid.min.x={:.2}",
        p.max.x,
        g.min.x,
    );
}

#[test]
fn piano_roll_renders_without_panicking() {
    let mut harness = Harness::new_ui(|ui| {
        ui.set_min_size(egui::vec2(1200.0, 700.0));
        let theme = ui.ctx().armas_theme();
        PianoRoll::new().show(ui, &theme);
    });
    harness.run();
}

#[test]
fn clicking_piano_key_does_not_add_note() {
    use armas_audio::Note;

    let initial_for_run = vec![Note::new(60, 0.0, 1.0)];
    let captured_notes: Arc<Mutex<Vec<Note>>> = Arc::new(Mutex::new(initial_for_run.clone()));
    let captured_notes_clone = captured_notes.clone();

    let mut harness = Harness::new_ui(move |ui| {
        ui.set_min_size(egui::vec2(1200.0, 700.0));
        let theme = ui.ctx().armas_theme();
        let current = captured_notes_clone.lock().unwrap().clone();
        let resp = PianoRoll::new()
            .id("piano_test")
            .notes(current)
            .show(ui, &theme);
        *captured_notes_clone.lock().unwrap() = resp.notes;
    });

    harness.run();

    // The vertical piano sits at the very top-left of the harness ui, occupying
    // roughly x=[8, 128] with white_key_height=120. Pick a point well inside the
    // piano (a white key in the middle of the keyboard).
    let click_pos = egui::pos2(40.0, 300.0);
    harness.hover_at(click_pos);
    harness.run();
    harness.drag_at(click_pos);
    harness.run();
    harness.drop_at(click_pos);
    harness.run();
    harness.run();

    let after = captured_notes.lock().unwrap().clone();
    assert_eq!(
        after, initial_for_run,
        "Clicking a piano key must not add or remove notes (got: {after:?})"
    );
}
