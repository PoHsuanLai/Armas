#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use armas::components::audio::{AutomationEditor, CanvasConfig};
use audio_automation::{AutomationEnvelope, AutomationPoint, CurveType};
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "AutomationEditor Demo",
        options,
        Box::new(|_cc| Ok(Box::new(AutomationEditorDemo::default()))),
    )
}

struct AutomationEditorDemo {
    envelope: AutomationEnvelope<CurveType>,
    playhead_position: f64,
    selected_point: Option<usize>,
    config: CanvasConfig,
}

impl Default for AutomationEditorDemo {
    fn default() -> Self {
        let mut envelope = AutomationEnvelope::new(CurveType::Linear);

        // Add some initial points
        envelope.add_point(AutomationPoint::new(0.0, 0.0));
        envelope.add_point(AutomationPoint::new(2.0, 0.3));
        envelope.add_point(AutomationPoint::new(4.0, 0.8));
        envelope.add_point(AutomationPoint::new(6.0, 0.5));
        envelope.add_point(AutomationPoint::new(8.0, 1.0));

        Self {
            envelope,
            playhead_position: 0.0,
            selected_point: None,
            config: CanvasConfig {
                pixels_per_beat: 60.0,
                pixels_per_value: 40.0,
                min_value: 0.0,
                max_value: 1.0,
                grid_subdivisions: 4,
                snap_enabled: true,
                snap_interval: 0.25,
            },
        }
    }
}

impl eframe::App for AutomationEditorDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎚️ Automation Editor Demo");
            ui.separator();

            // Controls
            ui.horizontal(|ui| {
                ui.label("Playhead:");
                let mut playhead = self.playhead_position as f32;
                if ui
                    .add(egui::Slider::new(&mut playhead, 0.0..=10.0))
                    .changed()
                {
                    self.playhead_position = playhead as f64;
                }
            });

            ui.horizontal(|ui| {
                ui.label(format!("Total Points: {}", self.envelope.points.len()));
            });

            ui.horizontal(|ui| {
                if ui.button("Clear Points").clicked() {
                    self.envelope.points.clear();
                }
                if ui.button("Reset").clicked() {
                    *self = Self::default();
                }
            });

            ui.separator();

            // Main editor
            let response = AutomationEditor::new(&mut self.envelope)
                .canvas_size(egui::Vec2::new(800.0, 300.0))
                .canvas_config(self.config.clone())
                .playhead(self.playhead_position)
                .selected_point(self.selected_point)
                .point_color(egui::Color32::from_rgb(100, 200, 255))
                .show_values(true)
                .show(ui);

            // Handle responses
            if response.changed {
                if let Some(idx) = response.selected_point {
                    self.selected_point = Some(idx);
                }

                if let Some((idx, point)) = response.point_edited {
                    println!(
                        "Point {} edited: time={:.2}, value={:.2}",
                        idx, point.time, point.value
                    );
                }
            }

            ui.separator();

            // Info panel
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Points:");
                    for (idx, point) in self.envelope.points.iter().enumerate() {
                        let is_selected = self.selected_point == Some(idx);
                        let text = if is_selected {
                            format!(
                                "► Point {}: time={:.2}, value={:.2}",
                                idx, point.time, point.value
                            )
                        } else {
                            format!(
                                "  Point {}: time={:.2}, value={:.2}",
                                idx, point.time, point.value
                            )
                        };
                        ui.label(text);
                    }
                });

                ui.vertical(|ui| {
                    ui.label("Canvas Config:");
                    ui.label(format!("Pixels/beat: {}", self.config.pixels_per_beat));
                    ui.label(format!("Pixels/value: {}", self.config.pixels_per_value));
                    ui.label(format!("Min value: {}", self.config.min_value));
                    ui.label(format!("Max value: {}", self.config.max_value));
                    ui.label(format!("Snap enabled: {}", self.config.snap_enabled));
                    ui.label(format!("Snap interval: {}", self.config.snap_interval));
                });
            });

            // Auto-advance playhead for demo
            self.playhead_position += 0.005;
            if self.playhead_position > 10.0 {
                self.playhead_position = 0.0;
            }

            ctx.request_repaint();
        });
    }
}
