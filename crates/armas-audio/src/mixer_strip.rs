//! Mixer Strip Component
//!
//! Complete DAW-style mixer channel strip with sends, routing, inserts, pan,
//! mute/solo, meter, and fader.

use crate::{AudioMeter, Fader, Knob, Slot};
use armas_basic::components::basic::{Badge, Select, SelectOption};
use armas_basic::components::button::{Button, ButtonSize, ButtonVariant};
use armas_basic::components::cards::{Card, CardVariant};
use egui::{Color32, Id, Response, Ui, Vec2};

/// Mixer channel strip component
///
/// A complete mixer channel strip inspired by professional DAW interfaces.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # use armas_basic::Theme;
/// # fn example(ui: &mut Ui, theme: &Theme) {
/// use armas_audio::MixerStrip;
///
/// let mut strip = MixerStrip::new("Channel 1");
/// strip.show(ui, theme);
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
    /// Custom slot color (overrides auto-detection from effect name)
    pub color: Option<Color32>,
}

impl Insert {
    /// Create an empty insert slot
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            name: None,
            bypassed: false,
            color: None,
        }
    }

    /// Create an insert with a plugin
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            bypassed: false,
            color: None,
        }
    }

    /// Set a custom slot color
    #[must_use]
    pub const fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
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

/// Display mode for mixer strip layout
///
/// Controls which sections are visible and their sizing. Use `Auto` (default)
/// to let the strip adapt to available height automatically.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MixerStripMode {
    /// Automatically detect mode based on available height
    #[default]
    Auto,
    /// Full layout - 4 scroll rows per section
    Full,
    /// Standard layout - 3 scroll rows per section
    Standard,
    /// Compact layout - 2 scroll rows per section
    Compact,
    /// Minimal layout - 1 scroll row per section
    Minimal,
}

/// Internal resolved layout decisions for a single frame
struct ResolvedLayout {
    show_pan_label: bool,
    show_gain_display: bool,
    show_record_monitor: bool,
    /// Number of visible rows in scrollable sections (sends, inserts)
    scroll_rows: u8,
    button_size: ButtonSize,
    row_height: f32,
    section_gap: f32,
    knob_diameter: f32,
    meter_fader_height: f32,
}

impl ResolvedLayout {
    fn from_mode(mode: MixerStripMode, scale: f32) -> Self {
        match mode {
            MixerStripMode::Full => Self {
                show_pan_label: true,
                show_gain_display: true,
                show_record_monitor: true,
                scroll_rows: 4,

                button_size: ButtonSize::Xs,
                row_height: 22.0 * scale,
                section_gap: 2.0 * scale,
                knob_diameter: 32.0 * scale,
                meter_fader_height: 180.0 * scale,
            },
            MixerStripMode::Standard => Self {
                show_pan_label: true,
                show_gain_display: true,
                show_record_monitor: true,
                scroll_rows: 3,

                button_size: ButtonSize::Xs,
                row_height: 22.0 * scale,
                section_gap: 2.0 * scale,
                knob_diameter: 30.0 * scale,
                meter_fader_height: 160.0 * scale,
            },
            MixerStripMode::Compact => Self {
                show_pan_label: false,
                show_gain_display: true,
                show_record_monitor: false,
                scroll_rows: 2,

                button_size: ButtonSize::Xs,
                row_height: 20.0 * scale,
                section_gap: 1.0 * scale,
                knob_diameter: 26.0 * scale,
                meter_fader_height: 140.0 * scale,
            },
            MixerStripMode::Minimal => Self {
                show_pan_label: false,
                show_gain_display: false,
                show_record_monitor: false,
                scroll_rows: 1,

                button_size: ButtonSize::Xs,
                row_height: 20.0 * scale,
                section_gap: 1.0 * scale,
                knob_diameter: 22.0 * scale,
                meter_fader_height: 120.0 * scale,
            },
            MixerStripMode::Auto => unreachable!("Auto should be resolved before calling from_mode"),
        }
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
    /// Send add button (+) clicked
    pub send_add_clicked: bool,
    /// Input routing changed this frame
    pub input_route_changed: bool,
    /// Output routing changed this frame
    pub output_route_changed: bool,
}

/// Mixer channel strip with fader, pan, meters, and routing controls
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
    /// Input routing selected value
    input_route: String,
    /// Input routing options
    input_options: Vec<SelectOption>,
    /// Output routing selected value
    output_route: String,
    /// Output routing options
    output_options: Vec<SelectOption>,
    /// Card background color
    card_color: Option<Color32>,
    /// Knob glow color (overrides `card_color` if set)
    knob_color: Option<Color32>,
    /// Meter color (overrides `card_color` if set)
    meter_color: Option<Color32>,
    /// Sends
    sends: Vec<Send>,
    /// Display mode
    mode: MixerStripMode,
    /// Section background colors: (sends, routing, inserts)
    sends_color: Option<Color32>,
    routing_color: Option<Color32>,
    inserts_color: Option<Color32>,
}

