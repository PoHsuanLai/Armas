//! Pagination Component
//!
//! Page navigation for paginated content

use crate::ext::ArmasContextExt;
use crate::{Button, ButtonVariant};
use egui::Ui;

/// Pagination component for navigating through pages
///
/// # Example
///
/// ```rust,no_run
/// use armas::Pagination;
///
/// let mut pagination = Pagination::new(1, 10);
///
/// let response = pagination.show(ui, &theme);
/// if let Some(page) = response.page_changed {
///     println!("Changed to page: {}", page);
/// }
/// ```
pub struct Pagination {
    current_page: usize,
    total_pages: usize,
    max_visible_pages: usize,
    show_first_last: bool,
    show_prev_next: bool,
    spacing: f32,
}

impl Pagination {
    /// Create a new pagination component
    ///
    /// # Arguments
    /// * `current_page` - Current page (1-indexed)
    /// * `total_pages` - Total number of pages
    pub fn new(current_page: usize, total_pages: usize) -> Self {
        Self {
            current_page: current_page.max(1).min(total_pages.max(1)),
            total_pages: total_pages.max(1),
            max_visible_pages: 7,
            show_first_last: true,
            show_prev_next: true,
            spacing: 4.0,
        }
    }

    /// Set maximum number of visible page buttons
    pub fn max_visible_pages(mut self, max: usize) -> Self {
        self.max_visible_pages = max.max(3);
        self
    }

    /// Show or hide first/last buttons
    pub fn show_first_last(mut self, show: bool) -> Self {
        self.show_first_last = show;
        self
    }

    /// Show or hide previous/next buttons
    pub fn show_prev_next(mut self, show: bool) -> Self {
        self.show_prev_next = show;
        self
    }

    /// Set spacing between buttons
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Show the pagination
    pub fn show(mut self, ui: &mut Ui) -> PaginationResponse {
        let _theme = ui.ctx().armas_theme();

        // Generate a stable ID for this pagination instance based on current UI scope
        let pagination_id = ui.id().with("pagination_state");

        // Load previous state from egui memory (if current_page is the default)
        if self.current_page == 1 {
            if let Some(stored_page) = ui.ctx().data_mut(|d| d.get_persisted::<usize>(pagination_id)) {
                self.current_page = stored_page;
            }
        }

        let mut page_changed = None;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = self.spacing;
            // First button
            if self.show_first_last {
                let enabled = self.current_page > 1;
                let mut button = Button::new("«")
                    .variant(ButtonVariant::Outlined)
                    .min_size(egui::vec2(32.0, 32.0));

                if !enabled {
                    button = button.enabled(false);
                }

                if button.show(ui).clicked() && enabled {
                    self.current_page = 1;
                    page_changed = Some(1);
                }
            }

            // Previous button
            if self.show_prev_next {
                let enabled = self.current_page > 1;
                let mut button = Button::new("‹")
                    .variant(ButtonVariant::Outlined)
                    .min_size(egui::vec2(32.0, 32.0));

                if !enabled {
                    button = button.enabled(false);
                }

                if button.show(ui).clicked() && enabled {
                    self.current_page -= 1;
                    page_changed = Some(self.current_page);
                }
            }

            // Calculate visible page range
            let pages = self.calculate_visible_pages();

            // Page number buttons
            for (_idx, page) in pages.iter().enumerate() {
                if let Some(page_num) = page {
                    let is_current = *page_num == self.current_page;
                    let variant = if is_current {
                        ButtonVariant::Filled
                    } else {
                        ButtonVariant::Outlined
                    };

                    if Button::new(&page_num.to_string())
                        .variant(variant)
                        .min_size(egui::vec2(32.0, 32.0))
                        .show(ui)
                        .clicked()
                        && !is_current
                    {
                        self.current_page = *page_num;
                        page_changed = Some(*page_num);
                    }
                } else {
                    // Ellipsis
                    ui.add_space(4.0);
                    ui.label("…");
                    ui.add_space(4.0);
                }
            }

            // Next button
            if self.show_prev_next {
                let enabled = self.current_page < self.total_pages;
                let mut button = Button::new("›")
                    .variant(ButtonVariant::Outlined)
                    .min_size(egui::vec2(32.0, 32.0));

                if !enabled {
                    button = button.enabled(false);
                }

                if button.show(ui).clicked() && enabled {
                    self.current_page += 1;
                    page_changed = Some(self.current_page);
                }
            }

            // Last button
            if self.show_first_last {
                let enabled = self.current_page < self.total_pages;
                let mut button = Button::new("»")
                    .variant(ButtonVariant::Outlined)
                    .min_size(egui::vec2(32.0, 32.0));

                if !enabled {
                    button = button.enabled(false);
                }

                if button.show(ui).clicked() && enabled {
                    self.current_page = self.total_pages;
                    page_changed = Some(self.total_pages);
                }
            }
        });

        // Store current page if changed
        if page_changed.is_some() {
            ui.ctx().data_mut(|d| {
                d.insert_persisted(pagination_id, self.current_page);
            });
        }

        PaginationResponse { page_changed }
    }

    /// Calculate which pages to show, including ellipsis
    fn calculate_visible_pages(&self) -> Vec<Option<usize>> {
        let mut pages = Vec::new();

        if self.total_pages <= self.max_visible_pages {
            // Show all pages
            for i in 1..=self.total_pages {
                pages.push(Some(i));
            }
        } else {
            // Calculate range around current page
            let half = self.max_visible_pages / 2;
            let mut start = self.current_page.saturating_sub(half);
            let mut end = self.current_page + half;

            // Adjust if at boundaries
            if start < 1 {
                start = 1;
                end = self.max_visible_pages;
            }
            if end > self.total_pages {
                end = self.total_pages;
                start = self.total_pages.saturating_sub(self.max_visible_pages - 1);
            }

            // Always show first page
            if start > 1 {
                pages.push(Some(1));
                if start > 2 {
                    pages.push(None); // Ellipsis
                }
            }

            // Show range
            for i in start..=end {
                pages.push(Some(i));
            }

            // Always show last page
            if end < self.total_pages {
                if end < self.total_pages - 1 {
                    pages.push(None); // Ellipsis
                }
                pages.push(Some(self.total_pages));
            }
        }

        pages
    }
}

/// Response from pagination
#[derive(Debug, Clone, Copy)]
pub struct PaginationResponse {
    /// The new page number if changed
    pub page_changed: Option<usize>,
}
