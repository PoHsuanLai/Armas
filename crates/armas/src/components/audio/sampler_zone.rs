//! Sample zone component for sampler
//!
//! Combines waveform display, playback parameters, and envelope controls for a single sample zone.

use crate::components::audio::{
    ADSREnvelope, Fader, Knob, PlaybackParams, WaveformConfig, WaveformDisplay,
};
use crate::components::cards::GlassPanel;
use crate::ext::ArmasContextExt;
use crate::theme::Theme;
use egui::{Color32, Ui, Vec2};

// Knob size constants matching Studio One
const KNOB_BIG: f32 = 48.0;
const KNOB_MEDIUM: f32 = 32.0;
const KNOB_SMALL: f32 = 24.0;  // Slightly taller to fill space
const CONTROL_SPACING: f32 = 3.0;
const FADER_WIDTH: f32 = 16.0;
const FADER_HEIGHT: f32 = 110.0;  // Height of 2 big knobs + spacing (48*2 + 14 = 110)
const LABEL_SIZE: f32 = 8.0;
const VALUE_SIZE: f32 = 7.0;
const CARD_WIDTH: f32 = 350.0;  // Fixed width for all control cards
const CARD_HEIGHT: f32 = 180.0; // Fixed height for all control cards

/// A single sample zone with all editing controls
pub struct SampleZone<'a, T> {
    /// Zone identifier
    pub name: String,
    /// Sample data
    pub sample_data: &'a [T],
    /// Amplitude function
    pub amplitude_fn: fn(&T) -> f32,
    /// Sample duration in seconds
    pub duration: f64,
    /// Sample rate
    pub sample_rate: u32,
    /// Playback parameters
    pub playback_params: &'a mut PlaybackParams,
    /// ADSR envelope
    pub envelope: &'a mut ADSREnvelope,
    /// Theme
    pub theme: &'a Theme,

    // UI configuration
    pub waveform_config: WaveformConfig,
    pub waveform_size: Vec2,
}

/// Response from sample zone
#[derive(Debug, Clone)]
pub struct SampleZoneResponse {
    pub playback_params_changed: bool,
    pub envelope_changed: bool,
}

impl<'a, T> SampleZone<'a, T> {
    /// Create a new sample zone
    pub fn new(
        name: String,
        sample_data: &'a [T],
        amplitude_fn: fn(&T) -> f32,
        duration: f64,
        sample_rate: u32,
        playback_params: &'a mut PlaybackParams,
        envelope: &'a mut ADSREnvelope,
        theme: &'a Theme,
    ) -> Self {
        Self {
            name,
            sample_data,
            amplitude_fn,
            duration,
            sample_rate,
            playback_params,
            envelope,
            theme,
            waveform_config: WaveformConfig::default(),
            waveform_size: Vec2::new(800.0, 200.0),
        }
    }

    /// Set waveform display configuration
    pub fn waveform_config(mut self, config: WaveformConfig) -> Self {
        self.waveform_config = config;
        self
    }

    /// Set waveform display size
    pub fn waveform_size(mut self, size: Vec2) -> Self {
        self.waveform_size = size;
        self
    }

