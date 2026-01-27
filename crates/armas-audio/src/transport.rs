//! Transport Control Component
//!
//! Professional DAW transport bar with playback controls, time display,
//! tempo, and loop/metronome controls.

use crate::icons::TransportIcon;
use armas::components::basic::input::{Input, InputVariant};
use armas::components::button::{ButtonVariant, IconButton};
use armas::components::cards::{Card, CardVariant};
use armas::theme::Theme;
use egui::{Align, Color32, Response, Ui};

/// Transport button visibility configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransportButtons {
    /// Show rewind/go-to-start button
    pub show_rewind: bool,
    /// Show play/pause button
    pub show_play: bool,
    /// Show stop button
    pub show_stop: bool,
    /// Show record button
    pub show_record: bool,
    /// Show loop toggle
    pub show_loop: bool,
    /// Show metronome toggle
    pub show_metronome: bool,
}

impl Default for TransportButtons {
    fn default() -> Self {
        Self {
            show_rewind: true,
            show_play: true,
            show_stop: false,
            show_record: true,
            show_loop: true,
            show_metronome: true,
        }
    }
}

/// Transport playback state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportState {
    /// Transport is stopped
    Stopped,
    /// Transport is playing
    Playing,
    /// Transport is paused
    Paused,
    /// Transport is recording
    Recording,
}

/// Response from the transport control
#[derive(Debug, Clone)]
pub struct TransportResponse {
    /// The UI response
    pub response: Response,
    /// Current playback state
    pub state: TransportState,
    /// Current time in seconds
    pub current_time: f64,
    /// Current tempo in BPM
    pub tempo: f32,
    /// Current time signature
    pub time_signature: (u8, u8),
    /// Loop enabled state
    pub loop_enabled: bool,
    /// Metronome enabled state
    pub metronome_enabled: bool,
    /// Play button clicked
    pub play_clicked: bool,
    /// Pause button clicked (when playing)
    pub pause_clicked: bool,
    /// Stop button clicked
    pub stop_clicked: bool,
    /// Record button clicked
    pub record_clicked: bool,
    /// Rewind (go to start) clicked
    pub rewind_clicked: bool,
    /// Fast forward clicked
    pub forward_clicked: bool,
    /// Loop toggle clicked
    pub loop_toggled: bool,
    /// Metronome toggle clicked
    pub metronome_toggled: bool,
    /// Tempo changed
    pub tempo_changed: bool,
}

/// Transport Control component
///
/// Professional DAW-style transport bar with playback controls, time display,
/// tempo controls, and toggles for loop and metronome.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # use armas::Theme;
/// # fn example(ui: &mut Ui, theme: &Theme) {
/// use armas_audio::TransportControl;
///
/// let transport = TransportControl::new()
///     .tempo(120.0)
///     .time_signature(4, 4);
///
/// let response = transport.show(ui, theme);
///
/// if response.play_clicked {
///     // Start playback
/// }
/// if response.stop_clicked {
///     // Stop playback
/// }
/// // Access current state from response
/// println!("Current tempo: {}", response.tempo);
/// println!("Current time: {}", response.current_time);
/// # }
/// ```
#[derive(Clone)]
pub struct TransportControl {
    /// Current playback state
    state: TransportState,
    /// Current time in seconds
    current_time: f64,
    /// Tempo in BPM
    tempo: f32,
    /// Time signature (numerator, denominator)
    time_signature: (u8, u8),
    /// Loop enabled
    loop_enabled: bool,
    /// Metronome enabled
    metronome_enabled: bool,
    /// Width of the transport
    width: Option<f32>,
    /// Button color
    button_color: Option<Color32>,
    /// Button visibility configuration
    buttons: TransportButtons,
}

