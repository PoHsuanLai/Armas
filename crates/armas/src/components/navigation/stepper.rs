//! Stepper Component
//!
//! Step-by-step progress indicator for multi-step workflows

use crate::ext::ArmasContextExt;
use crate::Theme;
use egui::{vec2, Color32, Pos2, Stroke, Ui};

/// A single step in the stepper
#[derive(Clone)]
pub struct Step {
    /// Step label
    pub label: String,
    /// Optional description
    pub description: Option<String>,
    /// Optional icon/emoji
    pub icon: Option<String>,
}

impl Step {
    /// Create a new step
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: None,
            icon: None,
        }
    }

    /// Set a description for the step
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set an icon for the step
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Stepper orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepperOrientation {
    /// Horizontal stepper
    Horizontal,
    /// Vertical stepper
    Vertical,
}

/// Stepper component for multi-step workflows
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::Stepper;
///
/// let current_step = 1;
/// let response = Stepper::new()
///     .clickable(true)
///     .show(ui, current_step, |stepper| {
///         stepper.step("Account").description("Create your account");
///         stepper.step("Profile").description("Set up your profile");
///         stepper.step("Complete").description("Finish setup");
///     });
/// # }
/// ```
pub struct Stepper {
    orientation: StepperOrientation,
    clickable: bool,
    show_numbers: bool,
}

impl Stepper {
    /// Create a new stepper
    pub fn new() -> Self {
        Self {
            orientation: StepperOrientation::Horizontal,
            clickable: false,
            show_numbers: true,
        }
    }