impl MixerStrip {
    /// Create a new mixer strip
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let id = Id::new(format!("mixer_strip_{name}"));

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
            input_route: "input_1".to_string(),
            input_options: vec![
                SelectOption::new("input_1", "Input 1"),
                SelectOption::new("input_2", "Input 2"),
            ],
            output_route: "main".to_string(),
            output_options: vec![
                SelectOption::new("main", "Main"),
                SelectOption::new("bus_a", "Bus A"),
            ],
            card_color: None,
            knob_color: None,
            meter_color: None,
            sends: vec![],
            mode: MixerStripMode::Auto,
            sends_color: None,
            routing_color: None,
            inserts_color: None,
        }
    }

    /// Set strip width
    #[must_use]
    pub const fn width(mut self, width: f32) -> Self {
        self.width = width.max(60.0);
        self
    }

    /// Set scale factor for zoom (1.0 = 100%, 0.8 = 80%, 1.2 = 120%)
    #[must_use]
    pub const fn scale(mut self, scale: f32) -> Self {
        self.scale = scale.clamp(0.5, 2.0); // Clamp between 50% and 200%
        self
    }

    /// Set display mode
    ///
    /// Controls scroll area size and component sizing:
    /// - `Auto` (default): adapts to available height
    /// - `Full`: 4 scroll rows, largest knob/meter
    /// - `Standard`: 3 scroll rows
    /// - `Compact`: 2 scroll rows
    /// - `Minimal`: 1 scroll row, smallest knob/meter
    #[must_use]
    pub const fn mode(mut self, mode: MixerStripMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set fader level
    #[must_use]
    pub const fn fader_level(mut self, level: f32) -> Self {
        self.fader_level = level.clamp(0.0, 1.0);
        self
    }

    /// Set pan value
    #[must_use]
    pub const fn pan(mut self, pan: f32) -> Self {
        self.pan = pan.clamp(-1.0, 1.0);
        self
    }

    /// Set meter level
    #[must_use]
    pub const fn meter_level(mut self, level: f32) -> Self {
        self.meter_level = level.clamp(0.0, 1.0);
        self
    }

    /// Set card background color
    #[must_use]
    pub const fn card_color(mut self, color: Color32) -> Self {
        self.card_color = Some(color);
        self
    }

    /// Set knob glow color (overrides `card_color` for knob)
    #[must_use]
    pub const fn knob_color(mut self, color: Color32) -> Self {
        self.knob_color = Some(color);
        self
    }

    /// Set meter color (overrides `card_color` for meter)
    #[must_use]
    pub const fn meter_color(mut self, color: Color32) -> Self {
        self.meter_color = Some(color);
        self
    }

    /// Set sends section background color
    #[must_use]
    pub const fn sends_color(mut self, color: Color32) -> Self {
        self.sends_color = Some(color);
        self
    }

    /// Set routing section background color
    #[must_use]
    pub const fn routing_color(mut self, color: Color32) -> Self {
        self.routing_color = Some(color);
        self
    }

    /// Set inserts section background color
    #[must_use]
    pub const fn inserts_color(mut self, color: Color32) -> Self {
        self.inserts_color = Some(color);
        self
    }

    /// Set mute state
    #[must_use]
    pub const fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    /// Set solo state
    #[must_use]
    pub const fn soloed(mut self, soloed: bool) -> Self {
        self.soloed = soloed;
        self
    }

    /// Set record arm state
    #[must_use]
    pub const fn record_armed(mut self, armed: bool) -> Self {
        self.record_armed = armed;
        self
    }

    /// Set input monitoring state
    #[must_use]
    pub const fn input_monitoring(mut self, monitoring: bool) -> Self {
        self.input_monitoring = monitoring;
        self
    }

    /// Set insert slots
    #[must_use]
    pub fn inserts(mut self, inserts: Vec<Insert>) -> Self {
        self.inserts = inserts;
        self
    }

    /// Set sends
    #[must_use]
    pub fn sends(mut self, sends: Vec<Send>) -> Self {
        self.sends = sends;
        self
    }

    /// Set input route options and selected value
    #[must_use]
    pub fn input_route(mut self, selected: impl Into<String>, options: Vec<SelectOption>) -> Self {
        self.input_route = selected.into();
        self.input_options = options;
        self
    }

    /// Set output route options and selected value
    #[must_use]
    pub fn output_route(mut self, selected: impl Into<String>, options: Vec<SelectOption>) -> Self {
        self.output_route = selected.into();
        self.output_options = options;
        self
    }

    // Getter methods

    /// Get the channel name
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the fader level (0.0 to 1.0)
    #[must_use]
    pub const fn get_fader_level(&self) -> f32 {
        self.fader_level
    }

    /// Get the pan value (-1.0 to 1.0)
    #[must_use]
    pub const fn get_pan(&self) -> f32 {
        self.pan
    }

    /// Get mute state
    #[must_use]
    pub const fn is_muted(&self) -> bool {
        self.muted
    }

    /// Get solo state
    #[must_use]
    pub const fn is_soloed(&self) -> bool {
        self.soloed
    }

    /// Get record arm state
    #[must_use]
    pub const fn is_record_armed(&self) -> bool {
        self.record_armed
    }

    /// Get input monitoring state
    #[must_use]
    pub const fn is_input_monitoring(&self) -> bool {
        self.input_monitoring
    }

    /// Get meter level
    #[must_use]
    pub const fn get_meter_level(&self) -> f32 {
        self.meter_level
    }

    /// Get inserts
    #[must_use]
    pub fn get_inserts(&self) -> &[Insert] {
        &self.inserts
    }

    /// Get input route selected value
    #[must_use]
    pub fn get_input_route(&self) -> &str {
        &self.input_route
    }

    /// Get output route selected value
    #[must_use]
    pub fn get_output_route(&self) -> &str {
        &self.output_route
    }

    /// Get sends
    #[must_use]
    pub fn get_sends(&self) -> &[Send] {
        &self.sends
    }

    /// Resolve the effective mode from Auto or an explicit mode
    fn resolve_mode(&self, available_height: f32) -> MixerStripMode {
        match self.mode {
            MixerStripMode::Auto => {
                let normalized = available_height / self.scale;
                if normalized >= 480.0 {
                    MixerStripMode::Full
                } else if normalized >= 380.0 {
                    MixerStripMode::Standard
                } else if normalized >= 280.0 {
                    MixerStripMode::Compact
                } else {
                    MixerStripMode::Minimal
                }
            }
            explicit => explicit,
        }
    }

    /// Convert fader position (0..1) to dB value using piecewise linear interpolation
    fn fader_to_db(fader_level: f32) -> f32 {
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

        let mut db = -60.0;
        for i in 0..points.len() - 1 {
            if fader_level >= points[i].0 && fader_level <= points[i + 1].0 {
                let t = (fader_level - points[i].0) / (points[i + 1].0 - points[i].0);
                db = points[i].1 + t * (points[i + 1].1 - points[i].1);
                break;
            }
        }
        db
    }

    /// Format pan value as display string (C, L50, R50, etc.)
    fn format_pan(pan: f32) -> String {
        if pan.abs() < 0.05 {
            "C".to_string() // Center
        } else if pan < 0.0 {
            format!("L{}", (pan.abs() * 100.0) as i32) // Left
        } else {
            format!("R{}", (pan * 100.0) as i32) // Right
        }
    }

    /// Render sends section (header button and badges)
    fn render_sends_section(
        &self,
        ui: &mut Ui,
        theme: &armas_basic::Theme,
        layout: &ResolvedLayout,
        button_width_full: f32,
    ) -> bool {
        let mut add_clicked = false;
        let scale = self.scale;

        let send_gap = 1.0;
        let rows = f32::from(layout.scroll_rows);
        let scroll_height = layout.row_height * rows + send_gap * (rows - 1.0).max(0.0);

        egui::ScrollArea::vertical()
            .id_salt(self.id.with("sends_scroll"))
            .max_height(scroll_height)
            .min_scrolled_height(scroll_height)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                if self.sends.is_empty() {
                    // Empty state: show "Sends +" button
                    if Button::new("Sends +")
                        .variant(ButtonVariant::Outline)
                        .size(layout.button_size)
                        .height(layout.row_height)
                        .min_width(button_width_full)
                        .show(ui, theme)
                        .clicked()
                    {
                        add_clicked = true;
                    }
                } else {
                    // Populated: show send badges + "+" button
                    for (i, send) in self.sends.iter().enumerate() {
                        if i > 0 {
                            ui.add_space(send_gap);
                        }
                        ui.allocate_ui_with_layout(
                            Vec2::new(button_width_full, layout.row_height),
                            egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                            |ui| {
                                Badge::new(&send.name)
                                    .corner_radius(4.0 * scale)
                                    .size(11.0 * scale)
                                    .height(layout.row_height)
                                    .min_width(button_width_full)
                                    .show(ui, theme);
                            },
                        );
                    }

                    ui.add_space(send_gap);
                    if Button::new("+")
                        .variant(ButtonVariant::Outline)
                        .size(layout.button_size)
                        .height(layout.row_height)
                        .min_width(button_width_full)
                        .show(ui, theme)
                        .clicked()
                    {
                        add_clicked = true;
                    }
                }
            });

        add_clicked
    }

    /// Render routing selects (input and output)
    fn render_routing_section(
        &mut self,
        ui: &mut Ui,
        theme: &armas_basic::Theme,
        layout: &ResolvedLayout,
        button_width_full: f32,
    ) -> (bool, bool) {
        let mut input_changed = false;
        let mut output_changed = false;

        let input_resp = Select::new(self.input_options.clone())
            .id(self.id.with("input_route_select"))
            .selected(&self.input_route)
            .placeholder("Input...")
            .width(button_width_full)
            .height(layout.row_height)
            .searchable(false)
            .show(ui, theme);

        if input_resp.changed {
            if let Some(val) = input_resp.selected_value {
                self.input_route = val;
                input_changed = true;
            }
        }

        let output_resp = Select::new(self.output_options.clone())
            .id(self.id.with("output_route_select"))
            .selected(&self.output_route)
            .placeholder("Output...")
            .width(button_width_full)
            .height(layout.row_height)
            .searchable(false)
            .show(ui, theme);

        if output_resp.changed {
            if let Some(val) = output_resp.selected_value {
                self.output_route = val;
                output_changed = true;
            }
        }

        (input_changed, output_changed)
    }

    /// Render insert slots
    fn render_inserts(
        &self,
        ui: &mut Ui,
        theme: &armas_basic::Theme,
        layout: &ResolvedLayout,
        button_width_full: f32,
    ) {
        let slot_height = (layout.row_height * 0.72).round();
        let slot_gap = 1.0;
        let rows = f32::from(layout.scroll_rows);
        let scroll_height = slot_height * rows + slot_gap * (rows - 1.0).max(0.0);

        egui::ScrollArea::vertical()
            .id_salt(self.id.with("inserts_scroll"))
            .max_height(scroll_height)
            .min_scrolled_height(scroll_height)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                for (i, insert) in self.inserts.iter().enumerate() {
                    if i > 0 {
                        ui.add_space(slot_gap);
                    }
                    let mut slot = insert.name.as_ref().map_or_else(
                        || Slot::new().width(button_width_full).height(slot_height),
                        |name| {
                            Slot::new()
                                .effect(name)
                                .width(button_width_full)
                                .height(slot_height)
                        },
                    );
                    if let Some(color) = insert.color {
                        slot = slot.color(color);
                    }

                    slot.show(ui, theme);
                }
            });
    }

    /// Render pan knob (without label)
    fn render_pan_knob(
        &mut self,
        ui: &mut Ui,
        theme: &armas_basic::Theme,
        knob_diameter: f32,
    ) {
        // Load pan state
        let pan_state_id = self.id.with("pan_state");
        let current_pan = ui
            .ctx()
            .data_mut(|d| d.get_temp(pan_state_id).unwrap_or(self.pan));

        // Convert pan from -1..1 to 0..1 for knob
        let mut pan_knob_value = f32::midpoint(current_pan, 1.0);

        let glow_color = self.knob_color.unwrap_or_else(|| theme.primary());
        let knob = Knob::new(pan_knob_value)
            .diameter(knob_diameter)
            .show_value(false)
            .glow_color(glow_color);

        let knob_resp = knob.show(ui, &mut pan_knob_value, theme);
        self.pan = knob_resp.value.mul_add(2.0, -1.0).clamp(-1.0, 1.0);

        // Save pan state
        ui.ctx().data_mut(|d| {
            d.insert_temp(pan_state_id, self.pan);
        });
    }

    /// Render M/S/R/I button grid
    fn render_control_buttons(
        &mut self,
        ui: &mut Ui,
        theme: &armas_basic::Theme,
        layout: &ResolvedLayout,
        button_width_full: f32,
    ) {
        let button_width_grid =
            (button_width_full - 1.0) / 2.0;

        // First row: Mute and Solo
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;

            if Button::new("M")
                .variant(ButtonVariant::Outline)
                .size(layout.button_size)
                .height(layout.row_height)
                .min_width(button_width_grid)
                .show(ui, theme)
                .clicked()
            {
                self.muted = !self.muted;
            }

            if Button::new("S")
                .variant(ButtonVariant::Outline)
                .size(layout.button_size)
                .height(layout.row_height)
                .min_width(button_width_grid)
                .show(ui, theme)
                .clicked()
            {
                self.soloed = !self.soloed;
            }
        });

        // Second row: Record and Input Monitor (hidden in compact/minimal)
        if layout.show_record_monitor {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 1.0;

                if Button::new("R")
                    .variant(ButtonVariant::Outline)
                    .size(layout.button_size)
                    .height(layout.row_height)
                    .min_width(button_width_grid)
                    .show(ui, theme)
                    .clicked()
                {
                    self.record_armed = !self.record_armed;
                }

                if Button::new("I")
                    .variant(ButtonVariant::Outline)
                    .size(layout.button_size)
                    .height(layout.row_height)
                    .min_width(button_width_grid)
                    .show(ui, theme)
                    .clicked()
                {
                    self.input_monitoring = !self.input_monitoring;
                }
            });
        }
    }

    /// Render meter and fader section (without gain display)
    fn render_meter_fader(
        &mut self,
        ui: &mut Ui,
        theme: &armas_basic::Theme,
        meter_width: f32,
        fader_width: f32,
        meter_fader_height: f32,
    ) {
        // Meter and fader side by side
        ui.horizontal(|ui| {
            let meter_color = self.meter_color.unwrap_or_else(|| theme.primary());
            let meter = AudioMeter::new(self.meter_level)
                .width(meter_width)
                .height(meter_fader_height)
                .scale_left()
                .color_range(Color32::from_rgb(0, 0, 0), meter_color);

            let _ = meter.show(ui, theme);

            let fader_resp = Fader::new(self.fader_level)
                .id(self.id.with("fader"))
                .size(fader_width, meter_fader_height)
                .show(ui, theme);
            self.fader_level = fader_resp.value;
        });
    }

    /// Render gain display (dB value)
    fn render_gain_display(&self, ui: &mut Ui, theme: &armas_basic::Theme) {
        // Load current fader level from state
        let current_fader_level = {
            let state_id = self.id.with("fader").with("fader_state");
            ui.ctx()
                .data_mut(|d| d.get_temp(state_id).unwrap_or(self.fader_level))
        };

        let db_value = Self::fader_to_db(current_fader_level);
        ui.colored_label(theme.foreground(), format!("{db_value:+.1}"));
    }

    /// Show the mixer strip
    pub fn show(&mut self, ui: &mut Ui, theme: &armas_basic::Theme) -> MixerStripResponse {
        let scale = self.scale;

        // Resolve display mode
        let effective_mode = self.resolve_mode(ui.available_height());
        let layout = ResolvedLayout::from_mode(effective_mode, scale);

        // Track changes this frame
        let old_fader = self.fader_level;
        let old_pan = self.pan;
        let old_mute = self.muted;
        let old_solo = self.soloed;
        let old_record = self.record_armed;
        let old_monitor = self.input_monitoring;

        // Apply scale to dimensions
        let scaled_width = self.width * scale;
        let meter_width = 12.0 * scale;
        let fader_width = 30.0 * scale;
        let button_width_full = 4.0f32.mul_add(-scale, scaled_width);

        let mut send_add_clicked = false;
        let mut input_route_changed = false;
        let mut output_route_changed = false;

        let default_color = Color32::from_rgb(28, 28, 30);
        let card_response = Card::new()
            .variant(CardVariant::Filled)
            .width(scaled_width)
            .corner_radius(8.0 * scale)
            .inner_margin(2.0 * scale)
            .fill(self.card_color.unwrap_or(default_color))
            .show(ui, theme, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.y = 1.0;
                    ui.add_space(layout.section_gap);

                    let section_padding = 3.0 * scale;
                    let section_content_width = button_width_full - section_padding * 2.0;
                    let section_radius = 4.0 * scale;

                    let section_margin = egui::Margin {
                        left: section_padding as i8,
                        right: section_padding as i8,
                        top: (section_padding * 0.5) as i8,
                        bottom: (section_padding * 0.5) as i8,
                    };

                    // Per-section backgrounds
                    let default_bg = theme.card().gamma_multiply(0.6);
                    let sends_bg = self.sends_color.unwrap_or(default_bg);
                    let routing_bg = self.routing_color.unwrap_or(default_bg);
                    let inserts_bg = self.inserts_color.unwrap_or(default_bg);

                    // Sends section
                    egui::Frame::NONE
                        .fill(sends_bg)
                        .corner_radius(section_radius)
                        .inner_margin(section_margin)
                        .show(ui, |ui| {
                            send_add_clicked =
                                self.render_sends_section(ui, theme, &layout, section_content_width);
                        });

                    // Routing selects
                    egui::Frame::NONE
                        .fill(routing_bg)
                        .corner_radius(section_radius)
                        .inner_margin(section_margin)
                        .show(ui, |ui| {
                            let (input_changed, output_changed) =
                                self.render_routing_section(ui, theme, &layout, section_content_width);
                            input_route_changed = input_changed;
                            output_route_changed = output_changed;
                        });

                    // Insert slots
                    egui::Frame::NONE
                        .fill(inserts_bg)
                        .corner_radius(section_radius)
                        .inner_margin(section_margin)
                        .show(ui, |ui| {
                            self.render_inserts(ui, theme, &layout, section_content_width);
                        });

                    // Pan control
                    ui.add_space(layout.section_gap);
                    self.render_pan_knob(ui, theme, layout.knob_diameter);

                    // Pan label
                    if layout.show_pan_label {
                        ui.colored_label(
                            theme.muted_foreground(),
                            Self::format_pan(self.pan),
                        );
                    }

                    // M/S/R/I buttons
                    self.render_control_buttons(ui, theme, &layout, button_width_full);

                    // Gain display (dB)
                    if layout.show_gain_display {
                        self.render_gain_display(ui, theme);
                    }

                    // Meter and fader
                    self.render_meter_fader(
                        ui,
                        theme,
                        meter_width,
                        fader_width,
                        layout.meter_fader_height,
                    );

                    // Channel name
                    ui.colored_label(theme.foreground(), &self.name);
                });
            });

        MixerStripResponse {
            response: card_response.response,
            fader_changed: (self.fader_level - old_fader).abs() > 0.001,
            pan_changed: (self.pan - old_pan).abs() > 0.001,
            mute_toggled: self.muted != old_mute,
            solo_toggled: self.soloed != old_solo,
            record_toggled: self.record_armed != old_record,
            monitor_toggled: self.input_monitoring != old_monitor,
            send_add_clicked,
            input_route_changed,
            output_route_changed,
        }
    }
}

impl Default for MixerStrip {
    fn default() -> Self {
        Self::new("Channel")
    }
}