    /// Show the sample zone
    pub fn show(self, ui: &mut Ui) -> SampleZoneResponse {
        let theme = ui.ctx().armas_theme();
        let mut response = SampleZoneResponse {
            playback_params_changed: false,
            envelope_changed: false,
        };

        ui.spacing_mut().item_spacing.y = self.theme.spacing.sm;

        // Main container with glassmorphic styling
        GlassPanel::new()
            .opacity(0.6)
            .glow_intensity(0.4)
            .corner_radius(16.0)
            .show(ui, &theme, |ui| {
                ui.spacing_mut().item_spacing.y = self.theme.spacing.sm;

                // Zone title
                ui.heading(
                    egui::RichText::new(&self.name)
                        .size(14.0)
                        .color(theme.foreground()),
                );

                // ==================== WAVEFORM SECTION ====================
                // Apply pitch, volume, and pan transformations
                let processed_samples = apply_pitch_volume_and_pan(
                    self.sample_data,
                    self.amplitude_fn,
                    self.playback_params.pitch,
                    self.playback_params.volume,
                    self.playback_params.pan,
                );

                let processed_duration = calculate_pitch_adjusted_duration(
                    self.duration,
                    self.playback_params.pitch,
                );

                // Generate left and right channel waveforms
                let (left_channel, right_channel) = create_stereo_channels(
                    &processed_samples,
                    self.playback_params.pan,
                );

                // Calculate waveform width to match the control cards below
                // Using constants defined at the top of the file
                let card_padding = theme.spacing.sm * 2.0; // Left + right padding per card
                let card_spacing = theme.spacing.md;

                // PITCH: Big knob is 48, 2×2 grid is 2*20+3=43, so use 48
                let card1_width = KNOB_BIG + card_padding;
                // FILTER: Big knob is 48, 2×3 grid is 3*20+2*3=66, so use 66
                let card2_width = (3.0 * KNOB_SMALL + 2.0 * CONTROL_SPACING) + card_padding;
                // AMP: Big knob is 48, 2×2 grid is 2*20+3=43, so use 48
                let card3_width = KNOB_BIG + card_padding;
                // ENVELOPE: 4 faders
                let card4_width = 4.0 * FADER_WIDTH + 3.0 * CONTROL_SPACING + card_padding;
                // LFO: 2 medium knobs
                let card5_width = 2.0 * KNOB_MEDIUM + CONTROL_SPACING + card_padding;

                let waveform_width = card1_width + card2_width + card3_width + card4_width + card5_width + 4.0 * card_spacing;
                let waveform_height = self.waveform_size.y / 2.0; // Half height since we're stacking vertically

                // Display stereo waveforms vertically stacked in a card
                GlassPanel::new()
                    .opacity(0.5)
                    .glow_intensity(0.2)
                    .inner_margin(theme.spacing.sm)
                    .show(ui, &theme, |ui| {
                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = self.theme.spacing.xs;

                            // Left channel
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new("L")
                                        .size(10.0)
                                        .color(theme.muted_foreground()),
                                );
                                let _left_response = WaveformDisplay::new(
                                    &left_channel,
                                    |s| *s,
                                    processed_duration,
                                    self.sample_rate,
                                    self.theme,
                                )
                                .config(self.waveform_config.clone())
                                .sample_bounds(0.0, processed_duration)
                                .show(ui, Vec2::new(waveform_width, waveform_height));
                            });

                            // Right channel
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new("R")
                                        .size(10.0)
                                        .color(theme.muted_foreground()),
                                );
                                let _right_response = WaveformDisplay::new(
                                    &right_channel,
                                    |s| *s,
                                    processed_duration,
                                    self.sample_rate,
                                    self.theme,
                                )
                                .config(self.waveform_config.clone())
                                .sample_bounds(0.0, processed_duration)
                                .show(ui, Vec2::new(waveform_width, waveform_height));
                            });
                        });
                    });

                // ==================== CARD-BASED CONTROL GROUPS ====================
                // Color indicators for functional vs placeholder controls
                let functional_color = theme.primary(); // Green/accent for functional
                let placeholder_color = theme.muted_foreground(); // Dimmed for placeholders

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = theme.spacing.md;

                    // ━━━━━━ CARD 1: PITCH (with Envelope) ━━━━━━
                    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                        draw_pitch_card(ui, self.playback_params, self.envelope, self.theme, functional_color, placeholder_color, &mut response);
                    });

                    // ━━━━━━ CARD 2: FILTER (with Envelope) ━━━━━━
                    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                        draw_filter_card(ui, self.envelope, self.theme, functional_color, placeholder_color, &mut response);
                    });

                    // ━━━━━━ CARD 3: AMP (with Envelope) ━━━━━━
                    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                        draw_amp_card(ui, self.playback_params, self.envelope, self.theme, functional_color, placeholder_color, &mut response);
                    });

                    // ━━━━━━ CARD 4: LFO ━━━━━━
                    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                        draw_lfo_card(ui, self.theme, placeholder_color);
                    });
                });
            });

        response
    }
}

// ==================== Helper Functions for Card UI ====================