    /// Set the orientation
    pub fn orientation(mut self, orientation: StepperOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Make steps clickable
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    /// Show or hide step numbers
    pub fn show_numbers(mut self, show: bool) -> Self {
        self.show_numbers = show;
        self
    }

    /// Show the stepper with closure-based API
    pub fn show<R>(
        self,
        ui: &mut Ui,
        current_step: usize,
        content: impl FnOnce(&mut StepperBuilder) -> R,
    ) -> StepperResponse {
        let theme = ui.ctx().armas_theme();
        let mut response = StepperResponse { clicked_step: None };

        // Build steps using closure
        let mut builder = StepperBuilder {
            steps: Vec::new(),
            step_index: 0,
        };
        content(&mut builder);
        let steps = builder.steps;

        match self.orientation {
            StepperOrientation::Horizontal => {
                Self::show_horizontal(&steps, ui, &theme, current_step, &mut response, self.clickable, self.show_numbers);
            }
            StepperOrientation::Vertical => {
                Self::show_vertical(&steps, ui, &theme, current_step, &mut response, self.clickable, self.show_numbers);
            }
        }

        response
    }

    fn show_horizontal(
        steps: &[StepData],
        ui: &mut Ui,
        theme: &Theme,
        current_step: usize,
        response: &mut StepperResponse,
        clickable: bool,
        show_numbers: bool,
    ) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            for (idx, step) in steps.iter().enumerate() {
                let is_completed = idx < current_step;
                let is_current = idx == current_step;
                let _is_future = idx > current_step;

                // Step circle
                let circle_size = 40.0;
                let (rect, step_response) = ui.allocate_exact_size(
                    vec2(circle_size, circle_size),
                    if clickable {
                        egui::Sense::click()
                    } else {
                        egui::Sense::hover()
                    },
                );

                if step_response.clicked() && clickable {
                    response.clicked_step = Some(idx);
                }

                let center = rect.center();
                let radius = circle_size / 2.0 - 2.0;

                // Circle background and border
                let (bg_color, border_color, text_color) = if is_completed {
                    (theme.primary(), theme.primary(), Color32::WHITE)
                } else if is_current {
                    (theme.surface(), theme.primary(), theme.primary())
                } else {
                    (
                        theme.surface(),
                        theme.outline().linear_multiply(0.5),
                        theme.on_surface_variant().linear_multiply(0.5),
                    )
                };

                ui.painter().circle_filled(center, radius, bg_color);
                ui.painter()
                    .circle_stroke(center, radius, Stroke::new(2.0, border_color));

                // Step number or icon
                if is_completed && !show_numbers {
                    // Checkmark for completed steps
                    let check_text = "✓";
                    let font_id = egui::FontId::proportional(20.0);
                    let galley = ui.painter().layout_no_wrap(
                        check_text.to_string(),
                        font_id,
                        Color32::WHITE,
                    );
                    let text_pos = center - galley.size() / 2.0;
                    ui.painter().galley(text_pos, galley, Color32::WHITE);
                } else if let Some(icon) = &step.icon {
                    let font_id = egui::FontId::proportional(18.0);
                    let galley = ui
                        .painter()
                        .layout_no_wrap(icon.clone(), font_id, text_color);
                    let text_pos = center - galley.size() / 2.0;
                    ui.painter().galley(text_pos, galley, text_color);
                } else {
                    let number = (idx + 1).to_string();
                    let font_id = egui::FontId::proportional(16.0);
                    let galley = ui.painter().layout_no_wrap(number, font_id, text_color);
                    let text_pos = center - galley.size() / 2.0;
                    ui.painter().galley(text_pos, galley, text_color);
                }

                // Label below circle
                ui.add_space(8.0);
                ui.allocate_ui_with_layout(
                    vec2(120.0, 40.0),
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        let label_color = if is_current || is_completed {
                            theme.on_surface()
                        } else {
                            theme.on_surface_variant().linear_multiply(0.7)
                        };

                        ui.colored_label(label_color, &step.label);

                        if let Some(desc) = &step.description {
                            ui.add_space(2.0);
                            ui.colored_label(theme.on_surface_variant().linear_multiply(0.6), desc);
                        }
                    },
                );

                // Connector line to next step
                if idx < steps.len() - 1 {
                    ui.add_space(16.0);

                    let line_width = 60.0;
                    let line_height = 2.0;
                    let (line_rect, _) =
                        ui.allocate_exact_size(vec2(line_width, circle_size), egui::Sense::hover());

                    let line_y = line_rect.center().y;
                    let line_start = Pos2::new(line_rect.left(), line_y);
                    let line_end = Pos2::new(line_rect.right(), line_y);

                    let line_color = if is_completed {
                        theme.primary()
                    } else {
                        theme.outline().linear_multiply(0.3)
                    };

                    ui.painter()
                        .line_segment([line_start, line_end], Stroke::new(line_height, line_color));

                    ui.add_space(16.0);
                }
            }
        });
    }

    fn show_vertical(
        steps: &[StepData],
        ui: &mut Ui,
        theme: &Theme,
        current_step: usize,
        response: &mut StepperResponse,
        clickable: bool,
        show_numbers: bool,
    ) {
        for (idx, step) in steps.iter().enumerate() {
            let is_completed = idx < current_step;
            let is_current = idx == current_step;
            let _is_future = idx > current_step;

            ui.horizontal(|ui| {
                // Step circle
                let circle_size = 40.0;
                let (rect, step_response) = ui.allocate_exact_size(
                    vec2(circle_size, circle_size),
                    if clickable {
                        egui::Sense::click()
                    } else {
                        egui::Sense::hover()
                    },
                );

                if step_response.clicked() && clickable {
                    response.clicked_step = Some(idx);
                }

                let center = rect.center();
                let radius = circle_size / 2.0 - 2.0;

                let (bg_color, border_color, text_color) = if is_completed {
                    (theme.primary(), theme.primary(), Color32::WHITE)
                } else if is_current {
                    (theme.surface(), theme.primary(), theme.primary())
                } else {
                    (
                        theme.surface(),
                        theme.outline().linear_multiply(0.5),
                        theme.on_surface_variant().linear_multiply(0.5),
                    )
                };

                ui.painter().circle_filled(center, radius, bg_color);
                ui.painter()
                    .circle_stroke(center, radius, Stroke::new(2.0, border_color));

                // Step number or icon
                if is_completed && !show_numbers {
                    let check_text = "✓";
                    let font_id = egui::FontId::proportional(20.0);
                    let galley = ui.painter().layout_no_wrap(
                        check_text.to_string(),
                        font_id,
                        Color32::WHITE,
                    );
                    let text_pos = center - galley.size() / 2.0;
                    ui.painter().galley(text_pos, galley, Color32::WHITE);
                } else if let Some(icon) = &step.icon {
                    let font_id = egui::FontId::proportional(18.0);
                    let galley = ui
                        .painter()
                        .layout_no_wrap(icon.clone(), font_id, text_color);
                    let text_pos = center - galley.size() / 2.0;
                    ui.painter().galley(text_pos, galley, text_color);
                } else {
                    let number = (idx + 1).to_string();
                    let font_id = egui::FontId::proportional(16.0);
                    let galley = ui.painter().layout_no_wrap(number, font_id, text_color);
                    let text_pos = center - galley.size() / 2.0;
                    ui.painter().galley(text_pos, galley, text_color);
                }

                ui.add_space(16.0);

                // Label and description
                ui.vertical(|ui| {
                    let label_color = if is_current || is_completed {
                        theme.on_surface()
                    } else {
                        theme.on_surface_variant().linear_multiply(0.7)
                    };

                    ui.colored_label(label_color, &step.label);

                    if let Some(desc) = &step.description {
                        ui.add_space(2.0);
                        ui.colored_label(theme.on_surface_variant().linear_multiply(0.6), desc);
                    }
                });
            });

            // Connector line to next step
            if idx < steps.len() - 1 {
                ui.add_space(4.0);

                let line_height = 30.0;
                let (line_rect, _) =
                    ui.allocate_exact_size(vec2(40.0, line_height), egui::Sense::hover());

                let line_x = line_rect.left() + 20.0;
                let line_start = Pos2::new(line_x, line_rect.top());
                let line_end = Pos2::new(line_x, line_rect.bottom());

                let line_color = if is_completed {
                    theme.primary()
                } else {
                    theme.outline().linear_multiply(0.3)
                };

                ui.painter()
                    .line_segment([line_start, line_end], Stroke::new(2.0, line_color));

                ui.add_space(4.0);
            }
        }
    }
}

