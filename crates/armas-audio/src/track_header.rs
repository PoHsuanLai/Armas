//! Track Header Component
//!
//! Header section for DAW timeline tracks with name, controls, and color indicator.

use armas::color::lerp_color;
use armas::theme::Theme;
use armas::{Button, ButtonVariant, Card, CardVariant};
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
    /// Track name was changed
    pub name_changed: bool,
    /// New track name (if changed)
    pub name: String,
    /// Mute button was clicked
    pub mute_clicked: bool,
    /// Solo button was clicked
    pub solo_clicked: bool,
    /// Record arm button was clicked
    pub arm_clicked: bool,
    /// Controls state (after any changes)
    pub controls: TrackControls,
    /// Collapse/expand button was clicked (for folder tracks)
    pub collapse_clicked: bool,
}

/// Track header component for DAW timeline
///
/// Shows track name, color indicator, and control buttons (mute, solo, record arm).
///
/// # Example
///
/// ```rust,no_run
/// use armas_audio::{TrackHeader, TrackControls};
///
/// fn ui(ui: &mut egui::Ui, theme: &armas::Theme) {
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
    /// Card background color
    card_color: Option<Color32>,
    /// Allow name editing
    editable: bool,
    /// Show control buttons
    show_controls: bool,
    /// Compact mode (smaller controls)
    compact: bool,
    /// Is this a folder track?
    is_folder: bool,
    /// Indentation level (for nested tracks)
    indent_level: usize,
}

