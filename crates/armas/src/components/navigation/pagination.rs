//! Pagination Component
//!
//! Page navigation styled like shadcn/ui Pagination.
//! Provides previous/next buttons and page number navigation with ellipsis support.
//!
//! # Example
//!
//! ```rust,no_run
//! # use egui::Ui;
//! # fn example(ui: &mut Ui) {
//! use armas::Pagination;
//!
//! let (_, current_page) = Pagination::new(1, 10).show(ui);
//! // current_page is the page after user interaction
//! # }
//! ```

use crate::ext::ArmasContextExt;
use crate::{Button, ButtonVariant};
use egui::{vec2, Sense, Ui};

// shadcn Pagination constants
const BUTTON_SIZE: f32 = 36.0; // size-9
const BUTTON_GAP: f32 = 4.0; // gap-1
const ICON_SIZE: f32 = 16.0; // size-4
const CORNER_RADIUS: f32 = 6.0; // rounded-md
const DEFAULT_SIBLING_COUNT: usize = 1;

/// Pagination component for navigating through pages
///
/// Styled like shadcn/ui Pagination with previous/next buttons and page numbers.
///
/// # Example
///
/// ```rust,no_run
/// # use egui::Ui;
/// # fn example(ui: &mut Ui) {
/// use armas::Pagination;
///
/// let (_, current_page) = Pagination::new(1, 10).show(ui);
/// // current_page is the current page after any user interaction
/// # }
/// ```
pub struct Pagination {
    id: Option<egui::Id>,
    initial_page: usize,
    total_pages: usize,
    sibling_count: usize,
    show_prev_next: bool,
}

impl Pagination {
    /// Create a new pagination component
    ///
    /// # Arguments
    /// * `initial_page` - Initial/current page (1-indexed)
    /// * `total_pages` - Total number of pages
    pub fn new(initial_page: usize, total_pages: usize) -> Self {
        Self {
            id: None,
            initial_page: initial_page.max(1).min(total_pages.max(1)),
            total_pages: total_pages.max(1),
            sibling_count: DEFAULT_SIBLING_COUNT,
            show_prev_next: true,
        }
    }

    /// Set ID for state persistence
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the number of sibling pages to show on each side of current page
    pub fn sibling_count(mut self, count: usize) -> Self {
        self.sibling_count = count;
        self
    }

    /// Show or hide previous/next buttons
    pub fn show_prev_next(mut self, show: bool) -> Self {
        self.show_prev_next = show;
        self
    }

    /// Show the pagination and return (Response, current_page)
    pub fn show(self, ui: &mut Ui) -> (egui::Response, usize) {
        let theme = ui.ctx().armas_theme();
        let total_pages = self.total_pages;

        // Load state from memory if ID is set
        let mut current_page = if let Some(id) = self.id {
            let state_id = id.with("page");
            ui.ctx()
                .data_mut(|d| d.get_temp(state_id).unwrap_or(self.initial_page))
        } else {
            self.initial_page
        };

        // Calculate visible pages
        let pages = calculate_visible_pages(current_page, total_pages, self.sibling_count);

        let response = ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = BUTTON_GAP;

            // Previous button - custom drawn with icon + text
            if self.show_prev_next {
                let can_go_prev = current_page > 1;
                let prev_clicked = draw_nav_button(ui, &theme, "Previous", true, can_go_prev);
                if prev_clicked {
                    current_page -= 1;
                }
            }

            // Page number buttons
            for page in pages.iter() {
                if let Some(page_num) = page {
                    let is_current = *page_num == current_page;
                    let variant = if is_current {
                        ButtonVariant::Outlined
                    } else {
                        ButtonVariant::Ghost
                    };

                    let btn = Button::new(page_num.to_string())
                        .variant(variant)
                        .min_width(BUTTON_SIZE)
                        .show(ui);

                    if btn.clicked() && !is_current {
                        current_page = *page_num;
                    }
                } else {
                    // Ellipsis - shadcn uses MoreHorizontal icon (three dots)
                    let (rect, _) =
                        ui.allocate_exact_size(vec2(BUTTON_SIZE, BUTTON_SIZE), Sense::hover());

                    if ui.is_rect_visible(rect) {
                        // Draw three horizontal dots (MoreHorizontal icon)
                        let dot_radius = 2.0;
                        let dot_spacing = 4.0;
                        let center = rect.center();
                        let color = theme.muted_foreground();

                        for i in -1..=1 {
                            let x = center.x + (i as f32 * dot_spacing);
                            ui.painter()
                                .circle_filled(egui::pos2(x, center.y), dot_radius, color);
                        }
                    }
                }
            }

            // Next button - custom drawn with text + icon
            if self.show_prev_next {
                let can_go_next = current_page < total_pages;
                let next_clicked = draw_nav_button(ui, &theme, "Next", false, can_go_next);
                if next_clicked {
                    current_page += 1;
                }
            }
        })
        .response;

        // Save state to memory if ID is set
        if let Some(id) = self.id {
            let state_id = id.with("page");
            ui.ctx().data_mut(|d| {
                d.insert_temp(state_id, current_page);
            });
        }

        (response, current_page)
    }
}