/// Draw compact ADSR faders
fn draw_adsr_faders(
    ui: &mut Ui,
    envelope: &mut ADSREnvelope,
    _theme: &Theme,
    functional_color: Color32,
    response: &mut SampleZoneResponse,
) {
    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        ui.spacing_mut().item_spacing.x = CONTROL_SPACING;

        // Attack
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.label(egui::RichText::new("A").size(LABEL_SIZE).strong().color(functional_color));
            let attack = envelope.attack / 5.0;
            let (_r, new_a) = Fader::new(attack).size(FADER_WIDTH, FADER_HEIGHT).show(ui);
            envelope.attack = new_a * 5.0;
            ui.label(egui::RichText::new(format!("{:.2}s", envelope.attack)).size(VALUE_SIZE));
            response.envelope_changed = true;
        });

        // Decay
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.label(egui::RichText::new("D").size(LABEL_SIZE).strong().color(functional_color));
            let decay = envelope.decay / 5.0;
            let (_r, new_d) = Fader::new(decay).size(FADER_WIDTH, FADER_HEIGHT).show(ui);
            envelope.decay = new_d * 5.0;
            ui.label(egui::RichText::new(format!("{:.2}s", envelope.decay)).size(VALUE_SIZE));
            response.envelope_changed = true;
        });

        // Sustain
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.label(egui::RichText::new("S").size(LABEL_SIZE).strong().color(functional_color));
            let sustain = envelope.sustain;
            let (_r, new_s) = Fader::new(sustain).size(FADER_WIDTH, FADER_HEIGHT).show(ui);
            envelope.sustain = new_s;
            ui.label(egui::RichText::new(format!("{:.2}", envelope.sustain)).size(VALUE_SIZE));
            response.envelope_changed = true;
        });

        // Release
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.label(egui::RichText::new("R").size(LABEL_SIZE).strong().color(functional_color));
            let release = envelope.release / 5.0;
            let (_r, new_r) = Fader::new(release).size(FADER_WIDTH, FADER_HEIGHT).show(ui);
            envelope.release = new_r * 5.0;
            ui.label(egui::RichText::new(format!("{:.2}s", envelope.release)).size(VALUE_SIZE));
            response.envelope_changed = true;
        });
    });
}

/// Draw a pitch card matching Studio One layout: 2 big knobs (left) + ADSR faders (center) + small knobs (right)
fn draw_pitch_card(
    ui: &mut Ui,
    playback_params: &mut PlaybackParams,
    envelope: &mut ADSREnvelope,
    theme: &Theme,
    functional_color: Color32,
    placeholder_color: Color32,
    response: &mut SampleZoneResponse,
) {
    let mut dummy_val = 0.5;

    GlassPanel::new()
        .title("PITCH")
        .opacity(0.5)
        .glow_intensity(0.3)
        .inner_margin(theme.spacing.sm)
        .width(CARD_WIDTH)
        .height(CARD_HEIGHT)
        .show(ui, theme, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = theme.spacing.md;

                // Big knobs (Transpose, Tune)
                ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = theme.spacing.sm;

                            // Transpose knob (✓ FUNCTIONAL)
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 2.0;
                                ui.label(egui::RichText::new("Transpose").size(LABEL_SIZE).strong().color(functional_color))
                                    .on_hover_text("Connected: Pitch shift in semitones");
                                let mut pitch_norm = (playback_params.pitch + 24.0) / 48.0;
                                let knob_resp = Knob::new(pitch_norm).diameter(KNOB_BIG).show(ui, &mut pitch_norm, theme);
                                playback_params.pitch = (knob_resp.value * 48.0) - 24.0;
                                ui.label(egui::RichText::new(format!("{:.0}st", playback_params.pitch)).size(VALUE_SIZE));
                                response.playback_params_changed = true;
                            });

                            // Tune knob (placeholder for fine tuning in cents)
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 2.0;
                                ui.label(egui::RichText::new("Tune").size(LABEL_SIZE).color(placeholder_color))
                                    .on_hover_text("Placeholder: Fine tune in cents");
                                let _ = Knob::new(dummy_val).diameter(KNOB_BIG).show(ui, &mut dummy_val, theme);
                                ui.label(egui::RichText::new("+0¢").size(VALUE_SIZE).color(placeholder_color));
                            });
                        });

                // ADSR Faders
                draw_adsr_faders(ui, envelope, theme, functional_color, response);

                // Right: Small knobs stacked vertically
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = CONTROL_SPACING;

                    // Env knob (placeholder)
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("Env").size(LABEL_SIZE).color(placeholder_color));
                        let _ = Knob::new(dummy_val).diameter(KNOB_SMALL).show(ui, &mut dummy_val, theme);
                        ui.label(egui::RichText::new("0").size(VALUE_SIZE).color(placeholder_color));
                    });

                    // LFO knob (placeholder)
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("LFO").size(LABEL_SIZE).color(placeholder_color));
                        let _ = Knob::new(dummy_val).diameter(KNOB_SMALL).show(ui, &mut dummy_val, theme);
                        ui.label(egui::RichText::new("0").size(VALUE_SIZE).color(placeholder_color));
                    });
                });
            });
        });
}

