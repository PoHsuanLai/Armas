//! DatePicker Component
//!
//! Calendar date selection styled like shadcn/ui.
//! Combines a Button trigger with a Calendar popover.
//!
//! # Example
//!
//! ```rust,no_run
//! # use egui::{Context, Ui};
//! # fn example(ctx: &Context, ui: &mut Ui) {
//! use armas::{DatePicker, Date, Theme};
//!
//! let theme = Theme::dark();
//! let mut date_picker = DatePicker::new("birthday");
//! let mut selected_date = None;
//!
//! date_picker.show(ctx, &theme, ui, &mut selected_date);
//! # }
//! ```

use crate::icon::{render_icon, WindowIcon};
use crate::{Popover, PopoverPosition, Theme};
use egui::{vec2, Color32, Id, Rect, Sense, Ui};

// shadcn calendar constants
const CELL_SIZE: f32 = 32.0; // --cell-size: 2rem
const CALENDAR_PADDING: f32 = 12.0; // p-3
const CALENDAR_WIDTH: f32 = 252.0; // 7 * 32px + 6 * 2px gaps + padding
const NAV_BUTTON_SIZE: f32 = 32.0; // h-[--cell-size] w-[--cell-size]
const TRIGGER_WIDTH: f32 = 280.0; // w-[280px]
const TRIGGER_HEIGHT: f32 = 40.0; // h-10
const FONT_SIZE: f32 = 14.0; // text-sm
const SMALL_FONT_SIZE: f32 = 12.0; // text-xs for weekday headers
const CORNER_RADIUS: f32 = 6.0; // rounded-md

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

    /// Format as human-readable (e.g., "January 15, 2024")
    pub fn format_display(&self) -> String {
        format!("{} {}, {}", self.month_name(), self.day, self.year)
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

/// DatePicker component styled like shadcn/ui
///
/// # Example
///
/// ```rust,no_run
/// # use egui::{Context, Ui};
/// # fn example(ctx: &Context, ui: &mut Ui) {
/// use armas::{DatePicker, Date, Theme};
///
/// let theme = Theme::dark();
/// let mut date_picker = DatePicker::new("birthday");
/// let mut selected_date = None;
///
/// date_picker.show(ctx, &theme, ui, &mut selected_date);
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
                .padding(0.0) // shadcn: p-0
                .width(CALENDAR_WIDTH + CALENDAR_PADDING * 2.0),
            placeholder: "Pick a date".to_string(),
            label: None,
            show_footer: false, // shadcn default: no footer
            width: TRIGGER_WIDTH,
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

    /// Show Today/Clear footer buttons
    pub fn show_footer(mut self, show: bool) -> Self {
        self.show_footer = show;
        self
    }

    /// Set trigger button width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Show the date picker
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        theme: &Theme,
        ui: &mut Ui,
        selected_date: &mut Option<Date>,
    ) -> DatePickerResponse {
        let mut response = DatePickerResponse { changed: false };

        // Load internal state from context
        let state_id = self.id.with("state");

        // Get or initialize today's date (cached globally once per session)
        let today_id = Id::new("datepicker_today_cache");
        let today = ctx
            .data(|d| d.get_temp::<Date>(today_id))
            .unwrap_or_else(|| {
                let today = Date::today();
                ctx.data_mut(|d| {
                    d.insert_temp(today_id, today);
                });
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

        // Trigger button (shadcn outline variant style)
        let trigger_size = vec2(self.width, TRIGGER_HEIGHT);
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

            // Draw a simple calendar icon using the painter
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
            let text = if let Some(date) = selected_date {
                date.format_display()
            } else {
                self.placeholder.clone()
            };

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

        // Toggle popover on click
        if trigger_response.clicked() {
            is_open = !is_open;
            // Navigate to selected date's month when opening
            if is_open {
                if let Some(date) = selected_date {
                    viewing_year = date.year;
                    viewing_month = date.month;
                }
            }
        }

        // Calendar popover
        let mut date_clicked = None;
        let mut goto_today = false;
        let mut clear_date = false;
        let mut prev_month_clicked = false;
        let mut next_month_clicked = false;

        let show_footer = self.show_footer;

        self.popover.set_open(is_open);

        let popover_response = self.popover.show(ctx, theme, trigger_rect, |ui| {
            ui.set_min_width(CALENDAR_WIDTH);

            // Calendar content with padding
            egui::Frame::new()
                .inner_margin(CALENDAR_PADDING)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 4.0;

                        // Header: navigation + month/year
                        ui.horizontal(|ui| {
                            // Previous month button (ghost variant)
                            let (prev_rect, prev_response) = ui.allocate_exact_size(
                                vec2(NAV_BUTTON_SIZE, NAV_BUTTON_SIZE),
                                Sense::click(),
                            );

                            if ui.is_rect_visible(prev_rect) {
                                if prev_response.hovered() {
                                    ui.painter().rect_filled(prev_rect, 4.0, theme.accent());
                                }

                                let icon_rect =
                                    Rect::from_center_size(prev_rect.center(), vec2(16.0, 16.0));
                                render_icon(
                                    ui.painter(),
                                    icon_rect,
                                    WindowIcon::ChevronLeft.data(),
                                    if prev_response.hovered() {
                                        theme.accent_foreground()
                                    } else {
                                        theme.foreground()
                                    },
                                );
                            }

                            if prev_response.clicked() {
                                prev_month_clicked = true;
                            }

                            // Month/Year label (centered)
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

                            // Next month button (ghost variant)
                            let (next_rect, next_response) = ui.allocate_exact_size(
                                vec2(NAV_BUTTON_SIZE, NAV_BUTTON_SIZE),
                                Sense::click(),
                            );

                            if ui.is_rect_visible(next_rect) {
                                if next_response.hovered() {
                                    ui.painter().rect_filled(next_rect, 4.0, theme.accent());
                                }

                                let icon_rect =
                                    Rect::from_center_size(next_rect.center(), vec2(16.0, 16.0));
                                render_icon(
                                    ui.painter(),
                                    icon_rect,
                                    WindowIcon::ChevronRight.data(),
                                    if next_response.hovered() {
                                        theme.accent_foreground()
                                    } else {
                                        theme.foreground()
                                    },
                                );
                            }

                            if next_response.clicked() {
                                next_month_clicked = true;
                            }
                        });

                        ui.add_space(4.0);

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

                        // Calculate previous/next month info
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

                        // Render 6 rows
                        for row in 0..6 {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 2.0;

                                for col in 0..7 {
                                    let cell_index = row * 7 + col;

                                    // Determine which day to show
                                    let (day, is_current_month, actual_year, actual_month) =
                                        if cell_index < first_weekday {
                                            let day =
                                                prev_month_days - (first_weekday - cell_index - 1);
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

                                    let date = Date::new(actual_year, actual_month, day)
                                        .expect("Calendar day should be valid");
                                    let is_today = date == today;
                                    let is_selected = *selected_date == Some(date);

                                    let sense = if is_current_month {
                                        Sense::click()
                                    } else {
                                        Sense::hover()
                                    };

                                    let (rect, cell_response) =
                                        ui.allocate_exact_size(vec2(CELL_SIZE, CELL_SIZE), sense);

                                    if ui.is_rect_visible(rect) {
                                        let hovered = cell_response.hovered() && is_current_month;

                                        // Determine colors based on state (shadcn style)
                                        let (bg_color, text_color) = if is_selected {
                                            // Selected: bg-primary text-primary-foreground
                                            (Some(theme.primary()), theme.primary_foreground())
                                        } else if is_today {
                                            // Today: bg-accent text-accent-foreground
                                            (Some(theme.accent()), theme.accent_foreground())
                                        } else if hovered {
                                            // Hover: bg-accent text-accent-foreground
                                            (Some(theme.accent()), theme.accent_foreground())
                                        } else if !is_current_month {
                                            // Outside month: text-muted-foreground
                                            (None, theme.muted_foreground())
                                        } else {
                                            // Normal
                                            (None, theme.foreground())
                                        };

                                        // Background
                                        if let Some(bg) = bg_color {
                                            ui.painter().rect_filled(rect, 4.0, bg);
                                        }

                                        // Day number
                                        ui.painter().text(
                                            rect.center(),
                                            egui::Align2::CENTER_CENTER,
                                            day.to_string(),
                                            egui::FontId::proportional(FONT_SIZE),
                                            text_color,
                                        );
                                    }

                                    if cell_response.clicked() && is_current_month {
                                        date_clicked = Some(date);
                                    }
                                }
                            });
                        }

                        // Optional footer with Today/Clear buttons
                        if show_footer {
                            ui.add_space(8.0);

                            // Separator
                            let sep_rect = ui.allocate_space(vec2(ui.available_width(), 1.0)).1;
                            ui.painter().rect_filled(sep_rect, 0.0, theme.border());

                            ui.add_space(8.0);

                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 8.0;

                                // Today button (ghost variant)
                                let today_btn_size = vec2(60.0, 32.0);
                                let (today_rect, today_response) =
                                    ui.allocate_exact_size(today_btn_size, Sense::click());

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
                                    goto_today = true;
                                }

                                // Clear button (ghost destructive variant)
                                let clear_btn_size = vec2(60.0, 32.0);
                                let (clear_rect, clear_response) =
                                    ui.allocate_exact_size(clear_btn_size, Sense::click());

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
                                    clear_date = true;
                                }
                            });
                        }
                    });
                });
        });

        // Handle clicking outside the popover to close
        if popover_response.clicked_outside || popover_response.should_close {
            is_open = false;
        }

        // Handle month navigation
        if prev_month_clicked {
            if viewing_month == 1 {
                viewing_month = 12;
                viewing_year -= 1;
            } else {
                viewing_month -= 1;
            }
        }
        if next_month_clicked {
            if viewing_month == 12 {
                viewing_month = 1;
                viewing_year += 1;
            } else {
                viewing_month += 1;
            }
        }

        // Handle date selection
        if let Some(date) = date_clicked {
            *selected_date = Some(date);
            is_open = false;
            response.changed = true;
        }

        if goto_today {
            *selected_date = Some(today);
            viewing_year = today.year;
            viewing_month = today.month;
            is_open = false;
            response.changed = true;
        }

        if clear_date {
            *selected_date = None;
            is_open = false;
            response.changed = true;
        }

        // Save internal state back to context
        ctx.data_mut(|d| {
            d.insert_temp(state_id, (is_open, viewing_year, viewing_month));
        });

        response
    }
}

/// Response from a date picker
#[derive(Debug, Clone, Copy)]
pub struct DatePickerResponse {
    /// Whether the selected date changed
    pub changed: bool,
}