impl TrackHeader {
    /// Create a new track header
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: None,
            width: 200.0,
            height: 60.0,
            color: None,
            parent_color: None,
            card_color: None,
            editable: true,
            show_controls: true,
            compact: false,
            is_folder: false,
            indent_level: 0,
        }
    }

    /// Set custom ID (important when using multiple track headers)
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the header width
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the header height
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set the track color
    #[must_use]
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the card background color
    #[must_use]
    pub fn card_color(mut self, color: Color32) -> Self {
        self.card_color = Some(color);
        self
    }

    /// Set the parent track color (for nested folder gradient interpolation)
    #[must_use]
    pub fn parent_color(mut self, color: Color32) -> Self {
        self.parent_color = Some(color);
        self
    }

    /// Set whether the name is editable
    #[must_use]
    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = editable;
        self
    }

    /// Set whether to show control buttons
    #[must_use]
    pub fn show_controls(mut self, show: bool) -> Self {
        self.show_controls = show;
        self
    }

    /// Set compact mode (smaller controls)
    #[must_use]
    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    /// Set whether this is a folder track
    #[must_use]
    pub fn is_folder(mut self, is_folder: bool) -> Self {
        self.is_folder = is_folder;
        self
    }

    /// Set indentation level for nested tracks
    #[must_use]
    pub fn indent_level(mut self, level: usize) -> Self {
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
        let track_color = self.color.unwrap_or(theme.primary());
        let button_size = if self.compact { 20.0 } else { 24.0 };
        let spacing = if self.compact { 2.0 } else { 4.0 };
        let color_bar_width = if self.compact { 3.0 } else { 4.0 };
        let indent_pixels = (self.indent_level as f32) * 16.0;

        let mut name_changed = false;
        let mut mute_clicked = false;
        let mut solo_clicked = false;
        let mut arm_clicked = false;
        let mut collapse_clicked = false;

        // Get actual egui measurements
        let text_height = ui.text_style_height(&egui::TextStyle::Body);
        let content_spacing = if self.show_controls {
            spacing / 2.0
        } else {
            0.0
        };
        let buttons_height = if self.show_controls { button_size } else { 0.0 };

        let content_height = text_height + content_spacing + buttons_height;

        // Calculate padding for left/right only
        let horizontal_padding = 8.0;

        let mut card = Card::new()
            .variant(CardVariant::Filled)
            .width(self.width)
            .height(self.height)
            .inner_margin(0.0); // No card padding

        // Apply custom card color if provided
        if let Some(color) = self.card_color {
            card = card.fill(color);
        }

        let card_response = card.show(ui, theme, |ui| {
            // Allocate exact size first (like TimelineTrack does)
            let (track_rect, _) =
                ui.allocate_exact_size(Vec2::new(self.width, self.height), Sense::hover());

            // Calculate vertical centering
            let content_y = track_rect.min.y + (self.height - content_height) / 2.0;

            // Create a scoped UI within a sub-rect for the content
            let content_rect = egui::Rect::from_min_size(
                egui::Pos2::new(track_rect.min.x + horizontal_padding, content_y),
                Vec2::new(self.width - horizontal_padding * 2.0, content_height),
            );

            ui.scope_builder(
                egui::UiBuilder::new()
                    .max_rect(content_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Min)),
                |ui| {
                    // Add indentation space for nested tracks
                    if self.indent_level > 0 {
                        ui.add_space(indent_pixels);
                    }

                    // Color indicator bar with glassmorphism and subtle glow
                    let (rect, response) = ui.allocate_exact_size(
                        Vec2::new(color_bar_width, content_height),
                        Sense::click(),
                    );

                    let painter = ui.painter();

                    if self.is_folder {
                        // Folder track: Enhanced visual with gradient and stronger glow

                        // Vertical gradient: parent color (or theme primary) → track color
                        // Root folders: primary → self
                        // Child folders: parent → self
                        let gradient_start = self.parent_color.unwrap_or(theme.primary());
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

                        // Draw gradient using lerp_color interpolation
                        let num_steps = 10;
                        let step_height = rect.height() / num_steps as f32;
                        for i in 0..num_steps {
                            let t = i as f32 / (num_steps - 1) as f32;
                            let step_color = lerp_color(top_color, bottom_color, t);

                            let step_rect = egui::Rect::from_min_max(
                                egui::Pos2::new(rect.min.x, rect.min.y + i as f32 * step_height),
                                egui::Pos2::new(
                                    rect.max.x,
                                    rect.min.y + (i + 1) as f32 * step_height,
                                ),
                            );
                            painter.rect_filled(step_rect, 0.0, step_color);
                        }

                        // Stronger glow for folder tracks
                        let glow_alpha: u8 = 40;
                        for i in 0..5 {
                            let inset = (i + 1) as f32 * 0.4;
                            let alpha = glow_alpha.saturating_sub((i * 8) as u8);
                            let inset_rect = rect.shrink(inset);
                            let glow_color = Color32::from_rgba_unmultiplied(255, 255, 255, alpha);
                            painter.rect_filled(inset_rect, 0.0, glow_color);
                        }

                        // Check for clicks on the color bar to toggle collapse
                        if response.clicked() {
                            *collapsed = !*collapsed;
                            collapse_clicked = true;
                        }
                    } else {
                        // Regular track: Simple glassmorphic color bar
                        let glass_color = Color32::from_rgba_unmultiplied(
                            track_color.r(),
                            track_color.g(),
                            track_color.b(),
                            100,
                        );
                        painter.rect_filled(rect, 0.0, glass_color);

                        // Subtle inner glow
                        let glow_alpha: u8 = 20;
                        for i in 0..3 {
                            let inset = (i + 1) as f32 * 0.5;
                            let alpha = glow_alpha.saturating_sub((i * 6) as u8);
                            let inset_rect = rect.shrink(inset);
                            let glow_color = Color32::from_rgba_unmultiplied(255, 255, 255, alpha);
                            painter.rect_filled(inset_rect, 0.0, glow_color);
                        }
                    }

                    ui.add_space(if self.compact { 4.0 } else { 6.0 });

                    ui.vertical(|ui| {
                        // Track name - editable text or label
                        if self.editable {
                            // Get card background color for text edit
                            let card_bg = self.card_color.unwrap_or(theme.muted());

                            // Calculate available width
                            let used_width = horizontal_padding * 2.0
                                + indent_pixels
                                + color_bar_width
                                + (if self.compact { 4.0 } else { 6.0 });
                            let available_width = (self.width - used_width).max(50.0);

                            let mut text_edit = TextEdit::singleline(name)
                                .desired_width(available_width)
                                .hint_text("Track Name")
                                .text_color(theme.foreground())
                                .background_color(card_bg);

                            // Apply custom ID if provided
                            if let Some(id) = self.id {
                                text_edit = text_edit.id(id);
                            }

                            let response = ui.add(text_edit);
                            if response.changed() {
                                name_changed = true;
                            }
                        } else {
                            ui.colored_label(theme.foreground(), name.as_str());
                        }

                        // Control buttons row
                        if self.show_controls {
                            ui.add_space(spacing / 2.0);

                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = spacing;

                                // Mute button
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

                                // Solo button
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

                                // Record arm button
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
                        }
                    });
                },
            )
        });

        TrackHeaderResponse {
            response: card_response.response,
            name_changed,
            name: name.clone(),
            mute_clicked,
            solo_clicked,
            arm_clicked,
            controls: *controls,
            collapse_clicked,
        }
    }
}

impl Default for TrackHeader {
    fn default() -> Self {
        Self::new()
    }
}
