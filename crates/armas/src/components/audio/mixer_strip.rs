//! Mixer Strip Component
//!
//! Complete DAW-style mixer channel strip with sends, routing, inserts, pan,
//! mute/solo, meter, and fader.

use crate::components::audio::{AudioMeter, Fader, Knob, Slot};
use crate::components::basic::Badge;
use crate::components::button::{Button, ButtonVariant};
use crate::components::cards::{Card, CardVariant};
use crate::components::overlays::Modal;
use crate::ext::ArmasContextExt;
use crate::layout::GlowingDivider;
use egui::{Color32, Id, Response, Ui, Vec2};

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
/// A send on the mixer strip
#[derive(Clone, Debug)]
pub struct Send {
    /// Send name (e.g., "Reverb", "Delay")
    pub name: String,
    /// Send level (0.0 to 1.0)
    pub level: f32,
    /// Pre-fader (true) or post-fader (false)
    pub pre_fader: bool,
    /// Mute state
    pub muted: bool,
}

impl Send {
    /// Create a new send
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            level: 0.5,
            pre_fader: false,
            muted: false,
        }
    }
}

/// An insert slot on the mixer strip (for plugins/effects)
#[derive(Clone, Debug)]
pub struct Insert {
    /// Plugin/effect name (None if empty slot)
    pub name: Option<String>,
    /// Whether the insert is bypassed
    pub bypassed: bool,
}

impl Insert {
    /// Create an empty insert slot
    pub fn empty() -> Self {
        Self {
            name: None,
            bypassed: false,
        }
    }

    /// Create an insert with a plugin
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            bypassed: false,
        }
    }
}

/// Routing configuration for input/output
#[derive(Clone, Debug)]
pub struct Route {
    /// Route name (e.g., "Input 1", "Main", "Bus A")
    pub name: String,
}

impl Route {
    /// Create a new route
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

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
    /// Insert slots
    inserts: Vec<Insert>,
    /// Input routing
    input_route: Route,
    /// Output routing
    output_route: Route,
    /// Card background color
    card_color: Option<Color32>,
    /// Sends
    sends: Vec<Send>,
}

impl MixerStrip {
    /// Create a new mixer strip
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let id = Id::new(format!("mixer_strip_{}", name));

