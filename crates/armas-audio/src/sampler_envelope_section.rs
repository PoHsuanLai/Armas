//! ADSR Envelope section for sampler
//!
//! Simplified automation editor wrapper configured for ADSR (Attack, Decay, Sustain, Release) curves.

use crate::{AutomationEditor, CanvasConfig};
use armas::components::basic::Slider;
use armas::components::cards::{Card, CardVariant};
use armas::ext::ArmasContextExt;
use armas::theme::Theme;
use audio_automation::{AutomationEnvelope, AutomationPoint, CurveType};
use egui::{Ui, Vec2};

/// ADSR envelope parameters
#[derive(Debug, Clone)]
pub struct ADSREnvelope {
    /// Attack time in seconds (0-5s)
    pub attack: f32,
    /// Decay time in seconds (0-5s)
    pub decay: f32,
    /// Sustain level (0.0-1.0)
    pub sustain: f32,
    /// Release time in seconds (0-5s)
    pub release: f32,
    /// Internal automation envelope for visualization
    pub automation: AutomationEnvelope<CurveType>,
}

impl Default for ADSREnvelope {
    fn default() -> Self {
        let mut automation = AutomationEnvelope::new(CurveType::Linear);

        // Default ADSR shape
        automation.add_point(AutomationPoint::new(0.0, 0.0));      // Start
        automation.add_point(AutomationPoint::new(0.1, 1.0));      // Attack peak
        automation.add_point(AutomationPoint::new(0.3, 0.7));      // Decay to sustain
        automation.add_point(AutomationPoint::new(0.8, 0.7));      // Sustain level
        automation.add_point(AutomationPoint::new(1.0, 0.0));      // Release end

        Self {
            attack: 0.1,
            decay: 0.2,
            sustain: 0.7,
            release: 0.2,
            automation,
        }
    }
}

/// Response from envelope section
#[derive(Debug, Clone)]
pub struct EnvelopeSectionResponse {
    pub attack_changed: bool,
    pub decay_changed: bool,
    pub sustain_changed: bool,
    pub release_changed: bool,
    pub envelope: ADSREnvelope,
}

/// ADSR envelope section UI
pub struct EnvelopeSection<'a> {
    envelope: &'a mut ADSREnvelope,
    theme: &'a Theme,
    canvas_size: Vec2,
    show_labels: bool,
}

impl<'a> EnvelopeSection<'a> {
    /// Create a new envelope section
    pub fn new(envelope: &'a mut ADSREnvelope, theme: &'a Theme) -> Self {
        Self {
            envelope,
            theme,
            canvas_size: Vec2::new(600.0, 200.0),
            show_labels: true,
        }
    }

    /// Set canvas size
    pub fn canvas_size(mut self, size: Vec2) -> Self {
        self.canvas_size = size;
        self
    }

    /// Show/hide parameter labels
    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    /// Show the envelope section
    pub fn show(mut self, ui: &mut Ui) -> EnvelopeSectionResponse {
        let theme = ui.ctx().armas_theme();
        let mut response = EnvelopeSectionResponse {
            attack_changed: false,
            decay_changed: false,
            sustain_changed: false,
            release_changed: false,
            envelope: self.envelope.clone(),
        };

        ui.spacing_mut().item_spacing.y = self.theme.spacing.lg;

        // Envelope visualization using Card with rounded corners
        Card::new()
            .variant(CardVariant::Filled)
            .corner_radius(12.0)
            .show(ui, &theme, |ui| {
                ui.spacing_mut().item_spacing.y = self.theme.spacing.md;

                ui.label(
                    egui::RichText::new("Envelope Curve")
                        .size(13.0)
                        .color(theme.muted_foreground()),
                );

                let config = CanvasConfig {
                    pixels_per_beat: 100.0,
                    pixels_per_value: self.canvas_size.y / 2.0,
                    min_value: 0.0,
                    max_value: 1.0,
                    grid_subdivisions: 4,
                    snap_enabled: true,
                    snap_interval: 0.05,
                };

                let _editor_response = AutomationEditor::new(&mut self.envelope.automation)
                    .canvas_size(self.canvas_size)
                    .canvas_config(config)
                    .point_color(self.theme.primary())
                    .show_values(true)
                    .show(ui);
            });

        ui.spacing_mut().item_spacing.y = self.theme.spacing.lg;

        // ADSR parameter controls in their own card with better spacing
        Card::new()
            .variant(CardVariant::Filled)
            .corner_radius(12.0)
            .show(ui, &theme, |ui| {
                ui.spacing_mut().item_spacing.y = self.theme.spacing.md;

                if self.show_labels {
                    ui.label(
                        egui::RichText::new("ADSR Parameters")
                            .size(13.0)
                            .color(theme.muted_foreground()),
                    );
                }

                ui.spacing_mut().item_spacing.x = self.theme.spacing.lg;
                ui.horizontal(|ui| {
                    // Attack
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = self.theme.spacing.sm;
                        let attack_response = Slider::new(0.0, 5.0)
                            .label("Attack")
                            .suffix("s")
                            .show_value(true)
                            .show(ui, &mut self.envelope.attack);
                        if attack_response.changed {
                            response.attack_changed = true;
                            self.update_automation();
                        }
                    });

                    // Decay
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = self.theme.spacing.sm;
                        let decay_response = Slider::new(0.0, 5.0)
                            .label("Decay")
                            .suffix("s")
                            .show_value(true)
                            .show(ui, &mut self.envelope.decay);
                        if decay_response.changed {
                            response.decay_changed = true;
                            self.update_automation();
                        }
                    });

                    // Sustain
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = self.theme.spacing.sm;
                        let sustain_response = Slider::new(0.0, 1.0)
                            .label("Sustain")
                            .show_value(true)
                            .show(ui, &mut self.envelope.sustain);
                        if sustain_response.changed {
                            response.sustain_changed = true;
                            self.update_automation();
                        }
                    });

                    // Release
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = self.theme.spacing.sm;
                        let release_response = Slider::new(0.0, 5.0)
                            .label("Release")
                            .suffix("s")
                            .show_value(true)
                            .show(ui, &mut self.envelope.release);
                        if release_response.changed {
                            response.release_changed = true;
                            self.update_automation();
                        }
                    });
                });
            });

        response.envelope = self.envelope.clone();
        response
    }

    /// Update automation envelope based on current ADSR parameters
    fn update_automation(&mut self) {
        // Calculate total time
        let total_time = self.envelope.attack + self.envelope.decay + 1.0 + self.envelope.release;

        // Clear existing points
        self.envelope.automation.points.clear();

        // Add points for ADSR shape
        // Start point
        self.envelope
            .automation
            .add_point(AutomationPoint::new(0.0, 0.0));

        // Attack peak
        let attack_time = self.envelope.attack / total_time;
        self.envelope
            .automation
            .add_point(AutomationPoint::new(attack_time as f64, 1.0));

        // Decay to sustain
        let decay_end_time = (self.envelope.attack + self.envelope.decay) / total_time;
        self.envelope.automation.add_point(AutomationPoint::new(
            decay_end_time as f64,
            self.envelope.sustain,
        ));

        // Sustain level (hold for fixed time)
        let sustain_end_time = (self.envelope.attack + self.envelope.decay + 1.0) / total_time;
        self.envelope.automation.add_point(AutomationPoint::new(
            sustain_end_time as f64,
            self.envelope.sustain,
        ));

        // Release end
        self.envelope
            .automation
            .add_point(AutomationPoint::new(1.0, 0.0));
    }
}