/// Draw a filter card matching Studio One layout: 2 big knobs (left) + ADSR faders (center) + small knobs (right)
fn draw_filter_card(
    ui: &mut Ui,
    envelope: &mut ADSREnvelope,
    theme: &Theme,
    functional_color: Color32,
    placeholder_color: Color32,
    response: &mut SampleZoneResponse,
) {
    let mut dummy_val = 0.5;

    GlassPanel::new()
        .title("FILTER")
        .opacity(0.5)
        .glow_intensity(0.3)
        .inner_margin(theme.spacing.sm)
        .width(CARD_WIDTH)
        .height(CARD_HEIGHT)
        .show(ui, theme, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = theme.spacing.md;

                // Big knobs (Cutoff, Res)
                ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = theme.spacing.sm;

                            // Cutoff knob (placeholder)
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 2.0;
                                ui.label(egui::RichText::new("Cutoff").size(LABEL_SIZE).color(placeholder_color))
                                    .on_hover_text("Placeholder: Filter cutoff frequency");
                                let _ = Knob::new(dummy_val).diameter(KNOB_BIG).show(ui, &mut dummy_val, theme);
                                ui.label(egui::RichText::new("20kHz").size(VALUE_SIZE).color(placeholder_color));
                            });

                            // Res knob (placeholder)
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 2.0;
                                ui.label(egui::RichText::new("Res").size(LABEL_SIZE).color(placeholder_color))
                                    .on_hover_text("Placeholder: Filter resonance");
                                let _ = Knob::new(dummy_val).diameter(KNOB_BIG).show(ui, &mut dummy_val, theme);
                                ui.label(egui::RichText::new("0%").size(VALUE_SIZE).color(placeholder_color));
                            });
                        });

                // ADSR Faders
                draw_adsr_faders(ui, envelope, theme, functional_color, response);

                // Right: Small knobs stacked vertically
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = CONTROL_SPACING;

                    // LFO knob
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("LFO").size(LABEL_SIZE).color(placeholder_color));
                        let _ = Knob::new(dummy_val).diameter(KNOB_SMALL).show(ui, &mut dummy_val, theme);
                        ui.label(egui::RichText::new("0").size(VALUE_SIZE).color(placeholder_color));
                    });

                    // Mod knob
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("Mod").size(LABEL_SIZE).color(placeholder_color));
                        let _ = Knob::new(dummy_val).diameter(KNOB_SMALL).show(ui, &mut dummy_val, theme);
                        ui.label(egui::RichText::new("0").size(VALUE_SIZE).color(placeholder_color));
                    });

                    // Env knob
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("Env").size(LABEL_SIZE).color(placeholder_color));
                        let _ = Knob::new(dummy_val).diameter(KNOB_SMALL).show(ui, &mut dummy_val, theme);
                        ui.label(egui::RichText::new("0").size(VALUE_SIZE).color(placeholder_color));
                    });

                    // Vol knob
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("Vol").size(LABEL_SIZE).color(placeholder_color));
                        let _ = Knob::new(dummy_val).diameter(KNOB_SMALL).show(ui, &mut dummy_val, theme);
                        ui.label(egui::RichText::new("100").size(VALUE_SIZE).color(placeholder_color));
                    });
                });
            });
        });
}