/// Draw a navigation button (Previous/Next) with icon
/// Returns true if clicked
fn draw_nav_button(
    ui: &mut Ui,
    theme: &crate::Theme,
    label: &str,
    is_previous: bool,
    enabled: bool,
) -> bool {
    let font_id = egui::FontId::proportional(14.0);
    // Approximate text width (average char width * length)
    let text_width = 8.0 * label.len() as f32;
    let icon_width = ICON_SIZE;
    let padding = 10.0;
    let gap = 4.0;

    let total_width = padding + icon_width + gap + text_width + padding;
    let (rect, response) = ui.allocate_exact_size(vec2(total_width, BUTTON_SIZE), Sense::click());

    let clicked = enabled && response.clicked();
    let hovered = enabled && response.hovered();

    if ui.is_rect_visible(rect) {
        // Background on hover (ghost button style)
        if hovered {
            ui.painter()
                .rect_filled(rect, CORNER_RADIUS, theme.accent());
        }

        let text_color = if enabled {
            if hovered {
                theme.accent_foreground()
            } else {
                theme.foreground()
            }
        } else {
            theme.muted_foreground()
        };

        let icon_color = text_color;

        if is_previous {
            // Icon on left, text on right
            let icon_center = egui::pos2(rect.left() + padding + icon_width / 2.0, rect.center().y);
            draw_chevron_left(ui.painter(), icon_center, icon_color);

            let text_pos = egui::pos2(rect.left() + padding + icon_width + gap, rect.center().y);
            ui.painter().text(
                text_pos,
                egui::Align2::LEFT_CENTER,
                label,
                font_id,
                text_color,
            );
        } else {
            // Text on left, icon on right
            let text_pos = egui::pos2(rect.left() + padding, rect.center().y);
            ui.painter().text(
                text_pos,
                egui::Align2::LEFT_CENTER,
                label,
                font_id,
                text_color,
            );

            let icon_center =
                egui::pos2(rect.right() - padding - icon_width / 2.0, rect.center().y);
            draw_chevron_right(ui.painter(), icon_center, icon_color);
        }
    }

    clicked
}

/// Draw a left chevron icon at center position
fn draw_chevron_left(painter: &egui::Painter, center: egui::Pos2, color: egui::Color32) {
    let half = ICON_SIZE * 0.15;
    let stroke = egui::Stroke::new(1.5, color);

    painter.line_segment(
        [
            egui::pos2(center.x + half, center.y - half * 2.0),
            egui::pos2(center.x - half, center.y),
        ],
        stroke,
    );
    painter.line_segment(
        [
            egui::pos2(center.x - half, center.y),
            egui::pos2(center.x + half, center.y + half * 2.0),
        ],
        stroke,
    );
}

/// Draw a right chevron icon at center position
fn draw_chevron_right(painter: &egui::Painter, center: egui::Pos2, color: egui::Color32) {
    let half = ICON_SIZE * 0.15;
    let stroke = egui::Stroke::new(1.5, color);

    painter.line_segment(
        [
            egui::pos2(center.x - half, center.y - half * 2.0),
            egui::pos2(center.x + half, center.y),
        ],
        stroke,
    );
    painter.line_segment(
        [
            egui::pos2(center.x + half, center.y),
            egui::pos2(center.x - half, center.y + half * 2.0),
        ],
        stroke,
    );
}

/// Calculate which pages to show, including ellipsis (None)
/// Uses shadcn/ui pagination pattern
fn calculate_visible_pages(current: usize, total: usize, siblings: usize) -> Vec<Option<usize>> {
    // Total slots: first + maybe_ellipsis + siblings + current + siblings + maybe_ellipsis + last
    // shadcn shows: [1] [...] [current-1] [current] [current+1] [...] [total]

    if total <= 7 {
        // Show all pages if 7 or fewer
        return (1..=total).map(Some).collect();
    }

    let left_sibling = current.saturating_sub(siblings).max(1);
    let right_sibling = (current + siblings).min(total);

    let show_left_ellipsis = left_sibling > 2;
    let show_right_ellipsis = right_sibling < total - 1;

    let mut pages = Vec::new();

    if !show_left_ellipsis && show_right_ellipsis {
        // Near start: [1] [2] [3] [4] [5] [...] [total]
        for i in 1..=5 {
            pages.push(Some(i));
        }
        pages.push(None);
        pages.push(Some(total));
    } else if show_left_ellipsis && !show_right_ellipsis {
        // Near end: [1] [...] [total-4] [total-3] [total-2] [total-1] [total]
        pages.push(Some(1));
        pages.push(None);
        for i in (total - 4)..=total {
            pages.push(Some(i));
        }
    } else if show_left_ellipsis && show_right_ellipsis {
        // Middle: [1] [...] [current-1] [current] [current+1] [...] [total]
        pages.push(Some(1));
        pages.push(None);
        for i in left_sibling..=right_sibling {
            pages.push(Some(i));
        }
        pages.push(None);
        pages.push(Some(total));
    } else {
        // Shouldn't happen with total > 7
        for i in 1..=total {
            pages.push(Some(i));
        }
    }

    pages
}
