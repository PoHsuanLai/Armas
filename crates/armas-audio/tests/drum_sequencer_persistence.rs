//! Verify `DrumSequencer` persists step toggles across frames when given an id.

use armas_audio::{DrumRow, DrumSequencer};
use armas_basic::ArmasContextExt;
use egui_kittest::Harness;
use std::sync::{Arc, Mutex};

#[test]
fn step_toggle_survives_rebuilt_rows_with_id() {
    let last_rows: Arc<Mutex<Option<Vec<DrumRow>>>> = Arc::new(Mutex::new(None));
    let last_rows_cap = last_rows.clone();

    let mut harness = Harness::new_ui(move |ui| {
        ui.set_min_size(egui::vec2(1200.0, 700.0));
        let theme = ui.ctx().armas_theme();

        // Same pattern as the demo: rows freshly built every frame.
        let mut rows = vec![DrumRow::new("Kick", 16), DrumRow::new("Snare", 16)];
        DrumSequencer::new(&mut rows)
            .steps(16)
            .id("persistence_test")
            .show(ui, &theme);

        *last_rows_cap.lock().unwrap() = Some(rows);
    });

    harness.run();

    // Inject a state directly: writing to ctx memory simulates a user click,
    // because that's exactly what the component does on toggle.
    let state_key = egui::Id::new("persistence_test").with("drum_sequencer_state");
    harness.ctx.data_mut(|d| {
        let mut steps_kick = vec![armas_audio::DrumStep::default(); 16];
        steps_kick[0].active = true;
        steps_kick[4].active = true;
        let steps_snare = vec![armas_audio::DrumStep::default(); 16];
        d.insert_temp::<Vec<Vec<armas_audio::DrumStep>>>(state_key, vec![steps_kick, steps_snare]);
    });

    harness.run();
    harness.run();

    let captured = last_rows.lock().unwrap().clone().expect("rows captured");
    assert!(
        captured[0].steps[0].active,
        "kick step 0 should have been restored from persisted state"
    );
    assert!(
        captured[0].steps[4].active,
        "kick step 4 should have been restored from persisted state"
    );
    assert!(
        !captured[1].steps[0].active,
        "snare step 0 was not toggled; should remain inactive"
    );
}

#[test]
fn clicking_a_step_toggles_and_persists_it() {
    let last_rows: Arc<Mutex<Option<Vec<DrumRow>>>> = Arc::new(Mutex::new(None));
    let last_rows_cap = last_rows.clone();
    let outer_rect: Arc<Mutex<Option<egui::Rect>>> = Arc::new(Mutex::new(None));
    let outer_cap = outer_rect.clone();

    let mut harness = Harness::new_ui(move |ui| {
        ui.set_min_size(egui::vec2(1200.0, 700.0));
        let theme = ui.ctx().armas_theme();
        let mut rows = vec![DrumRow::new("Kick", 16)];
        let resp = DrumSequencer::new(&mut rows)
            .steps(16)
            .id("click_test")
            .show(ui, &theme);
        *outer_cap.lock().unwrap() = Some(resp.response.rect);
        *last_rows_cap.lock().unwrap() = Some(rows);
    });

    harness.run();

    // Center of step 0: row_label_width=80, step_width=40, row_height=48.
    let r = outer_rect.lock().unwrap().expect("outer rect captured");
    let click_pos = egui::pos2(r.min.x + 80.0 + 20.0, r.min.y + 24.0);

    harness.hover_at(click_pos);
    harness.run();
    harness.drag_at(click_pos);
    harness.run();
    harness.drop_at(click_pos);
    harness.run();
    harness.run();

    let captured = last_rows.lock().unwrap().clone().expect("rows captured");
    let any_active = captured[0].steps.iter().any(|s| s.active);
    assert!(
        any_active,
        "After clicking on the kick row, at least one step should be active. Got: {:?}",
        captured[0]
            .steps
            .iter()
            .map(|s| s.active)
            .collect::<Vec<_>>()
    );
}
