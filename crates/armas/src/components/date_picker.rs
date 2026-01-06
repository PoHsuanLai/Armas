//! DatePicker Component
//!
//! Calendar date selection with input field and popover

use crate::layout::{Grid, HStack, VStack};
use crate::{Button, ButtonVariant, Input, Popover, PopoverPosition, Theme};
use egui::{vec2, Id, Ui};

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
/// date_picker.show(ui, &theme, &mut selected_date);
/// ```
pub struct DatePicker {
    id: Id,
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
            id,
            popover: Popover::new(id.with("popover"))
                .position(PopoverPosition::Bottom)
                .width(320.0),
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
        theme: &Theme,
        selected_date: &mut Option<Date>,
    ) -> DatePickerResponse {
        let mut response = DatePickerResponse { changed: false };

        // Update input text from selected date
        if let Some(date) = selected_date {
            self.input_text = date.format();
        }

        // Label
        if let Some(label) = &self.label {
            ui.label(label);
            ui.add_space(4.0);
        }

        // Input field
        let input = Input::new(&self.placeholder);
        let input_response = input.show(ui, &mut self.input_text, theme);
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

        self.popover
            .show(ui.ctx(), theme, input_rect, &mut self.is_open, |ui| {
                VStack::new(12.0).show(ui, |ui| {
                    // Header with month/year navigation
                    HStack::new(8.0).show(ui, |ui| {
                        if Button::new("‹")
                            .variant(ButtonVariant::Text)
                            .min_size(vec2(32.0, 32.0))
                            .show(ui, theme)
                            .clicked()
                        {
                            prev_month = true;
                        }

                        ui.with_layout(
                            egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                            |ui| {
                                ui.label(format!(
                                    "{} {}",
                                    Date::new(viewing_year, viewing_month, 1)
                                        .unwrap()
                                        .month_name(),
                                    viewing_year
                                ));
                            },
                        );

                        if Button::new("›")
                            .variant(ButtonVariant::Text)
                            .min_size(vec2(32.0, 32.0))
                            .show(ui, theme)
                            .clicked()
                        {
                            next_month = true;
                        }
                    });

                    ui.add_space(4.0);

                    // Weekday headers
                    Grid::new(7)
                        .gap(4.0)
                        .id_source("calendar_weekday_headers")
                        .show(ui, |grid| {
                            for day in &["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"] {
                                grid.cell(|ui| {
                                    ui.allocate_ui(vec2(40.0, 24.0), |ui| {
                                        ui.centered_and_justified(|ui| {
                                            ui.colored_label(theme.on_surface_variant(), *day);
                                        });
                                    });
                                });
                            }
                        });

                    ui.add_space(4.0);

                    // Calendar grid
                    let first_day = Date::new(viewing_year, viewing_month, 1).unwrap();
                    let first_weekday = first_day.day_of_week();
                    let days_in_month = Date::days_in_month(viewing_year, viewing_month);

                    Grid::new(7)
                        .gap(4.0)
                        .id_source("calendar_days_grid")
                        .show(ui, |grid| {
                            let mut day_counter = 1;

                            // Always render 6 weeks (42 cells) for consistent height
                            let total_cells = 42; // 6 weeks * 7 days

                            for cell_index in 0..total_cells {
                                grid.cell(|ui| {
                                    ui.allocate_ui(vec2(40.0, 36.0), |ui| {
                                        if cell_index >= first_weekday
                                            && day_counter <= days_in_month
                                        {
                                            // Render day button
                                            let day = day_counter;
                                            let date = Date::new(viewing_year, viewing_month, day)
                                                .unwrap();
                                            let is_today = date == today;
                                            let is_selected = *selected_date == Some(date);

                                            let variant = if is_selected {
                                                ButtonVariant::Filled
                                            } else if is_today {
                                                ButtonVariant::Outlined
                                            } else {
                                                ButtonVariant::Text
                                            };

                                            let button = Button::new(&day.to_string())
                                                .variant(variant)
                                                .min_size(vec2(36.0, 36.0));

                                            let btn_response = button.show(ui, theme);

                                            if btn_response.clicked() {
                                                date_clicked = Some(date);
                                            }

                                            day_counter += 1;
                                        } else {
                                            // Empty cell for days outside current month
                                            ui.allocate_space(vec2(36.0, 36.0));
                                        }
                                    });
                                });
                            }
                        });

                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(8.0);

                    // Footer with quick actions
                    HStack::new(8.0).show(ui, |ui| {
                        if Button::new("Today")
                            .variant(ButtonVariant::Text)
                            .show(ui, theme)
                            .clicked()
                        {
                            goto_today = true;
                        }

                        if Button::new("Clear")
                            .variant(ButtonVariant::Text)
                            .show(ui, theme)
                            .clicked()
                        {
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