        Self {
            name,
            id,
            width: 70.0,
            fader_level: 0.75,
            pan: 0.0,
            muted: false,
            soloed: false,
            meter_level: 0.0,
            inserts: vec![Insert::empty(); 4],
            input_route: Route::new("Input 1"),
            output_route: Route::new("Main"),
            card_color: None,
            sends: vec![Send::new("Reverb"), Send::new("Delay")],
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

    /// Set card background color
    pub fn card_color(mut self, color: Color32) -> Self {
        self.card_color = Some(color);
        self
    }

    /// Show the mixer strip
    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let theme = ui.ctx().armas_theme();

        let default_color = Color32::from_rgb(28, 28, 30);
        let card_response = Card::new()
            .variant(CardVariant::Filled)
            .width(self.width)
            .corner_radius(8.0)
            .inner_margin(2.0)
            .fill(self.card_color.unwrap_or(default_color))
            .show(ui, &theme, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.add_space(theme.spacing.xs);
                    // Sends section
                    // Sends header as clickable outlined button
                    let sends_response = Button::new("Sends")
                        .variant(ButtonVariant::Outlined)
                        .min_size(Vec2::new(40.0, theme.spacing.lg))
                        .show(ui);

                    if sends_response.clicked() {
                        ui.ctx().memory_mut(|mem| {
                            mem.data.insert_temp(self.id.with("send_modal_open"), true);
                            mem.data
                                .insert_temp(self.id.with("send_modal_view"), "list");
                        });
                    }

                    ui.add_space(theme.spacing.sm);

                    // Current sends as badges (centered, vertical layout)
                    for send in self.sends.iter() {
                        Badge::new(&send.name)
                            .corner_radius(4.0)
                            .size(13.0)
                            .vertical_padding(4.0)
                            .show(ui);
                        ui.add_space(theme.spacing.xs);
                    }

                    GlowingDivider::new().show(ui, &theme);

                    // Input routing
                    if Button::new(&self.input_route.name)
                        .variant(ButtonVariant::Outlined)
                        .min_size(Vec2::new(60.0, theme.spacing.lg))
                        .show(ui)
                        .clicked()
                    {
                        // TODO: Show dropdown
                    }

                    ui.add_space(theme.spacing.xs);

                    // Output routing
                    if Button::new(&self.output_route.name)
                        .variant(ButtonVariant::Outlined)
                        .min_size(Vec2::new(60.0, theme.spacing.lg))
                        .show(ui)
                        .clicked()
                    {
                        // TODO: Show dropdown
                    }

                    ui.add_space(theme.spacing.sm);

                    // Insert slots
                    for (_i, insert) in self.inserts.iter().enumerate() {
                        let slot = if let Some(ref name) = insert.name {
                            Slot::new().effect(name).width(self.width - 24.0)
                        } else {
                            Slot::new().width(self.width - 24.0)
                        };

                        slot.show(ui);
                        ui.add_space(theme.spacing.xs);
                    }

                    ui.add_space(-theme.spacing.sm);

                    // Pan knob
                    // Convert pan from -1..1 to 0..1 for knob
                    let mut pan_knob_value = (self.pan + 1.0) / 2.0;
                    Knob::new(pan_knob_value)
                        .diameter(40.0)
                        .show_value(false)
                        .show(ui, &mut pan_knob_value, &theme);
                    // Convert back from 0..1 to -1..1
                    self.pan = (pan_knob_value * 2.0 - 1.0).clamp(-1.0, 1.0);

                    // Compensate for knob's extra space allocation
                    ui.add_space(-22.0);

                    // Mute/Solo buttons
                    ui.horizontal(|ui| {
                        let button_width = (self.width - 8.0) / 2.0;

                        if Button::new("M")
                            .variant(ButtonVariant::Outlined)
                            .min_size(Vec2::new(button_width, theme.spacing.lg))
                            .show(ui)
                            .clicked()
                        {
                            self.muted = !self.muted;
                        }

                        ui.add_space(theme.spacing.xs);

                        if Button::new("S")
                            .variant(ButtonVariant::Outlined)
                            .min_size(Vec2::new(button_width, theme.spacing.lg))
                            .show(ui)
                            .clicked()
                        {
                            self.soloed = !self.soloed;
                        }
                    });

                    ui.add_space(theme.spacing.md);

                    // Gain display (dB)
                    let db_value = if self.fader_level > 0.0 {
                        20.0 * self.fader_level.log10()
                    } else {
                        -60.0
                    };

                    ui.label(format!("{:+.1}", db_value));

                    ui.add_space(theme.spacing.sm);

                    // Meter and fader side by side
                    ui.horizontal(|ui| {
                        // Meter
                        AudioMeter::new(self.meter_level)
                            .width(16.0)
                            .height(180.0)
                            .scale_left()
                            .show(ui);

                        ui.add_space(theme.spacing.xs);

                        // Fader
                        let (_, new_level) =
                            Fader::new(self.fader_level).size(30.0, 180.0).show(ui);
                        self.fader_level = new_level;
                    });

                    ui.add_space(8.0);

                    // Channel number and name
                    ui.label(&self.name);
                });
            });

        // Sends modal with hierarchical navigation
        let modal_open: bool = ui.ctx().memory(|mem| {
            mem.data
                .get_temp(self.id.with("send_modal_open"))
                .unwrap_or(false)
        });

        if modal_open {
            let current_view: String = ui.ctx().memory(|mem| {
                mem.data
                    .get_temp(self.id.with("send_modal_view"))
                    .unwrap_or_else(|| "list".to_string())
            });

            let (modal_title, show_back) = if current_view == "list" {
                ("Sends".to_string(), false)
            } else {
                let send_index: usize = ui.ctx().memory(|mem| {
                    mem.data
                        .get_temp(self.id.with("send_modal_index"))
                        .unwrap_or(0)
                });
                let send_name = self
                    .sends
                    .get(send_index)
                    .map(|s| s.name.as_str())
                    .unwrap_or("Send");
                (format!("Edit: {}", send_name), true)
            };

            let modal_response = Modal::new(self.id.with("send_modal"))
                .title(modal_title)
                .open(modal_open)
                .show(ui.ctx(), &theme, |ui| {
                    ui.add_space(8.0);

                    // Back button if in edit view
                    if show_back {
                        if ui.button("← Back to Sends").clicked() {
                            ui.ctx().memory_mut(|mem| {
                                mem.data
                                    .insert_temp(self.id.with("send_modal_view"), "list");
                            });
                        }
                        ui.add_space(8.0);

                        ui.add_space(8.0);
                    }

                    if current_view == "list" {
                        // Send list view
                        ui.heading("Add Sends");
                        ui.add_space(8.0);

                        if ui.button("+ Add Send").clicked() {
                            // TODO: Add new send
                        }

                        ui.add_space(16.0);
                        ui.separator();
                        ui.add_space(16.0);

                        ui.heading("Existing Sends");
                        ui.add_space(8.0);

                        // Dynamic send list
                        for (i, send) in self.sends.iter().enumerate() {
                            if ui.button(&send.name).clicked() {
                                ui.ctx().memory_mut(|mem| {
                                    mem.data
                                        .insert_temp(self.id.with("send_modal_view"), "edit");
                                    mem.data.insert_temp(self.id.with("send_modal_index"), i);
                                });
                            }
                        }
                    } else {
                        // Individual send edit view
                        ui.label("Level:");
                        let mut send_level = 0.5;
                        Knob::new(send_level).diameter(60.0).label("dB").show(
                            ui,
                            &mut send_level,
                            &theme,
                        );

                        ui.add_space(16.0);

                        // Pre/Post fader
                        ui.horizontal(|ui| {
                            ui.label("Routing:");
                            ui.add_space(8.0);
                            if ui.button("Pre-Fader").clicked() {
                                // TODO: Set pre-fader
                            }
                            ui.add_space(4.0);
                            if ui.button("Post-Fader").clicked() {
                                // TODO: Set post-fader
                            }
                        });

                        ui.add_space(16.0);

                        // Mute button
                        ui.horizontal(|ui| {
                            ui.label("Status:");
                            ui.add_space(8.0);
                            if ui.button("Mute").clicked() {
                                // TODO: Toggle mute
                            }
                        });
                    }

                    ui.add_space(8.0);
                });

            if modal_response.closed {
                ui.ctx().memory_mut(|mem| {
                    mem.data.insert_temp(self.id.with("send_modal_open"), false);
                    mem.data
                        .insert_temp(self.id.with("send_modal_view"), "list");
                });
            }
        }

        card_response.response
    }
}

impl Default for MixerStrip {
    fn default() -> Self {
        Self::new("Channel")
    }
}
