//! `DatePicker` Component
//!
//! Calendar date selection styled like shadcn/ui.
//! Combines a Button trigger with a Calendar popover.
//!
//! # Example
//!
//! ```rust,no_run
//! # use egui::{Context, Ui};
//! # fn example(ctx: &Context, ui: &mut Ui) {
//! use armas_basic::{DatePicker, Date};
//!
//! let mut date_picker = DatePicker::new("birthday");
//! let mut selected_date = None;
//!
//! date_picker.show(ctx, ui, &mut selected_date);
//! # }
//! ```

use crate::ext::ArmasContextExt;
use crate::{Popover, PopoverPosition, Theme};
use egui::{vec2, Id, Rect, Sense, Ui};

use super::calendar::{render_day_grid, render_footer, render_header, CalendarAction};

// shadcn calendar constants
const CALENDAR_PADDING: f32 = 12.0;
const CALENDAR_WIDTH: f32 = 252.0; // 7 * 32px + 6 * 2px gaps + padding
const TRIGGER_WIDTH: f32 = 280.0;
const TRIGGER_HEIGHT: f32 = 40.0;
const FONT_SIZE: f32 = 14.0;
const CORNER_RADIUS: f32 = 6.0;

/// A date value (year, month, day)
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Date {
    /// Year
    pub year: i32,
    /// Month (1-12)
    pub month: u32,
    /// Day of month (1-31)
    pub day: u32,
}

impl Date {
    /// Create a new date
    #[must_use]
    pub fn new(year: i32, month: u32, day: u32) -> Option<Self> {
        if !(1..=12).contains(&month) {
            return None;
        }
        let days_in_month = Self::days_in_month(year, month);
        if day < 1 || day > days_in_month {
            return None;
        }
        Some(Self { year, month, day })
    }

    /// Get today's date (using chrono)
    #[must_use]
    pub fn today() -> Self {
        use chrono::Datelike;
        let now = chrono::Local::now().date_naive();
        Self {
            year: now.year(),
            month: now.month(),
            day: now.day(),
        }
    }

    /// Check if a year is a leap year
    #[must_use]
    pub const fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Get the number of days in a month
    #[must_use]
    pub const fn days_in_month(year: i32, month: u32) -> u32 {
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
    #[must_use]
    #[allow(clippy::many_single_char_names, clippy::cast_possible_wrap)]
    pub const fn day_of_week(&self) -> u32 {
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

    /// Format as human-readable (e.g., "January 15, 2024")
    #[must_use]
    pub fn format_display(&self) -> String {
        format!("{} {}, {}", self.month_name(), self.day, self.year)
    }

    /// Format as YYYY-MM-DD
    #[must_use]
    pub fn format(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Parse from YYYY-MM-DD format
    #[must_use]
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
    #[must_use]
    pub const fn month_name(&self) -> &'static str {
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

/// `DatePicker` component styled like shadcn/ui
///
/// # Example
///
/// ```rust,no_run
/// # use egui::{Context, Ui};
/// # fn example(ctx: &Context, ui: &mut Ui) {
/// use armas_basic::{DatePicker, Date};
///
/// let mut date_picker = DatePicker::new("birthday");
/// let mut selected_date = None;
///
/// date_picker.show(ctx, ui, &mut selected_date);
/// # }
/// ```
#[derive(Clone)]
pub struct DatePicker {
    id: Id,
    popover: Popover,
    placeholder: String,
    label: Option<String>,
    show_footer: bool,
    width: f32,
}

impl DatePicker {
    /// Create a new date picker
    pub fn new(id: impl Into<Id>) -> Self {
        let id = id.into();
        Self {
            id,
            popover: Popover::new(id.with("popover"))
                .position(PopoverPosition::Bottom)
                .style(crate::PopoverStyle::Default)
                .padding(0.0)
                .width(CALENDAR_WIDTH + CALENDAR_PADDING * 2.0),
            placeholder: "Pick a date".to_string(),
            label: None,
            show_footer: false,
            width: TRIGGER_WIDTH,
        }
    }

    /// Set the placeholder text
    #[must_use]
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set a label for the date picker
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Show Today/Clear footer buttons
    #[must_use]
    pub const fn show_footer(mut self, show: bool) -> Self {
        self.show_footer = show;
        self
    }

    /// Set trigger button width
    #[must_use]
    pub const fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Show the date picker
    ///
    /// # Panics
    ///
    /// Panics if internal calendar calculations produce an invalid date.
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        ui: &mut Ui,
        selected_date: &mut Option<Date>,
    ) -> DatePickerResponse {
        let theme = ui.ctx().armas_theme();
        let mut date_changed = false;

        // Load internal state from context
        let state_id = self.id.with("state");

        let today_id = Id::new("datepicker_today_cache");
        let today = ctx
            .data(|d| d.get_temp::<Date>(today_id))
            .unwrap_or_else(|| {
                let today = Date::today();
                ctx.data_mut(|d| d.insert_temp(today_id, today));
                today
            });

        let (is_open, viewing_year, viewing_month) = ctx.data(|d| {
            d.get_temp::<(bool, i32, u32)>(state_id)
                .unwrap_or((false, today.year, today.month))
        });

        let mut is_open = is_open;
        let mut viewing_year = viewing_year;
        let mut viewing_month = viewing_month;

        // Label
        if let Some(label) = &self.label {
            ui.label(
                egui::RichText::new(label)
                    .size(FONT_SIZE)
                    .color(theme.foreground()),
            );
            ui.add_space(4.0);
        }

        // Trigger button
        let trigger_rect = Self::render_trigger(
            ui,
            &theme,
            selected_date.as_ref(),
            &self.placeholder,
            self.width,
        );

        // Toggle popover on click
        if ui
            .interact(trigger_rect, self.id.with("trigger"), Sense::click())
            .clicked()
        {
            is_open = !is_open;
            if is_open {
                if let Some(date) = selected_date {
                    viewing_year = date.year;
                    viewing_month = date.month;
                }
            }
        }

        // Calendar popover — delegates to shared calendar rendering functions
        let mut calendar_action = CalendarAction::new();
        let show_footer = self.show_footer;

        self.popover.set_open(is_open);

        let popover_response = self.popover.show(ctx, &theme, trigger_rect, |ui| {
            ui.set_min_width(CALENDAR_WIDTH);

            egui::Frame::new()
                .inner_margin(CALENDAR_PADDING)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 4.0;

                        render_header(
                            ui,
                            &theme,
                            viewing_year,
                            viewing_month,
                            &mut calendar_action,
                        );
                        ui.add_space(4.0);
                        render_day_grid(
                            ui,
                            &theme,
                            viewing_year,
                            viewing_month,
                            today,
                            selected_date.as_ref(),
                            true, // show_outside_days
                            &mut calendar_action,
                        );

                        if show_footer {
                            render_footer(ui, &theme, &mut calendar_action);
                        }
                    });
                });
        });