/// Draw an amp card matching Studio One layout: 2 big knobs (left) + ADSR faders (center) + small knobs (right)
fn draw_amp_card(
    ui: &mut Ui,
    playback_params: &mut PlaybackParams,
    envelope: &mut ADSREnvelope,
    theme: &Theme,
    functional_color: Color32,
    placeholder_color: Color32,
    response: &mut SampleZoneResponse,
) {
    let mut dummy_val = 0.5;

    GlassPanel::new()
        .title("AMP")
        .opacity(0.5)
        .glow_intensity(0.3)
        .inner_margin(theme.spacing.sm)
        .width(CARD_WIDTH)
        .height(CARD_HEIGHT)
        .show(ui, theme, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = theme.spacing.md;

                // Big knobs (Gain, Pan)
                ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = theme.spacing.sm;

                            // Gain knob (✓ FUNCTIONAL)
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 2.0;
                                ui.label(egui::RichText::new("Gain").size(LABEL_SIZE).strong().color(functional_color))
                                    .on_hover_text("Connected: Controls output amplitude");
                                let mut vol_knob = playback_params.volume;
                                let vol_resp = Knob::new(vol_knob).diameter(KNOB_BIG).show(ui, &mut vol_knob, theme);
                                playback_params.volume = vol_resp.value;
                                ui.label(egui::RichText::new(format!("{:.0}%", vol_resp.value * 100.0)).size(VALUE_SIZE));
                                response.playback_params_changed = true;
                            });

                            // Pan knob (✓ FUNCTIONAL)
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 2.0;
                                ui.label(egui::RichText::new("Pan").size(LABEL_SIZE).strong().color(functional_color))
                                    .on_hover_text("Connected: Controls stereo position");
                                let mut pan_norm = (playback_params.pan + 1.0) / 2.0;
                                let pan_resp = Knob::new(pan_norm).diameter(KNOB_BIG).show(ui, &mut pan_norm, theme);
                                playback_params.pan = (pan_resp.value * 2.0) - 1.0;
                                ui.label(egui::RichText::new(format!("{:+.1}", playback_params.pan)).size(VALUE_SIZE));
                                response.playback_params_changed = true;
                            });
                        });

                // ADSR Faders
                draw_adsr_faders(ui, envelope, theme, functional_color, response);

                // Right: Small knobs stacked vertically
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = CONTROL_SPACING;

                    // LFO knob
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("LFO").size(LABEL_SIZE).color(placeholder_color));
                        let _ = Knob::new(dummy_val).diameter(KNOB_SMALL).show(ui, &mut dummy_val, theme);
                        ui.label(egui::RichText::new("0").size(VALUE_SIZE).color(placeholder_color));
                    });

                    // Mod knob
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("Mod").size(LABEL_SIZE).color(placeholder_color));
                        let _ = Knob::new(dummy_val).diameter(KNOB_SMALL).show(ui, &mut dummy_val, theme);
                        ui.label(egui::RichText::new("0").size(VALUE_SIZE).color(placeholder_color));
                    });
                });
            });
        });
}

/// Draw LFO card matching Studio One layout: waveform selector + 2 medium knobs + buttons + small mod knob
fn draw_lfo_card(
    ui: &mut Ui,
    theme: &Theme,
    placeholder_color: Color32,
) {
    let mut dummy_val = 0.5;

    GlassPanel::new()
        .title("LFO")
        .opacity(0.5)
        .glow_intensity(0.3)
        .inner_margin(theme.spacing.sm)
        .width(CARD_WIDTH)
        .height(CARD_HEIGHT)
        .show(ui, theme, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = theme.spacing.md;

                // Left: Waveform selector (placeholder - just a button for now)
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 2.0;
                    ui.label(egui::RichText::new("Wave").size(LABEL_SIZE).color(placeholder_color));
                    if ui.button("~").clicked() {
                        // Placeholder for waveform selection
                    }
                });

                // Center: 2 medium knobs (Rate, Delay)
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = theme.spacing.sm;

                    // Rate knob
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("Rate").size(LABEL_SIZE).color(placeholder_color));
                        let _ = Knob::new(dummy_val).diameter(KNOB_MEDIUM).show(ui, &mut dummy_val, theme);
                        ui.label(egui::RichText::new("1Hz").size(VALUE_SIZE).color(placeholder_color));
                    });
                });

                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = theme.spacing.sm;

                    // Delay knob
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("Delay").size(LABEL_SIZE).color(placeholder_color));
                        let _ = Knob::new(dummy_val).diameter(KNOB_MEDIUM).show(ui, &mut dummy_val, theme);
                        ui.label(egui::RichText::new("0ms").size(VALUE_SIZE).color(placeholder_color));
                    });
                });

                // Right: Sync/Free buttons + Mod knob
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = theme.spacing.sm;

                    // Sync/Free buttons (stacked)
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        if ui.button("Sync").clicked() {
                            // Placeholder
                        }
                        if ui.button("Free").clicked() {
                            // Placeholder
                        }
                    });

                    // Mod knob
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        ui.label(egui::RichText::new("Mod").size(LABEL_SIZE).color(placeholder_color));
                        let _ = Knob::new(dummy_val).diameter(KNOB_SMALL).show(ui, &mut dummy_val, theme);
                        ui.label(egui::RichText::new("0").size(VALUE_SIZE).color(placeholder_color));
                    });
                });
            });
        });
}

