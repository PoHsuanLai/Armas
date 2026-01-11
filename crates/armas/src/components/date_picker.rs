//! DatePicker Component
//!
//! Calendar date selection with input field and popover
//! Modern design inspired by shadcn/ui with refined styling

use crate::ext::ArmasContextExt;
use crate::{Input, Popover, PopoverPosition};
use egui::{vec2, Color32, Id, Sense, Ui};

/// A date value (year, month, day)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub year: i32,
    pub month: u32, // 1-12
    pub day: u32,   // 1-31
}

impl Date {
    /// Create a new date
    pub fn new(year: i32, month: u32, day: u32) -> Option<Self> {
        if month < 1 || month > 12 {
            return None;
        }
        let days_in_month = Self::days_in_month(year, month);
        if day < 1 || day > days_in_month {
            return None;
        }
        Some(Self { year, month, day })
    }

    /// Get today's date (using system time)
    pub fn today() -> Self {
        let now = std::time::SystemTime::now();
        let duration = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        let days = duration.as_secs() / 86400;

        // Simple date calculation (approximate)
        let mut year = 1970;
        let mut remaining_days = days as i32;

        loop {
            let year_days = if Self::is_leap_year(year) { 366 } else { 365 };
            if remaining_days < year_days {
                break;
            }
            remaining_days -= year_days;
            year += 1;
        }

        let mut month = 1;
        while month <= 12 {
            let month_days = Self::days_in_month(year, month) as i32;
            if remaining_days < month_days {
                break;
            }
            remaining_days -= month_days;
            month += 1;
        }

        Self {
            year,
            month,
            day: (remaining_days + 1) as u32,
        }
    }

    /// Check if a year is a leap year
    pub fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Get the number of days in a month
    pub fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 0,
        }
    }

    /// Get the day of week (0 = Sunday, 6 = Saturday)
    pub fn day_of_week(&self) -> u32 {
        // Zeller's congruence algorithm
        let mut m = self.month as i32;
        let mut y = self.year;

        if m < 3 {
            m += 12;
            y -= 1;
        }

        let k = y % 100;
        let j = y / 100;

        let h = (self.day as i32 + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
        ((h + 6) % 7) as u32
    }

    /// Format as YYYY-MM-DD
    pub fn format(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Parse from YYYY-MM-DD format
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return None;
        }

        let year = parts[0].parse().ok()?;
        let month = parts[1].parse().ok()?;
        let day = parts[2].parse().ok()?;

        Self::new(year, month, day)
    }

    /// Get month name
    pub fn month_name(&self) -> &'static str {
        match self.month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Unknown",
        }
    }
}

/// DatePicker component
///
/// # Example
///
/// ```rust,no_run
/// use armas::{DatePicker, Date};
///
/// let mut date_picker = DatePicker::new("birthday");
/// let mut selected_date = None;
///
/// date_picker.show(ui, &mut selected_date);
/// ```
pub struct DatePicker {
    _id: Id,
    popover: Popover,
    viewing_year: i32,
    viewing_month: u32,
    input_text: String,
    is_open: bool,
    placeholder: String,
    label: Option<String>,
}

impl DatePicker {
    /// Create a new date picker
    pub fn new(id: impl Into<Id>) -> Self {
        let today = Date::today();
        let id = id.into();
        Self {
            _id: id,
            popover: Popover::new(id.with("popover"))
                .position(PopoverPosition::Bottom)
                .width(280.0), // Smaller, tighter width (7 * 36px cells + 6 * 2px gaps + padding)
            viewing_year: today.year,
            viewing_month: today.month,
            input_text: String::new(),
            is_open: false,
            placeholder: "Select a date...".to_string(),
            label: None,
        }
    }