        // Handle clicking outside the popover to close
        if popover_response.clicked_outside || popover_response.should_close {
            is_open = false;
        }

        // Handle month navigation
        if calendar_action.prev_month {
            if viewing_month == 1 {
                viewing_month = 12;
                viewing_year -= 1;
            } else {
                viewing_month -= 1;
            }
        }
        if calendar_action.next_month {
            if viewing_month == 12 {
                viewing_month = 1;
                viewing_year += 1;
            } else {
                viewing_month += 1;
            }
        }

        // Handle date selection
        if let Some(date) = calendar_action.date_clicked {
            *selected_date = Some(date);
            is_open = false;
            date_changed = true;
        }

        if calendar_action.goto_today {
            *selected_date = Some(today);
            viewing_year = today.year;
            viewing_month = today.month;
            is_open = false;
            date_changed = true;
        }

        if calendar_action.clear_date {
            *selected_date = None;
            is_open = false;
            date_changed = true;
        }

        // Save internal state back to context
        ctx.data_mut(|d| {
            d.insert_temp(state_id, (is_open, viewing_year, viewing_month));
        });

        let response = ui.interact(ui.min_rect(), self.id.with("response"), Sense::hover());

        DatePickerResponse {
            response,
            changed: date_changed,
        }
    }

    /// Render the trigger button that opens the calendar popover.
    fn render_trigger(
        ui: &mut Ui,
        theme: &Theme,
        selected_date: Option<&Date>,
        placeholder: &str,
        width: f32,
    ) -> Rect {
        let trigger_size = vec2(width, TRIGGER_HEIGHT);
        let (trigger_rect, trigger_response) = ui.allocate_exact_size(trigger_size, Sense::click());

        if ui.is_rect_visible(trigger_rect) {
            let hovered = trigger_response.hovered();
            let has_value = selected_date.is_some();

            // Background
            ui.painter()
                .rect_filled(trigger_rect, CORNER_RADIUS, theme.background());

            // Border (outline variant)
            let border_color = if hovered { theme.ring() } else { theme.input() };
            ui.painter().rect_stroke(
                trigger_rect,
                CORNER_RADIUS,
                egui::Stroke::new(1.0, border_color),
                egui::StrokeKind::Inside,
            );

            // Calendar icon (left side)
            let icon_size = 16.0;
            let icon_rect = Rect::from_center_size(
                trigger_rect.left_center() + vec2(16.0, 0.0),
                vec2(icon_size, icon_size),
            );

            let icon_color = theme.muted_foreground();
            let ir = icon_rect;

            // Calendar outline
            ui.painter().rect_stroke(
                Rect::from_min_size(ir.min + vec2(1.0, 2.0), vec2(14.0, 12.0)),
                2.0,
                egui::Stroke::new(1.5, icon_color),
                egui::StrokeKind::Inside,
            );
            // Calendar top hooks
            ui.painter().line_segment(
                [ir.min + vec2(5.0, 0.0), ir.min + vec2(5.0, 4.0)],
                egui::Stroke::new(1.5, icon_color),
            );
            ui.painter().line_segment(
                [ir.min + vec2(11.0, 0.0), ir.min + vec2(11.0, 4.0)],
                egui::Stroke::new(1.5, icon_color),
            );
            // Calendar horizontal line
            ui.painter().line_segment(
                [ir.min + vec2(1.0, 7.0), ir.min + vec2(15.0, 7.0)],
                egui::Stroke::new(1.0, icon_color),
            );

            // Text (date or placeholder)
            let text = selected_date.map_or_else(|| placeholder.to_string(), Date::format_display);

            let text_color = if has_value {
                theme.foreground()
            } else {
                theme.muted_foreground()
            };

            ui.painter().text(
                trigger_rect.left_center() + vec2(36.0, 0.0),
                egui::Align2::LEFT_CENTER,
                &text,
                egui::FontId::proportional(FONT_SIZE),
                text_color,
            );
        }

        trigger_rect
    }
}

/// Response from a date picker
pub struct DatePickerResponse {
    /// The UI response
    pub response: egui::Response,
    /// Whether the selected date changed
    pub changed: bool,
}
