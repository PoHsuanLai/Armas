#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use armas::components::audio::{
    ADSREnvelope, PlaybackParams, SampleZone, WaveformConfig,
};
use armas::ext::ArmasContextExt;
use eframe::egui;
use egui::Color32;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 750.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Sampler Demo",
        options,
        Box::new(|_cc| Ok(Box::new(SamplerDemo::default()))),
    )
}

struct SamplerZoneData {
    name: String,
    samples: Vec<f32>,
    duration: f64,
    playback_params: PlaybackParams,
    envelope: ADSREnvelope,
}

struct SamplerDemo {
    zones: Vec<SamplerZoneData>,
    selected_zone: usize,
}

impl Default for SamplerDemo {
    fn default() -> Self {
        Self {
            zones: vec![
                SamplerZoneData {
                    name: "Piano".to_string(),
                    samples: generate_sample(2.0, 440.0),
                    duration: 2.0,
                    playback_params: PlaybackParams::default(),
                    envelope: ADSREnvelope::default(),
                },
                SamplerZoneData {
                    name: "Kick".to_string(),
                    samples: generate_sample(0.5, 60.0),
                    duration: 0.5,
                    playback_params: PlaybackParams {
                        volume: 0.9,
                        ..Default::default()
                    },
                    envelope: ADSREnvelope {
                        attack: 0.01,
                        decay: 0.4,
                        sustain: 0.0,
                        release: 0.09,
                        ..Default::default()
                    },
                },
                SamplerZoneData {
                    name: "Snare".to_string(),
                    samples: generate_sample(0.3, 200.0),
                    duration: 0.3,
                    playback_params: PlaybackParams {
                        volume: 0.8,
                        ..Default::default()
                    },
                    envelope: ADSREnvelope {
                        attack: 0.005,
                        decay: 0.25,
                        sustain: 0.1,
                        release: 0.05,
                        ..Default::default()
                    },
                },
            ],
            selected_zone: 0,
        }
    }
}

impl eframe::App for SamplerDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Central panel for Armas components only
        egui::CentralPanel::default()
            .frame(egui::Frame {
                fill: Color32::from_rgb(20, 20, 30),
                ..Default::default()
            })
            .show(ctx, |ui| {
                // Zone selector
                ui.horizontal(|ui| {
                    for (idx, zone) in self.zones.iter().enumerate() {
                        if ui.selectable_label(idx == self.selected_zone, &zone.name).clicked() {
                            self.selected_zone = idx;
                        }
                    }
                });

                ui.separator();

                // Make the central panel scrollable
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                let theme = ui.ctx().armas_theme();
                let zone = &mut self.zones[self.selected_zone];

                // Waveform config
                let config = WaveformConfig {
                    pixels_per_second: 200.0,
                    height: 180.0,
                    peak_color: Color32::from_rgb(100, 200, 255),
                    rms_color: Color32::from_rgb(100, 200, 255).gamma_multiply(0.5),
                    show_grid: false,
                    grid_interval: 0.25,
                    color_by_frequency: false,
                    show_spectrogram: false,
                    zoom_level: 1.0,
                };

                // Calculate responsive waveform size
                let available_width = ui.available_width() - theme.spacing.lg;
                let waveform_width = available_width.max(400.0);
                let waveform_height = 180.0;

                // Sample zone with all controls
                let _response = SampleZone::new(
                    zone.name.clone(),
                    &zone.samples,
                    |sample| sample.abs(),
                    zone.duration,
                    44100,
                    &mut zone.playback_params,
                    &mut zone.envelope,
                    &theme,
                )
                .waveform_config(config)
                .waveform_size(egui::Vec2::new(waveform_width, waveform_height))
                .show(ui);
                    });
            });
    }
}

/// Generate a simple sine wave sample
fn generate_sample(duration: f64, frequency: f32) -> Vec<f32> {
    let sample_rate = 44100;
    let num_samples = (duration * sample_rate as f64) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;

        // Complex waveform with multiple harmonics (like a real instrument)
        let fundamental = 2.0 * std::f32::consts::PI * frequency * t;

        // Harmonic series with diminishing amplitudes
        let harmonic1 = fundamental.sin() * 0.5; // Fundamental
        let harmonic2 = (2.0 * fundamental).sin() * 0.3; // 2nd harmonic
        let harmonic3 = (3.0 * fundamental).sin() * 0.15; // 3rd harmonic
        let harmonic4 = (4.0 * fundamental).sin() * 0.1; // 4th harmonic
        let harmonic5 = (5.0 * fundamental).sin() * 0.05; // 5th harmonic

        // Add some noise-like component for realism
        let noise = ((t * 1000.0).sin() * (t * 731.0).cos()) * 0.08;

        // Exponential decay envelope
        let decay = (-1.5 * t).exp();

        // Combine all components
        let combined = harmonic1 + harmonic2 + harmonic3 + harmonic4 + harmonic5 + noise;
        let sample = combined * decay * 0.6;

        samples.push(sample.clamp(-1.0, 1.0));
    }

    samples
}