/// Apply pitch shift, volume scaling, and pan to sample data
fn apply_pitch_volume_and_pan<T>(
    sample_data: &[T],
    amplitude_fn: fn(&T) -> f32,
    pitch_semitones: f32,
    volume: f32,
    pan: f32,
) -> Vec<f32> {
    // Calculate pitch ratio (semitones to frequency ratio)
    // Each semitone is 2^(1/12) = 1.0594630943592953
    let pitch_ratio = 2.0_f32.powf(pitch_semitones / 12.0);

    // Resample based on pitch shift
    let resampled = if (pitch_ratio - 1.0).abs() > 0.001 {
        // Pitch was changed - need to resample
        resample_samples(sample_data, amplitude_fn, pitch_ratio)
    } else {
        // No pitch change - just apply amplitude function
        sample_data.iter().map(amplitude_fn).collect()
    };

    // Apply volume and pan scaling
    // Pan creates an asymmetrical waveform: left side quieter when panned right, vice versa
    resampled
        .iter()
        .enumerate()
        .map(|(i, &s)| {
            let sample_ratio = i as f32 / resampled.len().max(1) as f32;

            // Pan creates a gradient effect: -1.0 (left) to +1.0 (right)
            // At the left side of the waveform: multiply by (1 - pan) / 2
            // At the right side of the waveform: multiply by (1 + pan) / 2
            let pan_factor =
                ((1.0 - pan) * (1.0 - sample_ratio) + (1.0 + pan) * sample_ratio) / 2.0;
            s * volume * pan_factor
        })
        .collect()
}

/// Resample audio based on pitch ratio
fn resample_samples<T>(
    sample_data: &[T],
    amplitude_fn: fn(&T) -> f32,
    pitch_ratio: f32,
) -> Vec<f32> {
    if pitch_ratio <= 0.0 || sample_data.is_empty() {
        return vec![];
    }

    // When pitch is higher (ratio > 1), the sample plays faster, so duration gets shorter
    // When pitch is lower (ratio < 1), the sample plays slower, so duration gets longer
    let new_length = (sample_data.len() as f32 / pitch_ratio).ceil() as usize;
    let mut resampled = Vec::with_capacity(new_length);

    // Linear interpolation resampling
    for i in 0..new_length {
        let src_pos = i as f32 * pitch_ratio;
        let src_idx = src_pos as usize;

        if src_idx >= sample_data.len() {
            break;
        }

        if src_idx + 1 < sample_data.len() {
            // Linear interpolation between two samples
            let frac = src_pos - src_idx as f32;
            let sample1 = amplitude_fn(&sample_data[src_idx]);
            let sample2 = amplitude_fn(&sample_data[src_idx + 1]);
            let interpolated = sample1 * (1.0 - frac) + sample2 * frac;
            resampled.push(interpolated);
        } else {
            // Last sample
            resampled.push(amplitude_fn(&sample_data[src_idx]));
        }
    }

    resampled
}

/// Calculate the duration after pitch shifting
fn calculate_pitch_adjusted_duration(original_duration: f64, pitch_semitones: f32) -> f64 {
    let pitch_ratio = 2.0_f32.powf(pitch_semitones / 12.0) as f64;
    original_duration / pitch_ratio
}

/// Create stereo channels from a mono waveform with pan
fn create_stereo_channels(samples: &[f32], pan: f32) -> (Vec<f32>, Vec<f32>) {
    // Pan creates a stereo image:
    // - When panned left (pan < 0): left channel is louder
    // - When panned right (pan > 0): right channel is louder
    // - When centered (pan = 0): both channels equal

    // Calculate left and right gains from pan value (-1.0 to 1.0)
    // Using equal power panning law for natural sound
    let left_gain = ((1.0 - pan) / 2.0).sqrt();
    let right_gain = ((1.0 + pan) / 2.0).sqrt();

    let left_channel: Vec<f32> = samples.iter().map(|&s| s * left_gain).collect();
    let right_channel: Vec<f32> = samples.iter().map(|&s| s * right_gain).collect();

    (left_channel, right_channel)
}
