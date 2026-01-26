//! Mixer Strip Component
//!
//! Complete DAW-style mixer channel strip with sends, routing, inserts, pan,
//! mute/solo, meter, and fader.

use crate::{AudioMeter, Fader, Knob, Slot};
use armas::components::basic::Badge;
use armas::components::button::{Button, ButtonVariant};
use armas::components::cards::{Card, CardVariant};
use armas::components::overlays::Dialog;
use armas::ext::ArmasContextExt;
use armas::Separator;
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

/// Response from the mixer strip
#[derive(Debug, Clone)]
pub struct MixerStripResponse {
    /// The UI response
    pub response: Response,
    /// Fader level changed this frame
    pub fader_changed: bool,
    /// Pan value changed this frame
    pub pan_changed: bool,
    /// Mute state toggled this frame
    pub mute_toggled: bool,
    /// Solo state toggled this frame
    pub solo_toggled: bool,
    /// Record arm toggled this frame
    pub record_toggled: bool,
    /// Input monitoring toggled this frame
    pub monitor_toggled: bool,
    /// Sends button clicked
    pub sends_clicked: bool,
    /// Input routing clicked
    pub input_routing_clicked: bool,
    /// Output routing clicked
    pub output_routing_clicked: bool,
}

pub struct MixerStrip {
    /// Channel name
    name: String,
    /// Unique ID for state persistence
    id: Id,
    /// Width of the strip (base width, will be multiplied by scale)
    width: f32,
    /// Scale factor for zoom (1.0 = 100%, 0.8 = 80%, 1.2 = 120%)
    scale: f32,
    /// Fader level (0.0 to 1.0)
    fader_level: f32,
    /// Pan value (-1.0 to 1.0)
    pan: f32,
    /// Mute state
    muted: bool,
    /// Solo state
    soloed: bool,
    /// Record arm state
    record_armed: bool,
    /// Input monitoring state
    input_monitoring: bool,
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
    /// Knob glow color (overrides card_color if set)
    knob_color: Option<Color32>,
    /// Meter color (overrides card_color if set)
    meter_color: Option<Color32>,
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
            scale: 1.0,
            fader_level: 0.75,
            pan: 0.0,
            muted: false,
            soloed: false,
            record_armed: false,
            input_monitoring: false,
            meter_level: 0.0,
            inserts: vec![Insert::empty(); 4],
            input_route: Route::new("Input 1"),
            output_route: Route::new("Main"),
            card_color: None,
            knob_color: None,
            meter_color: None,
            sends: vec![Send::new("Reverb"), Send::new("Delay")],
        }
    }

    /// Set strip width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(60.0);
        self
    }

    /// Set scale factor for zoom (1.0 = 100%, 0.8 = 80%, 1.2 = 120%)
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale.clamp(0.5, 2.0); // Clamp between 50% and 200%
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

    /// Set knob glow color (overrides card_color for knob)
    pub fn knob_color(mut self, color: Color32) -> Self {
        self.knob_color = Some(color);
        self
    }

    /// Set meter color (overrides card_color for meter)
    pub fn meter_color(mut self, color: Color32) -> Self {
        self.meter_color = Some(color);
        self
    }

    /// Set mute state
    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    /// Set solo state
    pub fn soloed(mut self, soloed: bool) -> Self {
        self.soloed = soloed;
        self
    }

    /// Set record arm state
    pub fn record_armed(mut self, armed: bool) -> Self {
        self.record_armed = armed;
        self
    }

    /// Set input monitoring state
    pub fn input_monitoring(mut self, monitoring: bool) -> Self {
        self.input_monitoring = monitoring;
        self
    }

    /// Set insert slots
    pub fn inserts(mut self, inserts: Vec<Insert>) -> Self {
        self.inserts = inserts;
        self
    }

    /// Set sends
    pub fn sends(mut self, sends: Vec<Send>) -> Self {
        self.sends = sends;
        self
    }

    /// Set input route
    pub fn input_route(mut self, route: Route) -> Self {
        self.input_route = route;
        self
    }

    /// Set output route
    pub fn output_route(mut self, route: Route) -> Self {
        self.output_route = route;
        self
    }

    // Getter methods

    /// Get the channel name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the fader level (0.0 to 1.0)
    pub fn get_fader_level(&self) -> f32 {
        self.fader_level
    }

    /// Get the pan value (-1.0 to 1.0)
    pub fn get_pan(&self) -> f32 {
        self.pan
    }

    /// Get mute state
    pub fn is_muted(&self) -> bool {
        self.muted
    }

    /// Get solo state
    pub fn is_soloed(&self) -> bool {
        self.soloed
    }

    /// Get record arm state
    pub fn is_record_armed(&self) -> bool {
        self.record_armed
    }

    /// Get input monitoring state
    pub fn is_input_monitoring(&self) -> bool {
        self.input_monitoring
    }

    /// Get meter level
    pub fn get_meter_level(&self) -> f32 {
        self.meter_level
    }

    /// Get inserts
    pub fn get_inserts(&self) -> &[Insert] {
        &self.inserts
    }

    /// Get input route
    pub fn get_input_route(&self) -> &Route {
        &self.input_route
    }

    /// Get output route
    pub fn get_output_route(&self) -> &Route {
        &self.output_route
    }

    /// Get sends
    pub fn get_sends(&self) -> &[Send] {
        &self.sends
    }

    /// Show the mixer strip
    pub fn show(&mut self, ui: &mut Ui) -> MixerStripResponse {
        let theme = ui.ctx().armas_theme();
        let scale = self.scale;

        // Track changes this frame
        let old_fader = self.fader_level;
        let old_pan = self.pan;
        let old_mute = self.muted;
        let old_solo = self.soloed;
        let old_record = self.record_armed;
        let old_monitor = self.input_monitoring;
        let mut sends_clicked = false;
        let mut input_routing_clicked = false;
        let mut output_routing_clicked = false;

        // Apply scale to all dimensions
        let scaled_width = self.width * scale;
        let _button_height = 20.0 * scale;
        let slot_height = 20.0 * scale;
        let knob_diameter = 40.0 * scale;
        let meter_fader_height = 180.0 * scale;
        let meter_width = 12.0 * scale;
        let fader_width = 30.0 * scale;

        let default_color = Color32::from_rgb(28, 28, 30);
        let card_response = Card::new()
            .variant(CardVariant::Filled)
            .width(scaled_width)
            .corner_radius(8.0 * scale)
            .inner_margin(2.0 * scale)
            .fill(self.card_color.unwrap_or(default_color))
            .show(ui, &theme, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.add_space(theme.spacing.xs * scale);
                    // Sends section
                    // Sends header as clickable outlined button (compact height, fixed width)
                    let button_width_full = scaled_width - 4.0 * scale; // Card has 2px inner margin on each side
                    let sends_response = Button::new("Sends")
                        .variant(ButtonVariant::Outline)
                        .min_width(button_width_full)
                        .show(ui);

                    if sends_response.clicked() {
                        sends_clicked = true;
                        ui.ctx().memory_mut(|mem| {
                            mem.data.insert_temp(self.id.with("send_modal_open"), true);
                            mem.data
                                .insert_temp(self.id.with("send_modal_view"), "list");
                        });
                    }

                    ui.add_space(theme.spacing.xs * scale);

                    // Current sends as badges (centered, vertical layout, fixed width)
                    for send in self.sends.iter() {
                        // Use allocate_ui_with_layout to control width
                        ui.allocate_ui_with_layout(
                            Vec2::new(button_width_full, 20.0 * scale),
                            egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                            |ui| {
                                Badge::new(&send.name)
                                    .corner_radius(4.0 * scale)
                                    .size(13.0 * scale)
                                    .vertical_padding(4.0 * scale)
                                    .show(ui);
                            },
                        );
                        ui.add_space(theme.spacing.xs * scale);
                    }

                    Separator::new().show(ui);

                    // Input routing (compact height, fixed width)
                    if Button::new(&self.input_route.name)
                        .variant(ButtonVariant::Outline)
                        .min_width(button_width_full)
                        .show(ui)
                        .clicked()
                    {
                        input_routing_clicked = true;
                        // TODO: Show dropdown
                    }

                    ui.add_space(theme.spacing.xs * scale);

                    // Output routing (compact height, fixed width)
                    if Button::new(&self.output_route.name)
                        .variant(ButtonVariant::Outline)
                        .min_width(button_width_full)
                        .show(ui)
                        .clicked()
                    {
                        output_routing_clicked = true;
                        // TODO: Show dropdown
                    }

                    ui.add_space(theme.spacing.xs * scale);

                    // Insert slots - compact height for mixer strip
                    for insert in self.inserts.iter() {
                        let slot_width = scaled_width - 24.0 * scale;
                        let slot = if let Some(ref name) = insert.name {
                            Slot::new()
                                .effect(name)
                                .width(slot_width)
                                .height(slot_height)
                        } else {
                            Slot::new().width(slot_width).height(slot_height)
                        };

                        slot.show(ui);
                        ui.add_space(theme.spacing.xs * scale);
                    }

                    ui.add_space(theme.spacing.xs * scale);

                    // Pan knob
                    // Load pan state (if it exists)
                    let pan_state_id = self.id.with("pan_state");
                    let current_pan = ui
                        .ctx()
                        .data_mut(|d| d.get_temp(pan_state_id).unwrap_or(self.pan));

                    // Convert pan from -1..1 to 0..1 for knob
                    let mut pan_knob_value = (current_pan + 1.0) / 2.0;

                    // Set glow color: knob_color > theme primary
                    let glow_color = self.knob_color.unwrap_or_else(|| theme.primary());
                    let knob = Knob::new(pan_knob_value)
                        .diameter(knob_diameter)
                        .show_value(false)
                        .glow_color(glow_color);

                    let knob_resp = knob.show(ui, &mut pan_knob_value, &theme);
                    // Convert back from 0..1 to -1..1
                    self.pan = (knob_resp.value * 2.0 - 1.0).clamp(-1.0, 1.0);

                    // Save pan state
                    ui.ctx().data_mut(|d| {
                        d.insert_temp(pan_state_id, self.pan);
                    });

                    // Pan value display
                    let pan_text = if self.pan.abs() < 0.05 {
                        "C".to_string() // Center
                    } else if self.pan < 0.0 {
                        format!("L{}", (self.pan.abs() * 100.0) as i32) // Left
                    } else {
                        format!("R{}", (self.pan * 100.0) as i32) // Right
                    };
                    ui.colored_label(theme.muted_foreground(), pan_text);

                    // Compensate for knob's extra space allocation
                    ui.add_space(theme.spacing.xs * scale);

                    // M/S/R/I buttons in 2x2 grid
                    // Calculate button width: (strip_width - card_inner_margin * 2 - spacing_between) / 2
                    // Card has 2.0 inner margin on each side = 4.0 total
                    // spacing.xs (4.0) between the two buttons in each row
                    let button_width_grid =
                        (scaled_width - 4.0 * scale - theme.spacing.xs * scale) / 2.0;

                    // First row: Mute and Solo
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;

                        if Button::new("M")
                            .variant(ButtonVariant::Outline)
                            .min_width(button_width_grid)
                            .show(ui)
                            .clicked()
                        {
                            self.muted = !self.muted;
                        }

                        ui.add_space(theme.spacing.xs * scale);

                        if Button::new("S")
                            .variant(ButtonVariant::Outline)
                            .min_width(button_width_grid)
                            .show(ui)
                            .clicked()
                        {
                            self.soloed = !self.soloed;
                        }
                    });

                    ui.add_space(theme.spacing.xs * scale);

                    // Second row: Record and Input Monitor
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;

                        if Button::new("R")
                            .variant(ButtonVariant::Outline)
                            .min_width(button_width_grid)
                            .show(ui)
                            .clicked()
                        {
                            self.record_armed = !self.record_armed;
                        }

                        ui.add_space(theme.spacing.xs * scale);

                        if Button::new("I")
                            .variant(ButtonVariant::Outline)
                            .min_width(button_width_grid)
                            .show(ui)
                            .clicked()
                        {
                            self.input_monitoring = !self.input_monitoring;
                        }
                    });

                    ui.add_space(theme.spacing.xs * scale);

                    // Load the current fader level from state (if it exists)
                    let current_fader_level = {
                        let state_id = self.id.with("fader").with("fader_state");
                        ui.ctx()
                            .data_mut(|d| d.get_temp(state_id).unwrap_or(self.fader_level))
                    };

                    // Gain display (dB) - convert fader position to dB value
                    // Using piecewise linear interpolation between marked positions
                    let db_value = {
                        // Reference points from fader scale
                        let points = [
                            (0.0, -60.0),  // -∞ dB (shown as -60)
                            (0.13, -18.0), // -18 dB
                            (0.25, -12.0), // -12 dB
                            (0.44, -6.0),  // -6 dB
                            (0.59, -3.0),  // -3 dB
                            (0.75, 0.0),   // 0 dB - unity gain
                            (0.87, 3.0),   // +3 dB
                            (1.0, 6.0),    // +6 dB
                        ];

                        // Find the two points to interpolate between
                        let mut db = -60.0;
                        for i in 0..points.len() - 1 {
                            if current_fader_level >= points[i].0
                                && current_fader_level <= points[i + 1].0
                            {
                                let t = (current_fader_level - points[i].0)
                                    / (points[i + 1].0 - points[i].0);
                                db = points[i].1 + t * (points[i + 1].1 - points[i].1);
                                break;
                            }
                        }
                        db
                    };

                    ui.colored_label(theme.foreground(), format!("{:+.1}", db_value));

                    ui.add_space(theme.spacing.xs * scale);

                    // Meter and fader side by side
                    ui.horizontal(|ui| {
                        // Meter
                        let mut meter = AudioMeter::new(self.meter_level)
                            .width(meter_width)
                            .height(meter_fader_height)
                            .scale_left();

                        // Set meter color: meter_color > theme primary
                        let meter_color = self.meter_color.unwrap_or_else(|| theme.primary());
                        meter = meter.color_range(
                            Color32::from_rgb(0, 0, 0), // Dark at bottom
                            meter_color,                // Resolved color at top
                        );

                        let _ = meter.show(ui);

                        // Fader (same height as meter - both now use full height)
                        // No scale needed - the dB value is displayed above
                        let (_, new_level) = Fader::new(self.fader_level)
                            .id(self.id.with("fader"))
                            .size(fader_width, meter_fader_height)
                            .show(ui);
                        self.fader_level = new_level;
                    });

                    ui.add_space(8.0 * scale);

                    // Channel number and name
                    ui.colored_label(theme.foreground(), &self.name);
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

            let modal_response = Dialog::new(self.id.with("send_modal"))
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
                        ui.colored_label(
                            theme.foreground(),
                            egui::RichText::new("Add Sends").heading(),
                        );
                        ui.add_space(8.0);

                        if ui.button("+ Add Send").clicked() {
                            // TODO: Add new send
                        }

                        ui.add_space(16.0);
                        ui.separator();
                        ui.add_space(16.0);

                        ui.colored_label(
                            theme.foreground(),
                            egui::RichText::new("Existing Sends").heading(),
                        );
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
                        ui.colored_label(theme.foreground(), "Level:");
                        let mut send_level = 0.5;
                        let _ = Knob::new(send_level).diameter(60.0).label("dB").show(
                            ui,
                            &mut send_level,
                            &theme,
                        );

                        ui.add_space(16.0);

                        // Pre/Post fader
                        ui.horizontal(|ui| {
                            ui.colored_label(theme.foreground(), "Routing:");
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
                            ui.colored_label(theme.foreground(), "Status:");
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

        MixerStripResponse {
            response: card_response.response,
            fader_changed: (self.fader_level - old_fader).abs() > 0.001,
            pan_changed: (self.pan - old_pan).abs() > 0.001,
            mute_toggled: self.muted != old_mute,
            solo_toggled: self.soloed != old_solo,
            record_toggled: self.record_armed != old_record,
            monitor_toggled: self.input_monitoring != old_monitor,
            sends_clicked,
            input_routing_clicked,
            output_routing_clicked,
        }
    }
}

impl Default for MixerStrip {
    fn default() -> Self {
        Self::new("Channel")
    }
}
