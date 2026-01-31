//! Track Header Component
//!
//! Header section for DAW timeline tracks with name, controls, and color indicator.

use armas_basic::color::lerp_color;
use armas_basic::theme::Theme;
use armas_basic::{Button, ButtonVariant, Card, CardVariant};
use egui::{Color32, Response, Sense, TextEdit, Ui, Vec2};

/// Track control button state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TrackControls {
    /// Track is muted
    pub muted: bool,
    /// Track is soloed
    pub soloed: bool,
    /// Track is record-armed
    pub armed: bool,
}

/// Response from track header interaction
#[derive(Debug, Clone)]
pub struct TrackHeaderResponse {
    /// The egui response for the entire header
    pub response: Response,
    /// Mute button was clicked
    pub mute_clicked: bool,
    /// Solo button was clicked
    pub solo_clicked: bool,
    /// Record arm button was clicked
    pub arm_clicked: bool,
    /// Collapse/expand button was clicked (for folder tracks)
    pub collapse_clicked: bool,
}

/// Track header component for DAW timeline
///
/// Shows track name, color indicator, and control buttons (mute, solo, record arm).
///
/// # Example
///
/// ```rust,ignore
/// use armas_audio::{TrackHeader, TrackControls};
///
/// fn ui(ui: &mut egui::Ui, theme: &armas_basic::Theme) {
///     let mut name = "Audio 1".to_string();
///     let mut controls = TrackControls::default();
///     let mut collapsed = false;
///
///     let response = TrackHeader::new()
///         .width(200.0)
///         .color(egui::Color32::from_rgb(100, 150, 255))
///         .show(ui, &mut name, &mut controls, &mut collapsed, theme);
///
///     if response.mute_clicked {
///         println!("Mute toggled!");
///     }
/// }
/// ```
pub struct TrackHeader {
    /// Optional ID for the header
    id: Option<egui::Id>,
    /// Width of the header
    width: f32,
    /// Height of the header
    height: f32,
    /// Track color indicator
    color: Option<Color32>,
    /// Parent track color (for gradient interpolation in nested folders)
    parent_color: Option<Color32>,
    /// Is this a folder track?
    is_folder: bool,
    /// Indentation level (for nested tracks)
    indent_level: usize,
}

