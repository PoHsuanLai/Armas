//! Pagination Component (shadcn/ui style)
//!
//! Page navigation for paginated content.

use crate::ext::ArmasContextExt;
use crate::{Button, ButtonVariant};
use egui::{vec2, Ui};

/// Pagination component for navigating through pages
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
    max_visible_pages: usize,
    show_prev_next: bool,
    show_labels: bool,
    sibling_count: usize,
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
            max_visible_pages: 7,
            show_prev_next: true,
            show_labels: true,
            sibling_count: 1,
        }
    }

    /// Set ID for state persistence
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set maximum number of visible page buttons
    pub fn max_visible_pages(mut self, max: usize) -> Self {
        self.max_visible_pages = max.max(3);
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

    /// Show or hide text labels on prev/next buttons
    pub fn show_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
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

        // Calculate visible pages based on current state
        let pages = calculate_visible_pages(current_page, total_pages, self.sibling_count, self.max_visible_pages);

        let response = ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;

            // Previous button
            if self.show_prev_next {
                let can_go_prev = current_page > 1;
                let label = if self.show_labels { "< Previous" } else { "<" };
                let width = if self.show_labels { 100.0 } else { 36.0 };

                let btn = Button::new(label)
                    .variant(ButtonVariant::Text)
                    .min_size(vec2(width, 36.0))
                    .enabled(can_go_prev)
                    .show(ui);

                if can_go_prev && btn.clicked() {
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
                        ButtonVariant::Text
                    };

                    let btn = Button::new(page_num.to_string())
                        .variant(variant)
                        .min_size(vec2(36.0, 36.0))
                        .show(ui);

                    if btn.clicked() && !is_current {
                        current_page = *page_num;
                    }
                } else {
                    // Ellipsis
                    let (rect, _) = ui.allocate_exact_size(vec2(36.0, 36.0), egui::Sense::hover());
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "...",
                        egui::FontId::proportional(14.0),
                        theme.muted_foreground(),
                    );
                }
            }

            // Next button
            if self.show_prev_next {
                let can_go_next = current_page < total_pages;
                let label = if self.show_labels { "Next >" } else { ">" };
                let width = if self.show_labels { 80.0 } else { 36.0 };

                let btn = Button::new(label)
                    .variant(ButtonVariant::Text)
                    .min_size(vec2(width, 36.0))
                    .enabled(can_go_next)
                    .show(ui);

                if can_go_next && btn.clicked() {
                    current_page += 1;
                }
            }
        }).response;

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

/// Calculate which pages to show, including ellipsis (None)
/// Uses shadcn/ui pagination pattern with consistent element count
fn calculate_visible_pages(
    current: usize,
    total: usize,
    siblings: usize,
    max_visible: usize,
) -> Vec<Option<usize>> {
    if total <= max_visible {
        return (1..=total).map(Some).collect();
    }

    // Calculate boundaries
    // "boundary" pages are always shown at start/end
    let boundary = 1;
    // siblings on each side of current
    let sibling_count = siblings;

    // Calculate how many items we'd show without ellipsis
    // [1] [2] ... [current-1] [current] [current+1] ... [total-1] [total]

    let left_sibling_idx = current.saturating_sub(sibling_count).max(1);
    let right_sibling_idx = (current + sibling_count).min(total);

    let show_left_ellipsis = left_sibling_idx > boundary + 1;
    let show_right_ellipsis = right_sibling_idx < total - boundary;

    let mut pages = Vec::new();

    if !show_left_ellipsis && show_right_ellipsis {
        // Near start: show first few pages + ellipsis + last
        // [1] [2] [3] [4] [...] [total]
        let left_count = max_visible - 2; // -2 for ellipsis and last page
        for i in 1..=left_count {
            pages.push(Some(i));
        }
        pages.push(None); // ellipsis
        pages.push(Some(total));
    } else if show_left_ellipsis && !show_right_ellipsis {
        // Near end: show first + ellipsis + last few pages
        // [1] [...] [7] [8] [9] [10]
        pages.push(Some(1));
        pages.push(None); // ellipsis
        let right_count = max_visible - 2; // -2 for first page and ellipsis
        let start = total - right_count + 1;
        for i in start..=total {
            pages.push(Some(i));
        }
    } else if show_left_ellipsis && show_right_ellipsis {
        // Middle: show first + ellipsis + current vicinity + ellipsis + last
        // [1] [...] [4] [5] [6] [...] [10]
        pages.push(Some(1));
        pages.push(None); // left ellipsis
        for i in left_sibling_idx..=right_sibling_idx {
            pages.push(Some(i));
        }
        pages.push(None); // right ellipsis
        pages.push(Some(total));
    } else {
        // No ellipsis needed (shouldn't happen if total > max_visible)
        for i in 1..=total {
            pages.push(Some(i));
        }
    }

    pages
}