    /// Set the placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set a label for the date picker
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Show the date picker
    pub fn show(
        &mut self,
        ui: &mut Ui,
        selected_date: &mut Option<Date>,
    ) -> DatePickerResponse {
        let theme = ui.ctx().armas_theme();
        let mut response = DatePickerResponse { changed: false };

        // Update input text from selected date
        if let Some(date) = selected_date {
            self.input_text = date.format();
        }

        // Label
        if let Some(label) = &self.label {
            ui.label(
                egui::RichText::new(label)
                    .size(14.0)
                    .color(theme.on_surface()),
            );
            ui.add_space(4.0);
        }

        // Input field with calendar icon
        let input = Input::new(&self.placeholder)
            .with_left_icon("📅")
            .with_width(300.0);
        let input_response = input.show(ui, &mut self.input_text);
        let input_rect = input_response.rect;

        // Toggle popover on click
        if input_response.clicked() {
            self.is_open = !self.is_open;
        }

        // Try to parse input text
        if input_response.changed() {
            if let Some(date) = Date::parse(&self.input_text) {
                *selected_date = Some(date);
                self.viewing_year = date.year;
                self.viewing_month = date.month;
                response.changed = true;
            }
        }

        // Show calendar popover
        let today = Date::today();
        let mut date_clicked = None;
        let mut goto_today = false;
        let mut clear_date = false;
        let mut prev_month = false;
        let mut next_month = false;

        let viewing_year = self.viewing_year;
        let viewing_month = self.viewing_month;

        // Set popover open state externally
        self.popover.set_open(self.is_open);

        self.popover
            .show(ui.ctx(), &theme, input_rect, |ui| {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 8.0;
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    // Header with month/year navigation
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        // Previous month button
                        let prev_size = vec2(32.0, 32.0);
                        let (prev_rect, prev_response) = ui.allocate_exact_size(prev_size, Sense::click());

                        if ui.is_rect_visible(prev_rect) {
                            let hovered = prev_response.hovered();

                            // Background on hover
                            if hovered {
                                ui.painter().rect_filled(
                                    prev_rect,
                                    6.0,
                                    theme.surface_variant(),
                                );
                            }

                            // Chevron icon
                            ui.painter().text(
                                prev_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "‹",
                                egui::FontId::proportional(20.0),
                                if hovered { theme.on_surface() } else { theme.on_surface_variant() },
                            );
                        }

                        if prev_response.clicked() {
                            prev_month = true;
                        }

                        // Month/Year label
                        ui.with_layout(
                            egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                            |ui| {
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{} {}",
                                        Date::new(viewing_year, viewing_month, 1)
                                            .unwrap()
                                            .month_name(),
                                        viewing_year
                                    ))
                                    .size(14.0)
                                    .strong()
                                    .color(theme.on_surface()),
                                );
                            },
                        );

                        // Next month button
                        let next_size = vec2(32.0, 32.0);
                        let (next_rect, next_response) = ui.allocate_exact_size(next_size, Sense::click());

                        if ui.is_rect_visible(next_rect) {
                            let hovered = next_response.hovered();

                            // Background on hover
                            if hovered {
                                ui.painter().rect_filled(
                                    next_rect,
                                    6.0,
                                    theme.surface_variant(),
                                );
                            }

                            // Chevron icon
                            ui.painter().text(
                                next_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "›",
                                egui::FontId::proportional(20.0),
                                if hovered { theme.on_surface() } else { theme.on_surface_variant() },
                            );
                        }

