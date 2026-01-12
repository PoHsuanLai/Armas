//! Mixer Strip Component
//!
//! Complete DAW-style mixer channel strip with sends, routing, inserts, pan,
//! mute/solo, meter, and fader.

use crate::components::audio::{AudioMeter, Fader, Knob, Slot};
use crate::components::basic::AccordionItem;
use crate::ext::ArmasContextExt;
use egui::{Color32, Id, Response, Sense, Ui, Vec2};

/// Mixer channel strip component
///
/// A complete mixer channel strip inspired by professional DAW interfaces.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::components::MixerStrip;
///
/// let mut strip = MixerStrip::new("Channel 1");
/// strip.show(ui);
/// # }
/// ```
pub struct MixerStrip {
    /// Channel name
    name: String,
    /// Unique ID for state persistence
    id: Id,
    /// Width of the strip
    width: f32,
    /// Fader level (0.0 to 1.0)
    fader_level: f32,
    /// Pan value (-1.0 to 1.0)
    pan: f32,
    /// Mute state
    muted: bool,
    /// Solo state
    soloed: bool,
    /// Current meter level (0.0 to 1.0)
    meter_level: f32,
    /// Insert slots (plugin names)
    inserts: Vec<Option<String>>,
    /// Input routing
    input_route: String,
    /// Output routing
    output_route: String,
    /// Track color
    track_color: Option<Color32>,
}

impl MixerStrip {
    /// Create a new mixer strip
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let id = Id::new(format!("mixer_strip_{}", name));

        Self {
            name,
            id,
            width: 80.0,
            fader_level: 0.75,
            pan: 0.0,
            muted: false,
            soloed: false,
            meter_level: 0.0,
            inserts: vec![None; 4],
            input_route: "Input 1".to_string(),
            output_route: "Main".to_string(),
            track_color: None,
        }
    }

    /// Set strip width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(60.0);
        self
    }

    /// Set fader level
    pub fn fader_level(mut self, level: f32) -> Self {
        self.fader_level = level.clamp(0.0, 1.0);
        self
    }

    /// Set pan value
    pub fn pan(mut self, pan: f32) -> Self {
        self.pan = pan.clamp(-1.0, 1.0);
        self
    }

    /// Set meter level
    pub fn meter_level(mut self, level: f32) -> Self {
        self.meter_level = level.clamp(0.0, 1.0);
        self
    }

    /// Set track color
    pub fn track_color(mut self, color: Color32) -> Self {
        self.track_color = Some(color);
        self
    }

    /// Show the mixer strip
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().armas_theme();
        let desired_size = Vec2::new(self.width, ui.available_height());
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(rect));
            child_ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.vertical(|ui| {
                    ui.set_width(self.width);

                    // Sends section (collapsible)
                    let mut sends_item = AccordionItem::new("Sends")
                        .id(self.id.with("sends"))
                        .open(false);

                    sends_item.show(ui, |ui| {
                        ui.label("Send 1: Reverb");
                        ui.label("Send 2: Delay");
                    });

                    ui.add_space(4.0);

                    // Input routing
                    ui.label("Input");
                    ui.horizontal(|ui| {
                        ui.set_width(self.width - 8.0);
                        if ui.button(&self.input_route).clicked() {
                            // TODO: Show dropdown
                        }
                    });

                    ui.add_space(4.0);

                    // Output routing
                    ui.label("Output");
                    ui.horizontal(|ui| {
                        ui.set_width(self.width - 8.0);
                        if ui.button(&self.output_route).clicked() {
                            // TODO: Show dropdown
                        }
                    });

                    ui.add_space(8.0);

                    // Insert slots
                    for (i, insert) in self.inserts.iter().enumerate() {
                        let slot = if let Some(name) = insert {
                            Slot::new().effect(name).width(self.width - 8.0)
                        } else {
                            Slot::new().width(self.width - 8.0)
                        };

                        ui.horizontal(|ui| {
                            ui.add_space(4.0);
                            slot.show(ui);
                        });
                        ui.add_space(2.0);
                    }

                    ui.add_space(8.0);

                    // Pan knob
                    ui.horizontal(|ui| {
                        ui.add_space((self.width - 40.0) / 2.0);
                        Knob::new(self.pan)
                            .diameter(40.0)
                            .show_value(false)
                            .show(ui);
                    });

                    ui.horizontal(|ui| {
                        ui.add_space((self.width - 30.0) / 2.0);
                        ui.label("<C>");
                    });

                    ui.add_space(8.0);

                    // Mute/Solo buttons
                    ui.horizontal(|ui| {
                        ui.add_space(4.0);
                        let button_width = (self.width - 12.0) / 2.0;

                        if ui.add_sized([button_width, 24.0],
                            egui::Button::new("M")
                                .fill(if self.muted { theme.error() } else { theme.surface() })
                        ).clicked() {
                            self.muted = !self.muted;
                        }

                        if ui.add_sized([button_width, 24.0],
                            egui::Button::new("S")
                                .fill(if self.soloed { theme.warning() } else { theme.surface() })
                        ).clicked() {
                            self.soloed = !self.soloed;
                        }
                    });

                    ui.add_space(8.0);

                    // Gain display (dB)
                    let db_value = if self.fader_level > 0.0 {
                        20.0 * self.fader_level.log10()
                    } else {
                        -60.0
                    };

                    ui.horizontal(|ui| {
                        ui.add_space((self.width - 40.0) / 2.0);
                        ui.label(format!("{:+.1}", db_value));
                    });

                    ui.add_space(4.0);

                    // Meter and fader side by side
                    ui.horizontal(|ui| {
                        ui.add_space(4.0);

                        // Meter
                        AudioMeter::new(self.meter_level)
                            .width(16.0)
                            .height(180.0)
                            .scale_left()
                            .show(ui);

                        ui.add_space(4.0);

                        // Fader
                        let (_, new_level) = Fader::new(self.fader_level)
                            .size(30.0, 180.0)
                            .show(ui);
                        self.fader_level = new_level;
                    });

                    ui.add_space(8.0);

                    // Channel number and name
                    ui.horizontal(|ui| {
                        ui.add_space((self.width - 60.0) / 2.0);
                        ui.label(&self.name);
                    });

                    ui.add_space(4.0);

                    // Track label at bottom
                    if let Some(color) = self.track_color {
                        ui.horizontal(|ui| {
                            ui.add_space(4.0);
                            let label_rect = ui.available_rect_before_wrap();
                            ui.painter().rect_filled(
                                label_rect.shrink(2.0),
                                4.0,
                                color,
                            );
                            ui.allocate_space(Vec2::new(self.width - 8.0, 20.0));
                        });
                    }
                });
            });
        }

        response
    }
}

impl Default for MixerStrip {
    fn default() -> Self {
        Self::new("Channel")
    }
}
