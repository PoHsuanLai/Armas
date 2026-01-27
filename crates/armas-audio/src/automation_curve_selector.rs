//! Curve type selector toolbar
//!
//! UI for selecting different interpolation curve types

use armas::ext::ArmasContextExt;
use armas::theme::Theme;
use audio_automation::CurveType;
use egui::{Button, Response, Ui};

/// Toolbar for selecting automation curve types
pub struct CurveTypeSelector {
    selected: CurveType,
}

impl CurveTypeSelector {
    /// Create a new curve type selector
    pub fn new(selected: CurveType) -> Self {
        Self { selected }
    }

    /// Show the selector toolbar
    pub fn show(self, ui: &mut Ui) -> CurveTypeSelectorResponse {
        let theme = ui.ctx().armas_theme();

        let mut selected = self.selected;
        let mut changed = false;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = theme.spacing.sm;

            ui.label("Curve:");

            // Basic curves
            Self::curve_button(ui, &theme, "Linear", CurveType::Linear, &mut selected, &mut changed);
            Self::curve_button(ui, &theme, "Exponential", CurveType::Exponential, &mut selected, &mut changed);
            Self::curve_button(ui, &theme, "Logarithmic", CurveType::Logarithmic, &mut selected, &mut changed);
            Self::curve_button(ui, &theme, "S-Curve", CurveType::SCurve, &mut selected, &mut changed);
            Self::curve_button(ui, &theme, "Stepped", CurveType::Stepped, &mut selected, &mut changed);

            // Advanced easing
            ui.separator();

            Self::curve_button(ui, &theme, "Elastic", CurveType::Elastic, &mut selected, &mut changed);
            Self::curve_button(ui, &theme, "Bounce", CurveType::Bounce, &mut selected, &mut changed);
            Self::curve_button(ui, &theme, "Back", CurveType::Back, &mut selected, &mut changed);
            Self::curve_button(ui, &theme, "Circular", CurveType::Circular, &mut selected, &mut changed);
        });

        CurveTypeSelectorResponse {
            response: ui.label(""),
            selected,
            changed,
        }
    }

    /// Render a single curve type button
    fn curve_button(
        ui: &mut Ui,
        theme: &Theme,
        label: &str,
        curve_type: CurveType,
        selected: &mut CurveType,
        changed: &mut bool,
    ) {
        let is_selected = *selected == curve_type;

        let bg_color = if is_selected {
            theme.primary()
        } else {
            theme.muted()
        };

        let button = Button::new(label)
            .fill(bg_color)
            .stroke(egui::Stroke::new(1.0, theme.border()));

        if ui.add(button).clicked() {
            *selected = curve_type;
            *changed = true;
        }
    }
}

/// Response from curve type selector
pub struct CurveTypeSelectorResponse {
    /// Base egui response
    pub response: Response,
    /// Currently selected curve type
    pub selected: CurveType,
    /// Whether the selection changed this frame
    pub changed: bool,
}