impl TrackHeader {
    /// Create a new track header
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: None,
            width: 200.0,
            height: 60.0,
            color: None,
            parent_color: None,
            is_folder: false,
            indent_level: 0,
        }
    }

    /// Set custom ID (important when using multiple track headers)
    #[must_use]
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the header width
    #[must_use]
    pub const fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the header height
    #[must_use]
    pub const fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set the track color
    #[must_use]
    pub const fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the parent track color (for nested folder gradient interpolation)
    #[must_use]
    pub const fn parent_color(mut self, color: Color32) -> Self {
        self.parent_color = Some(color);
        self
    }

    /// Set whether this is a folder track
    #[must_use]
    pub const fn folder(mut self, is_folder: bool) -> Self {
        self.is_folder = is_folder;
        self
    }

    /// Set indentation level for nested tracks
    #[must_use]
    pub const fn indent_level(mut self, level: usize) -> Self {
        self.indent_level = level;
        self
    }

    /// Show the track header
    pub fn show(
        self,
        ui: &mut Ui,
        name: &mut String,
        controls: &mut TrackControls,
        collapsed: &mut bool,
        theme: &Theme,
    ) -> TrackHeaderResponse {
        let track_color = self.color.unwrap_or_else(|| theme.primary());
        let button_size = 24.0;
        let spacing = 4.0;
        let color_bar_width = 4.0;
        let indent_pixels = (self.indent_level as f32) * 16.0;

        let mut mute_clicked = false;
        let mut solo_clicked = false;
        let mut arm_clicked = false;
        let mut collapse_clicked = false;

        // Get actual egui measurements
        let text_height = ui.text_style_height(&egui::TextStyle::Body);
        let content_spacing = spacing / 2.0;
        let content_height = text_height + content_spacing + button_size;

        let horizontal_padding = 8.0;

        let card = Card::new()
            .variant(CardVariant::Filled)
            .width(self.width)
            .height(self.height)
            .inner_margin(0.0);

        let card_response = card.show(ui, theme, |ui| {
            let (track_rect, _) =
                ui.allocate_exact_size(Vec2::new(self.width, self.height), Sense::hover());

            let content_y = track_rect.min.y + (self.height - content_height) / 2.0;

            let content_rect = egui::Rect::from_min_size(
                egui::Pos2::new(track_rect.min.x + horizontal_padding, content_y),
                Vec2::new(self.width - horizontal_padding * 2.0, content_height),
            );

            ui.scope_builder(
                egui::UiBuilder::new()
                    .max_rect(content_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Min)),
                |ui| {
                    if self.indent_level > 0 {
                        ui.add_space(indent_pixels);
                    }

                    let (rect, response) = ui.allocate_exact_size(
                        Vec2::new(color_bar_width, content_height),
                        Sense::click(),
                    );

                    let painter = ui.painter();

                    if self.is_folder {
                        let gradient_start = self.parent_color.unwrap_or_else(|| theme.primary());
                        let top_color = Color32::from_rgba_unmultiplied(
                            gradient_start.r(),
                            gradient_start.g(),
                            gradient_start.b(),
                            120,
                        );
                        let bottom_color = Color32::from_rgba_unmultiplied(
                            track_color.r(),
                            track_color.g(),
                            track_color.b(),
                            120,
                        );

                        let num_steps = 10;
                        let step_height = rect.height() / num_steps as f32;
                        for i in 0..num_steps {
                            let t = i as f32 / (num_steps - 1) as f32;
                            let step_color = lerp_color(top_color, bottom_color, t);

                            let step_rect = egui::Rect::from_min_max(
                                egui::Pos2::new(
                                    rect.min.x,
                                    (i as f32).mul_add(step_height, rect.min.y),
                                ),
                                egui::Pos2::new(
                                    rect.max.x,
                                    ((i + 1) as f32).mul_add(step_height, rect.min.y),
                                ),
                            );
                            painter.rect_filled(step_rect, 0.0, step_color);
                        }

                        let glow_alpha: u8 = 40;
                        for i in 0..5 {
                            let inset = (i + 1) as f32 * 0.4;
                            let alpha = glow_alpha.saturating_sub((i * 8) as u8);
                            let inset_rect = rect.shrink(inset);
                            let glow_color = Color32::from_rgba_unmultiplied(255, 255, 255, alpha);
                            painter.rect_filled(inset_rect, 0.0, glow_color);
                        }

                        if response.clicked() {
                            *collapsed = !*collapsed;
                            collapse_clicked = true;
                        }
                    } else {
                        let glass_color = Color32::from_rgba_unmultiplied(
                            track_color.r(),
                            track_color.g(),
                            track_color.b(),
                            100,
                        );
                        painter.rect_filled(rect, 0.0, glass_color);

                        let glow_alpha: u8 = 20;
                        for i in 0..3 {
                            let inset = (i + 1) as f32 * 0.5;
                            let alpha = glow_alpha.saturating_sub((i * 6) as u8);
                            let inset_rect = rect.shrink(inset);
                            let glow_color = Color32::from_rgba_unmultiplied(255, 255, 255, alpha);
                            painter.rect_filled(inset_rect, 0.0, glow_color);
                        }
                    }

                    ui.add_space(6.0);

                    ui.vertical(|ui| {
                        // Track name - editable text
                        let card_bg = theme.muted();
                        let used_width = horizontal_padding * 2.0
                            + indent_pixels
                            + color_bar_width
                            + 6.0;
                        let available_width = (self.width - used_width).max(50.0);

                        let mut text_edit = TextEdit::singleline(name)
                            .desired_width(available_width)
                            .hint_text("Track Name")
                            .text_color(theme.foreground())
                            .background_color(card_bg);

                        if let Some(id) = self.id {
                            text_edit = text_edit.id(id);
                        }

                        ui.add(text_edit);

                        // Control buttons row
                        ui.add_space(spacing / 2.0);

                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = spacing;

                            let mute_variant = if controls.muted {
                                ButtonVariant::Default
                            } else {
                                ButtonVariant::Outline
                            };

                            if Button::new("M")
                                .variant(mute_variant)
                                .min_width(button_size)
                                .show(ui, theme)
                                .clicked()
                            {
                                controls.muted = !controls.muted;
                                mute_clicked = true;
                            }

                            let solo_variant = if controls.soloed {
                                ButtonVariant::Default
                            } else {
                                ButtonVariant::Outline
                            };

                            if Button::new("S")
                                .variant(solo_variant)
                                .min_width(button_size)
                                .show(ui, theme)
                                .clicked()
                            {
                                controls.soloed = !controls.soloed;
                                solo_clicked = true;
                            }

                            let arm_variant = if controls.armed {
                                ButtonVariant::Default
                            } else {
                                ButtonVariant::Outline
                            };

                            if Button::new("R")
                                .variant(arm_variant)
                                .min_width(button_size)
                                .show(ui, theme)
                                .clicked()
                            {
                                controls.armed = !controls.armed;
                                arm_clicked = true;
                            }
                        });
                    });
                },
            )
        });

        TrackHeaderResponse {
            response: card_response.response,
            mute_clicked,
            solo_clicked,
            arm_clicked,
            collapse_clicked,
        }
    }
}

impl Default for TrackHeader {
    fn default() -> Self {
        Self::new()
    }
}
