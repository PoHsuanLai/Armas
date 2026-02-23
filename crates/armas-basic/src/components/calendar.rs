//! Calendar Component (shadcn/ui style)
//!
//! A standalone month calendar for date selection.
//!
//! ```rust,no_run
//! # use egui::Ui;
//! # fn example(ui: &mut Ui) {
//! use armas_basic::prelude::*;
//!
//! let mut selected = None;
//! let mut calendar = Calendar::new("cal");
//! calendar.show(ui, &mut selected);
//! # }
//! ```

use crate::ext::ArmasContextExt;
use crate::icon;
use crate::Theme;
use egui::{vec2, Color32, Id, Rect, Sense, Ui};

use super::date_picker::Date;

// shadcn calendar constants
const CELL_SIZE: f32 = 32.0;
const CALENDAR_PADDING: f32 = 12.0;
const CALENDAR_WIDTH: f32 = 252.0;
const NAV_BUTTON_SIZE: f32 = 32.0;
const FONT_SIZE: f32 = 14.0;
const SMALL_FONT_SIZE: f32 = 12.0;

/// A standalone calendar for date selection.
pub struct Calendar {
    id: Id,
    show_footer: bool,
    show_outside_days: bool,
}

/// Response from a calendar.
pub struct CalendarResponse {
    /// The UI response.
    pub response: egui::Response,
    /// Whether the selected date changed this frame.
    pub changed: bool,
}

impl Calendar {
    /// Create a new calendar with a unique ID.
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            show_footer: false,
            show_outside_days: true,
        }
    }

    /// Show Today/Clear footer buttons.
    #[must_use]
    pub const fn show_footer(mut self, show: bool) -> Self {
        self.show_footer = show;
        self
    }

    /// Show days from adjacent months in leading/trailing cells.
    #[must_use]
    pub const fn show_outside_days(mut self, show: bool) -> Self {
        self.show_outside_days = show;
        self
    }

    /// Show the calendar.
    ///
    /// # Panics
    ///
    /// Panics if internal calendar calculations produce an invalid date.
    pub fn show(&mut self, ui: &mut Ui, selected_date: &mut Option<Date>) -> CalendarResponse {
        let theme = ui.ctx().armas_theme();
        let mut date_changed = false;

        let today_id = Id::new("calendar_today_cache");
        let today = ui
            .ctx()
            .data(|d| d.get_temp::<Date>(today_id))
            .unwrap_or_else(|| {
                let today = Date::today();
                ui.ctx().data_mut(|d| d.insert_temp(today_id, today));
                today
            });

        let state_id = self.id.with("cal_state");
        let (viewing_year, viewing_month) = ui.ctx().data(|d| {
            d.get_temp::<(i32, u32)>(state_id)
                .unwrap_or((today.year, today.month))
        });

        let mut viewing_year = viewing_year;
        let mut viewing_month = viewing_month;
        let mut action = CalendarAction::new();

        // Render
        let response = egui::Frame::new()
            .inner_margin(CALENDAR_PADDING)
            .show(ui, |ui| {
                ui.set_min_width(CALENDAR_WIDTH);
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 4.0;

                    render_header(ui, &theme, viewing_year, viewing_month, &mut action);
                    ui.add_space(4.0);
                    render_day_grid(
                        ui,
                        &theme,
                        viewing_year,
                        viewing_month,
                        today,
                        selected_date.as_ref(),
                        self.show_outside_days,
                        &mut action,
                    );

                    if self.show_footer {
                        render_footer(ui, &theme, &mut action);
                    }
                });
            })
            .response;

        // Handle actions
        if action.prev_month {
            if viewing_month == 1 {
                viewing_month = 12;
                viewing_year -= 1;
            } else {
                viewing_month -= 1;
            }
        }
        if action.next_month {
            if viewing_month == 12 {
                viewing_month = 1;
                viewing_year += 1;
            } else {
                viewing_month += 1;
            }
        }
        if let Some(date) = action.date_clicked {
            *selected_date = Some(date);
            date_changed = true;
        }
        if action.goto_today {
            *selected_date = Some(today);
            viewing_year = today.year;
            viewing_month = today.month;
            date_changed = true;
        }
        if action.clear_date {
            *selected_date = None;
            date_changed = true;
        }

        // Save state
        ui.ctx()
            .data_mut(|d| d.insert_temp(state_id, (viewing_year, viewing_month)));

        CalendarResponse {
            response,
            changed: date_changed,
        }
    }

    /// Navigate the calendar to the given year and month.
    pub fn set_viewing(&self, ctx: &egui::Context, year: i32, month: u32) {
        let state_id = self.id.with("cal_state");
        ctx.data_mut(|d| d.insert_temp(state_id, (year, month)));
    }
}