                        if next_response.clicked() {
                            next_month = true;
                        }
                    });

                    ui.add_space(4.0);

                    // Weekday headers (manual layout for perfect alignment)
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 2.0;
                        for day in &["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"] {
                            ui.allocate_ui(vec2(36.0, 28.0), |ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.label(
                                        egui::RichText::new(*day)
                                            .size(12.0)
                                            .font(egui::FontId::new(12.0, egui::FontFamily::Name("Inter".into())))
                                            .color(theme.on_surface_variant()),
                                    );
                                });
                            });
                        }
                    });

                    ui.add_space(2.0);

                    // Calendar grid with custom day cells (manual layout for perfect alignment)
                    let first_day = Date::new(viewing_year, viewing_month, 1).unwrap();
                    let first_weekday = first_day.day_of_week();
                    let days_in_month = Date::days_in_month(viewing_year, viewing_month);

                    // Calculate previous/next month info
                    let (prev_year, prev_month) = if viewing_month == 1 {
                        (viewing_year - 1, 12)
                    } else {
                        (viewing_year, viewing_month - 1)
                    };
                    let (next_year, next_month) = if viewing_month == 12 {
                        (viewing_year + 1, 1)
                    } else {
                        (viewing_year, viewing_month + 1)
                    };
                    let prev_month_days = Date::days_in_month(prev_year, prev_month);

                    let cell_size = vec2(36.0, 36.0);
                    let mut day_counter = 1;
                    let _total_cells = 42; // 6 weeks * 7 days

                    // Render 6 rows
                    for row in 0..6 {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 2.0;

                            // Render 7 columns
                            for col in 0..7 {
                                let cell_index = row * 7 + col;

                                // Determine which day to show
                                let (day, is_current_month, actual_year, actual_month) = if cell_index < first_weekday {
                                    // Previous month
                                    let day = prev_month_days - (first_weekday - cell_index - 1);
                                    (day, false, prev_year, prev_month)
                                } else if day_counter <= days_in_month {
                                    // Current month
                                    let day = day_counter;
                                    day_counter += 1;
                                    (day, true, viewing_year, viewing_month)
                                } else {
                                    // Next month
                                    let day = day_counter - days_in_month;
                                    day_counter += 1;
                                    (day, false, next_year, next_month)
                                };

                                if is_current_month {
                                    // Current month day - full interactivity
                                    let date = Date::new(actual_year, actual_month, day).unwrap();
                                    let is_today = date == today;
                                    let is_selected = *selected_date == Some(date);

                                    let (rect, response) = ui.allocate_exact_size(cell_size, Sense::click());

                                    if ui.is_rect_visible(rect) {
                                        let hovered = response.hovered();

                                        // Determine colors based on state (Flowbite-inspired)
                                        let (bg_color, text_color, font_weight) = if is_selected {
                                            // Selected: primary background with white text, bold
                                            (
                                                Some(theme.primary()),
                                                Color32::WHITE,
                                                true, // bold
                                            )
                                        } else if is_today {
                                            // Today: turquoise background with white text
                                            (
                                                Some(Color32::from_rgb(0, 209, 178)), // hsl(171, 100%, 41%)
                                                Color32::WHITE,
                                                false,
                                            )
                                        } else if hovered {
                                            // Hover: darken surface slightly
                                            let surface = theme.surface_variant();
                                            let darkened = Color32::from_rgb(
                                                (surface.r() as f32 * 0.975) as u8,
                                                (surface.g() as f32 * 0.975) as u8,
                                                (surface.b() as f32 * 0.975) as u8,
                                            );
                                            (
                                                Some(darkened),
                                                theme.on_surface(),
                                                false,
                                            )
                                        } else {
                                            // Normal: transparent
                                            (None, theme.on_surface(), false)
                                        };

                                        // Background
                                        if let Some(bg) = bg_color {
                                            ui.painter().rect_filled(
                                                rect,
                                                4.0, // 4px border radius like Flowbite
                                                bg,
                                            );
                                        }

                                        // Day number with optional bold using Inter font
                                        let font_id = if font_weight {
                                            egui::FontId::new(14.0, egui::FontFamily::Name("Inter".into()))
                                        } else {
                                            egui::FontId::new(13.0, egui::FontFamily::Name("Inter".into()))
                                        };

                                        let galley = ui.painter().layout_no_wrap(
                                            day.to_string(),
                                            font_id,
                                            text_color,
                                        );

                                        ui.painter().galley(
                                            rect.center() - galley.size() / 2.0,
                                            galley,
                                            text_color,
                                        );
                                    }

                                    if response.clicked() {
                                        date_clicked = Some(date);
                                    }
                                } else {
                                    // Previous/next month day - grey and non-interactive
                                    let (rect, _response) = ui.allocate_exact_size(cell_size, Sense::hover());

                                    if ui.is_rect_visible(rect) {
                                        // Day number in grey using Inter font
                                        ui.painter().text(
                                            rect.center(),
                                            egui::Align2::CENTER_CENTER,
                                            day.to_string(),
                                            egui::FontId::new(13.0, egui::FontFamily::Name("Inter".into())),
                                            theme.on_surface_variant().linear_multiply(0.5), // Very muted grey
                                        );
                                    }
                                }
                            }
                        });

                        // Add vertical spacing between rows
                        if row < 5 {
                            ui.add_space(2.0);
                        }
                    }

                    ui.add_space(8.0);

                    // Subtle separator
                    let sep_rect = ui.allocate_space(vec2(ui.available_width(), 1.0)).1;
                    ui.painter().rect_filled(
                        sep_rect,
                        0.0,
                        Color32::from_rgba_unmultiplied(
                            theme.outline().r(),
                            theme.outline().g(),
                            theme.outline().b(),
                            50,
                        ),
                    );

                    ui.add_space(8.0);

                    // Footer with quick actions - hardcoded centered positions
                    let button_width = 70.0;
                    let button_height = 32.0;
                    let button_spacing = 8.0;
                    let total_button_width = button_width * 2.0 + button_spacing;
                    let available_width = ui.available_width();
                    let start_x = (available_width - total_button_width) / 2.0;

                    let (footer_rect, _) = ui.allocate_exact_size(vec2(available_width, button_height), Sense::hover());

                    // Today button - outlined style
                    let today_text = "Today";
                    let today_rect = egui::Rect::from_min_size(
                        footer_rect.min + vec2(start_x, 0.0),
                        vec2(button_width, button_height)
                    );
                    let today_response = ui.interact(today_rect, ui.id().with("today_btn"), Sense::click());

                    if ui.is_rect_visible(today_rect) {
                        let hovered = today_response.hovered();

                        // Border
                        ui.painter().rect_stroke(
                            today_rect,
                            4.0,
                            egui::Stroke::new(
                                1.0,
                                if hovered { theme.primary() } else { theme.outline() }
                            ),
                            egui::StrokeKind::Outside,
                        );

                        // Background on hover
                        if hovered {
                            ui.painter().rect_filled(
                                today_rect,
                                4.0,
                                theme.primary().linear_multiply(0.1),
                            );
                        }

                        // Text
                        ui.painter().text(
                            today_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            today_text,
                            egui::FontId::new(13.0, egui::FontFamily::Name("Inter".into())),
                            if hovered { theme.primary() } else { theme.on_surface() },
                        );
                    }

                    if today_response.clicked() {
                        goto_today = true;
                    }

                    // Clear button - outlined style
                    let clear_text = "Clear";
                    let clear_rect = egui::Rect::from_min_size(
                        footer_rect.min + vec2(start_x + button_width + button_spacing, 0.0),
                        vec2(button_width, button_height)
                    );
                    let clear_response = ui.interact(clear_rect, ui.id().with("clear_btn"), Sense::click());

                    if ui.is_rect_visible(clear_rect) {
                        let hovered = clear_response.hovered();

                        // Border
                        ui.painter().rect_stroke(
                            clear_rect,
                            4.0,
                            egui::Stroke::new(
                                1.0,
                                if hovered { theme.error() } else { theme.outline() }
                            ),
                            egui::StrokeKind::Outside,
                        );

                        // Background on hover
                        if hovered {
                            ui.painter().rect_filled(
                                clear_rect,
                                4.0,
                                theme.error().linear_multiply(0.1),
                            );
                        }

                        // Text
                        ui.painter().text(
                            clear_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            clear_text,
                            egui::FontId::new(13.0, egui::FontFamily::Name("Inter".into())),
                            if hovered { theme.error() } else { theme.on_surface() },
                        );
                    }

                    if clear_response.clicked() {
                        clear_date = true;
                    }
                });
            });
        });

        // Handle month navigation
        if prev_month {
            self.previous_month();
        }
        if next_month {
            self.next_month();
        }

        // Handle date selection
        if let Some(date) = date_clicked {
            *selected_date = Some(date);
            self.input_text = date.format();
            self.is_open = false;
            response.changed = true;
        }

        if goto_today {
            *selected_date = Some(today);
            self.input_text = today.format();
            self.viewing_year = today.year;
            self.viewing_month = today.month;
            self.is_open = false;
            response.changed = true;
        }

        if clear_date {
            *selected_date = None;
            self.input_text.clear();
            self.is_open = false;
            response.changed = true;
        }

        response
    }

    fn previous_month(&mut self) {
        if self.viewing_month == 1 {
            self.viewing_month = 12;
            self.viewing_year -= 1;
        } else {
            self.viewing_month -= 1;
        }
    }

    fn next_month(&mut self) {
        if self.viewing_month == 12 {
            self.viewing_month = 1;
            self.viewing_year += 1;
        } else {
            self.viewing_month += 1;
        }
    }
}

/// Response from a date picker
#[derive(Debug, Clone, Copy)]
pub struct DatePickerResponse {
    /// Whether the selected date changed
    pub changed: bool,
}