impl Default for Stepper {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for adding stepper steps
pub struct StepperBuilder {
    steps: Vec<StepData>,
    step_index: usize,
}

#[derive(Clone)]
struct StepData {
    label: String,
    description: Option<String>,
    icon: Option<String>,
}

impl StepperBuilder {
    /// Add a step
    pub fn step(&mut self, label: &str) -> StepItemBuilder<'_> {
        let step_data = StepData {
            label: label.to_string(),
            description: None,
            icon: None,
        };

        self.steps.push(step_data);
        let current_index = self.step_index;
        self.step_index += 1;

        StepItemBuilder {
            steps: &mut self.steps,
            step_index: current_index,
        }
    }
}

/// Builder for chaining step modifiers
pub struct StepItemBuilder<'a> {
    steps: &'a mut Vec<StepData>,
    step_index: usize,
}

impl<'a> StepItemBuilder<'a> {
    /// Set a description for the step
    pub fn description(self, description: &str) -> Self {
        if let Some(step) = self.steps.get_mut(self.step_index) {
            step.description = Some(description.to_string());
        }
        self
    }

    /// Set an icon for the step
    pub fn icon(self, icon: &str) -> Self {
        if let Some(step) = self.steps.get_mut(self.step_index) {
            step.icon = Some(icon.to_string());
        }
        self
    }
}

/// Response from a stepper
#[derive(Debug, Clone, Copy)]
pub struct StepperResponse {
    /// The step that was clicked (if clickable)
    pub clicked_step: Option<usize>,
}