/// Accumulated user interactions from within the calendar.
#[derive(Default)]
pub(crate) struct CalendarAction {
    pub(crate) date_clicked: Option<Date>,
    pub(crate) goto_today: bool,
    pub(crate) clear_date: bool,
    pub(crate) prev_month: bool,
    pub(crate) next_month: bool,
}

impl CalendarAction {
    pub(crate) const fn new() -> Self {
        Self {
            date_clicked: None,
            goto_today: false,
            clear_date: false,
            prev_month: false,
            next_month: false,
        }
    }
}

/// Render the month/year navigation header with prev/next arrows.
pub(crate) fn render_header(
    ui: &mut Ui,
    theme: &Theme,
    viewing_year: i32,
    viewing_month: u32,
    action: &mut CalendarAction,
) {
    ui.horizontal(|ui| {
        // Previous month button
        let (prev_rect, prev_response) =
            ui.allocate_exact_size(vec2(NAV_BUTTON_SIZE, NAV_BUTTON_SIZE), Sense::click());

        if ui.is_rect_visible(prev_rect) {
            if prev_response.hovered() {
                ui.painter().rect_filled(prev_rect, 4.0, theme.accent());
            }

            let icon_rect = Rect::from_center_size(prev_rect.center(), vec2(16.0, 16.0));
            icon::draw_chevron_left(
                ui.painter(),
                icon_rect,
                if prev_response.hovered() {
                    theme.accent_foreground()
                } else {
                    theme.foreground()
                },
            );
        }

        if prev_response.clicked() {
            action.prev_month = true;
        }

        // Month/Year label
        let label_width = CALENDAR_WIDTH - NAV_BUTTON_SIZE * 2.0 - 8.0;
        ui.allocate_ui(vec2(label_width, NAV_BUTTON_SIZE), |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(
                    egui::RichText::new(format!(
                        "{} {}",
                        Date::new(viewing_year, viewing_month, 1)
                            .expect("First day of month should always be valid")
                            .month_name(),
                        viewing_year
                    ))
                    .size(FONT_SIZE)
                    .strong()
                    .color(theme.foreground()),
                );
            });
        });

        // Next month button
        let (next_rect, next_response) =
            ui.allocate_exact_size(vec2(NAV_BUTTON_SIZE, NAV_BUTTON_SIZE), Sense::click());

        if ui.is_rect_visible(next_rect) {
            if next_response.hovered() {
                ui.painter().rect_filled(next_rect, 4.0, theme.accent());
            }

            let icon_rect = Rect::from_center_size(next_rect.center(), vec2(16.0, 16.0));
            icon::draw_chevron_right(
                ui.painter(),
                icon_rect,
                if next_response.hovered() {
                    theme.accent_foreground()
                } else {
                    theme.foreground()
                },
            );
        }

        if next_response.clicked() {
            action.next_month = true;
        }
    });
}

