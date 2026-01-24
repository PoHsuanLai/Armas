//! Playback parameters component for sampler
//!
//! Controls for sample playback: pitch, pan, volume, and loop settings.

use armas::theme::Theme;
use armas::ext::ArmasContextExt;
use armas::components::cards::{Card, CardVariant};
use armas::components::basic::Slider;
use egui::Ui;

/// Playback parameters for a sample
#[derive(Debug, Clone, Copy)]
pub struct PlaybackParams {
    /// Pitch adjustment in semitones (-24 to +24)
    pub pitch: f32,
    /// Pan (-1.0 = left, 1.0 = right)
    pub pan: f32,
    /// Volume (0.0 to 1.0)
    pub volume: f32,
    /// Loop enabled
    pub loop_enabled: bool,
    /// Loop length in seconds
    pub loop_length: f64,
}

impl Default for PlaybackParams {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            pan: 0.0,
            volume: 0.8,
            loop_enabled: false,
            loop_length: 1.0,
        }
    }
}

/// Response from playback parameters UI
#[derive(Debug, Clone)]
pub struct PlaybackParamsResponse {
    pub pitch_changed: bool,
    pub pan_changed: bool,
    pub volume_changed: bool,
    pub loop_enabled_changed: bool,
    pub loop_length_changed: bool,
    pub params: PlaybackParams,
}

/// Playback parameters UI component
pub struct PlaybackParamsUI<'a> {
    params: &'a mut PlaybackParams,
    theme: &'a Theme,
    show_loop_controls: bool,
}

impl<'a> PlaybackParamsUI<'a> {
    /// Create a new playback parameters UI
    pub fn new(params: &'a mut PlaybackParams, theme: &'a Theme) -> Self {
        Self {
            params,
            theme,
            show_loop_controls: true,
        }
    }

    /// Hide/show loop controls
    pub fn show_loop_controls(mut self, show: bool) -> Self {
        self.show_loop_controls = show;
        self
    }

    /// Show the playback parameters UI
    pub fn show(self, ui: &mut Ui) -> PlaybackParamsResponse {
        let theme = ui.ctx().armas_theme();

        let mut response = PlaybackParamsResponse {
            pitch_changed: false,
            pan_changed: false,
            volume_changed: false,
            loop_enabled_changed: false,
            loop_length_changed: false,
            params: *self.params,
        };

        ui.spacing_mut().item_spacing.y = self.theme.spacing.lg;

        // Use card component with rounded corners for joyful feel
        Card::new()
            .variant(CardVariant::Filled)
            .corner_radius(12.0)
            .show(ui, &theme, |ui| {
                ui.spacing_mut().item_spacing.y = self.theme.spacing.md;

                // Subtle section label with visual hierarchy
                ui.label(
                    egui::RichText::new("Playback Controls")
                        .size(13.0)
                        .color(theme.muted_foreground()),
                );

                // Two-column layout for parameters - more spacious
                ui.spacing_mut().item_spacing.x = self.theme.spacing.xl;
                ui.horizontal(|ui| {
                    // Left column: Pitch & Pan
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = self.theme.spacing.md;

                        // Pitch slider with better label styling
                        let pitch_response = Slider::new(-24.0, 24.0)
                            .label("Pitch")
                            .suffix("st")
                            .show_value(true)
                            .show(ui, &mut self.params.pitch);
                        if pitch_response.changed {
                            response.pitch_changed = true;
                        }

                        // Visual spacing
                        ui.add_space(self.theme.spacing.sm);

                        // Pan slider
                        let pan_response = Slider::new(-1.0, 1.0)
                            .label("Pan")
                            .show_value(true)
                            .show(ui, &mut self.params.pan);
                        if pan_response.changed {
                            response.pan_changed = true;
                        }
                    });

                    // Right column: Volume & Loop
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = self.theme.spacing.md;

                        // Volume slider
                        let volume_response = Slider::new(0.0, 1.0)
                            .label("Volume")
                            .show_value(true)
                            .show(ui, &mut self.params.volume);
                        if volume_response.changed {
                            response.volume_changed = true;
                        }

                        // Visual spacing
                        ui.add_space(self.theme.spacing.sm);

                        // Loop toggle with better styling
                        if self.show_loop_controls {
                            if ui
                                .checkbox(&mut self.params.loop_enabled, "Loop")
                                .changed()
                            {
                                response.loop_enabled_changed = true;
                            }
                        }
                    });
                });

                // Loop length control (if enabled) - separate card section
                if self.show_loop_controls && self.params.loop_enabled {
                    ui.add_space(self.theme.spacing.md);
                    ui.separator();
                    ui.add_space(self.theme.spacing.md);

                    let mut loop_length_f32 = self.params.loop_length as f32;
                    let loop_response = Slider::new(0.1, 60.0)
                        .label("Loop Length")
                        .suffix("s")
                        .show_value(true)
                        .show(ui, &mut loop_length_f32);
                    if loop_response.changed {
                        self.params.loop_length = loop_length_f32 as f64;
                        response.loop_length_changed = true;
                    }
                }
            });

        response.params = *self.params;
        response
    }
}