impl TransportControl {
    /// Create a new transport control
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: TransportState::Stopped,
            current_time: 0.0,
            tempo: 120.0,
            time_signature: (4, 4),
            loop_enabled: false,
            metronome_enabled: false,
            width: None,
            button_color: None,
            buttons: TransportButtons::default(),
        }
    }

    /// Set the playback state
    #[must_use]
    pub fn state(mut self, state: TransportState) -> Self {
        self.state = state;
        self
    }

    /// Set the current time in seconds
    #[must_use]
    pub fn current_time(mut self, time: f64) -> Self {
        self.current_time = time;
        self
    }

    /// Set the tempo in BPM
    #[must_use]
    pub fn tempo(mut self, tempo: f32) -> Self {
        self.tempo = tempo;
        self
    }

    /// Set the time signature (numerator, denominator)
    #[must_use]
    pub fn time_signature(mut self, numerator: u8, denominator: u8) -> Self {
        self.time_signature = (numerator, denominator);
        self
    }

    /// Set loop enabled state
    #[must_use]
    pub fn loop_enabled(mut self, enabled: bool) -> Self {
        self.loop_enabled = enabled;
        self
    }

    /// Set metronome enabled state
    #[must_use]
    pub fn metronome_enabled(mut self, enabled: bool) -> Self {
        self.metronome_enabled = enabled;
        self
    }

    /// Set button color
    #[must_use]
    pub fn button_color(mut self, color: Color32) -> Self {
        self.button_color = Some(color);
        self
    }

    /// Set button visibility configuration
    #[must_use]
    pub fn buttons(mut self, buttons: TransportButtons) -> Self {
        self.buttons = buttons;
        self
    }

    /// Set the width of the transport (None = fill available width)
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Show the transport control
    pub fn show(mut self, ui: &mut Ui, theme: &Theme) -> TransportResponse {
        let mut play_clicked = false;
        let mut pause_clicked = false;
        let mut stop_clicked = false;
        let mut record_clicked = false;
        let mut rewind_clicked = false;
        let mut forward_clicked = false;
        let mut loop_toggled = false;
        let mut metronome_toggled = false;
        let mut tempo_changed = false;

        let card_response = Card::new()
            .variant(CardVariant::Elevated)
            .show(ui, theme, |ui| {
                ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                    ui.add_space(theme.spacing.md);

                    // Playback controls
                    self.render_playback_controls(
                        ui,
                        theme,
                        &mut play_clicked,
                        &mut pause_clicked,
                        &mut stop_clicked,
                        &mut rewind_clicked,
                        &mut forward_clicked,
                    );

                    ui.add_space(theme.spacing.lg);

                    // Time display
                    self.render_time_display(ui, theme);

                    ui.add_space(theme.spacing.lg);

                    // Tempo control
                    self.render_tempo_control(ui, theme, &mut tempo_changed);

                    ui.add_space(theme.spacing.md);

                    // Time signature display
                    self.render_time_signature(ui, theme);

                    ui.add_space(theme.spacing.lg);

                    // Loop and metronome controls
                    self.render_loop_controls(
                        ui,
                        theme,
                        &mut loop_toggled,
                        &mut metronome_toggled,
                        &mut record_clicked,
                    );

                    ui.add_space(theme.spacing.md);
                });
            });

        TransportResponse {
            response: card_response.response,
            state: self.state,
            current_time: self.current_time,
            tempo: self.tempo,
            time_signature: self.time_signature,
            loop_enabled: self.loop_enabled,
            metronome_enabled: self.metronome_enabled,
            play_clicked,
            pause_clicked,
            stop_clicked,
            record_clicked,
            rewind_clicked,
            forward_clicked,
            loop_toggled,
            metronome_toggled,
            tempo_changed,
        }
    }

    /// Render playback control buttons (rewind, play/pause, stop, forward)
    fn render_playback_controls(
        &mut self,
        ui: &mut Ui,
        theme: &Theme,
        play_clicked: &mut bool,
        pause_clicked: &mut bool,
        stop_clicked: &mut bool,
        rewind_clicked: &mut bool,
        forward_clicked: &mut bool,
    ) {
        ui.horizontal_centered(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;

            // Rewind to start
            if IconButton::new(TransportIcon::Rewind.data())
                .variant(ButtonVariant::Text)
                .size(24.0)
                .padding(4.0)
                .show(ui, theme)
                .clicked()
            {
                *rewind_clicked = true;
                self.current_time = 0.0;
            }

            // Play / Pause toggle
            let play_icon = match self.state {
                TransportState::Playing | TransportState::Recording => TransportIcon::Pause,
                _ => TransportIcon::Play,
            };

            if IconButton::new(play_icon.data())
                .variant(ButtonVariant::Text)
                .size(24.0)
                .padding(4.0)
                .show(ui, theme)
                .clicked()
            {
                self.handle_play_pause_click(play_clicked, pause_clicked);
            }

            // Stop
            if IconButton::new(TransportIcon::Stop.data())
                .variant(ButtonVariant::Text)
                .size(24.0)
                .padding(4.0)
                .show(ui, theme)
                .clicked()
            {
                *stop_clicked = true;
                self.state = TransportState::Stopped;
                self.current_time = 0.0;
            }

            // Fast forward
            if IconButton::new(TransportIcon::Forward.data())
                .variant(ButtonVariant::Text)
                .size(24.0)
                .padding(4.0)
                .show(ui, theme)
                .clicked()
            {
                *forward_clicked = true;
            }
        });
    }

    /// Render tempo control with BPM input
    fn render_tempo_control(&mut self, ui: &mut Ui, theme: &Theme, tempo_changed: &mut bool) {
        Card::new()
            .variant(CardVariant::Filled)
            .fill(theme.card())
            .inner_margin(8.0)
            .show(ui, theme, |ui| {
                ui.spacing_mut().item_spacing.x = 8.0;

                ui.label(
                    egui::RichText::new("BPM:")
                        .size(16.0)
                        .color(theme.foreground()),
                );

                let mut tempo_str = format!("{:.1}", self.tempo);
                let tempo_response = Input::new("")
                    .variant(InputVariant::Inline)
                    .width(60.0)
                    .font_size(16.0)
                    .text_color(theme.foreground())
                    .show(ui, &mut tempo_str, theme);

                if tempo_response.changed {
                    if let Ok(new_bpm) = tempo_str.parse::<f32>() {
                        if new_bpm > 0.0 && new_bpm <= 999.0 {
                            self.tempo = new_bpm;
                            *tempo_changed = true;
                        }
                    }
                }
            });
    }

    /// Render current time display
    fn render_time_display(&self, ui: &mut Ui, theme: &Theme) {
        Card::new()
            .variant(CardVariant::Filled)
            .fill(theme.card())
            .inner_margin(8.0)
            .show(ui, theme, |ui| {
                let minutes = (self.current_time / 60.0) as u32;
                let seconds = (self.current_time % 60.0) as u32;
                let millis = ((self.current_time % 1.0) * 1000.0) as u32;
                let time_str = format!("{minutes:02}:{seconds:02}.{millis:03}");

                ui.label(
                    egui::RichText::new(time_str)
                        .size(16.0)
                        .family(egui::FontFamily::Proportional)
                        .color(theme.foreground()),
                );
            });
    }

    /// Render time signature display
    fn render_time_signature(&self, ui: &mut Ui, theme: &Theme) {
        Card::new()
            .variant(CardVariant::Filled)
            .fill(theme.card())
            .inner_margin(8.0)
            .show(ui, theme, |ui| {
                let time_sig_str = format!("{}/{}", self.time_signature.0, self.time_signature.1);
                ui.label(
                    egui::RichText::new(time_sig_str)
                        .size(16.0)
                        .color(theme.foreground()),
                );
            });
    }

    /// Render loop, metronome, and record controls
    fn render_loop_controls(
        &mut self,
        ui: &mut Ui,
        theme: &Theme,
        loop_toggled: &mut bool,
        metronome_toggled: &mut bool,
        record_clicked: &mut bool,
    ) {
        ui.horizontal_centered(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;

            // Loop toggle
            if IconButton::new(TransportIcon::Loop.data())
                .variant(ButtonVariant::Text)
                .size(24.0)
                .padding(4.0)
                .show(ui, theme)
                .clicked()
            {
                self.loop_enabled = !self.loop_enabled;
                *loop_toggled = true;
            }

            // Metronome toggle
            if IconButton::new(TransportIcon::Metronome.data())
                .variant(ButtonVariant::Text)
                .size(24.0)
                .padding(4.0)
                .show(ui, theme)
                .clicked()
            {
                self.metronome_enabled = !self.metronome_enabled;
                *metronome_toggled = true;
            }

            // Record button
            if IconButton::new(TransportIcon::Record.data())
                .variant(ButtonVariant::Text)
                .size(24.0)
                .padding(4.0)
                .show(ui, theme)
                .clicked()
            {
                *record_clicked = true;
                self.handle_record_click();
            }
        });
    }

    /// Handle play/pause button interaction
    fn handle_play_pause_click(&mut self, play_clicked: &mut bool, pause_clicked: &mut bool) {
        match self.state {
            TransportState::Playing | TransportState::Recording => {
                *pause_clicked = true;
                self.state = TransportState::Paused;
            }
            _ => {
                *play_clicked = true;
                self.state = TransportState::Playing;
            }
        }
    }

    /// Handle record button interaction
    fn handle_record_click(&mut self) {
        if self.state == TransportState::Recording {
            self.state = TransportState::Playing;
        } else {
            self.state = TransportState::Recording;
        }
    }
}

impl Default for TransportControl {
    fn default() -> Self {
        Self::new()
    }
}