/// Render the weekday header row and calendar day grid.
#[allow(clippy::too_many_arguments)]
pub(crate) fn render_day_grid(
    ui: &mut Ui,
    theme: &Theme,
    viewing_year: i32,
    viewing_month: u32,
    today: Date,
    selected_date: Option<&Date>,
    show_outside_days: bool,
    action: &mut CalendarAction,
) {
    // Weekday headers
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        for day in &["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"] {
            ui.allocate_ui(vec2(CELL_SIZE, CELL_SIZE), |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        egui::RichText::new(*day)
                            .size(SMALL_FONT_SIZE)
                            .color(theme.muted_foreground()),
                    );
                });
            });
        }
    });

    // Calendar grid
    let first_day = Date::new(viewing_year, viewing_month, 1)
        .expect("First day of month should always be valid");
    let first_weekday = first_day.day_of_week();
    let days_in_month = Date::days_in_month(viewing_year, viewing_month);

    let (prev_year, prev_month_num) = if viewing_month == 1 {
        (viewing_year - 1, 12)
    } else {
        (viewing_year, viewing_month - 1)
    };
    let (next_year, next_month_num) = if viewing_month == 12 {
        (viewing_year + 1, 1)
    } else {
        (viewing_year, viewing_month + 1)
    };
    let prev_month_days = Date::days_in_month(prev_year, prev_month_num);

    let mut day_counter = 1u32;

    for row in 0..6 {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;

            for col in 0..7 {
                let cell_index = row * 7 + col;

                let (day, is_current_month, actual_year, actual_month) =
                    if cell_index < first_weekday {
                        let day = prev_month_days - (first_weekday - cell_index - 1);
                        (day, false, prev_year, prev_month_num)
                    } else if day_counter <= days_in_month {
                        let day = day_counter;
                        day_counter += 1;
                        (day, true, viewing_year, viewing_month)
                    } else {
                        let day = day_counter - days_in_month;
                        day_counter += 1;
                        (day, false, next_year, next_month_num)
                    };

                let show_cell = is_current_month || show_outside_days;

                let date = Date::new(actual_year, actual_month, day)
                    .expect("Calendar day should be valid");
                let is_today = date == today;
                let is_selected = selected_date == Some(&date);

                let sense = if is_current_month {
                    Sense::click()
                } else {
                    Sense::hover()
                };

                let (rect, cell_response) =
                    ui.allocate_exact_size(vec2(CELL_SIZE, CELL_SIZE), sense);

                if ui.is_rect_visible(rect) && show_cell {
                    let hovered = cell_response.hovered() && is_current_month;

                    let (bg_color, text_color) = if is_selected {
                        (Some(theme.primary()), theme.primary_foreground())
                    } else if is_today || hovered {
                        (Some(theme.accent()), theme.accent_foreground())
                    } else if !is_current_month {
                        (None, theme.muted_foreground())
                    } else {
                        (None, theme.foreground())
                    };

                    if let Some(bg) = bg_color {
                        ui.painter().rect_filled(rect, 4.0, bg);
                    }

                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        day.to_string(),
                        egui::FontId::proportional(FONT_SIZE),
                        text_color,
                    );
                }

                if cell_response.clicked() && is_current_month {
                    action.date_clicked = Some(date);
                }
            }
        });
    }
}

/// Render the optional footer with Today/Clear buttons.
pub(crate) fn render_footer(ui: &mut Ui, theme: &Theme, action: &mut CalendarAction) {
    ui.add_space(8.0);

    let sep_rect = ui.allocate_space(vec2(ui.available_width(), 1.0)).1;
    ui.painter().rect_filled(sep_rect, 0.0, theme.border());

    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;

        // Today button
        let today_btn_size = vec2(60.0, 32.0);
        let (today_rect, today_response) = ui.allocate_exact_size(today_btn_size, Sense::click());

        if ui.is_rect_visible(today_rect) {
            if today_response.hovered() {
                ui.painter().rect_filled(today_rect, 4.0, theme.accent());
            }

            ui.painter().text(
                today_rect.center(),
                egui::Align2::CENTER_CENTER,
                "Today",
                egui::FontId::proportional(FONT_SIZE),
                if today_response.hovered() {
                    theme.accent_foreground()
                } else {
                    theme.foreground()
                },
            );
        }

        if today_response.clicked() {
            action.goto_today = true;
        }

        // Clear button
        let clear_btn_size = vec2(60.0, 32.0);
        let (clear_rect, clear_response) = ui.allocate_exact_size(clear_btn_size, Sense::click());

        if ui.is_rect_visible(clear_rect) {
            if clear_response.hovered() {
                ui.painter().rect_filled(
                    clear_rect,
                    4.0,
                    Color32::from_rgba_unmultiplied(
                        theme.destructive().r(),
                        theme.destructive().g(),
                        theme.destructive().b(),
                        25,
                    ),
                );
            }

            ui.painter().text(
                clear_rect.center(),
                egui::Align2::CENTER_CENTER,
                "Clear",
                egui::FontId::proportional(FONT_SIZE),
                if clear_response.hovered() {
                    theme.destructive()
                } else {
                    theme.muted_foreground()
                },
            );
        }

        if clear_response.clicked() {
            action.clear_date = true;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_builder() {
        let cal = Calendar::new("test")
            .show_footer(true)
            .show_outside_days(false);
        assert!(cal.show_footer);
        assert!(!cal.show_outside_days);
    }
}
