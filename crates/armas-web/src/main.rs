#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod components;
mod markdown;
mod showcase_gen;
pub mod syntax;

use armas::*;
use components::*;
use eframe::egui;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

// Type aliases for complex types
type PageShowFn = fn(&mut egui::Ui);
type Page = (&'static str, PageShowFn);

// Layout constants for the showcase website
mod layout {
    /// Sidebar width when expanded (desktop)
    pub const SIDEBAR_WIDTH_EXPANDED: f32 = 240.0;
    /// Sidebar width for tablet
    pub const SIDEBAR_WIDTH_TABLET: f32 = 280.0;
    /// Sidebar width when collapsed (icon-only)
    #[allow(dead_code)]
    pub const SIDEBAR_WIDTH_COLLAPSED: f32 = 70.0;
    /// Maximum content width for optimal reading
    pub const CONTENT_MAX_WIDTH: f32 = 1200.0;
    /// Minimum content width before layout breaks
    #[allow(dead_code)]
    pub const CONTENT_MIN_WIDTH: f32 = 600.0;
    /// Mobile breakpoint - collapse sidebar, single column
    pub const MOBILE_BREAKPOINT: f32 = 768.0;
    /// Tablet breakpoint - 2 column grids
    pub const TABLET_BREAKPOINT: f32 = 1024.0;
    /// Desktop breakpoint - 3 column grids
    pub const DESKTOP_BREAKPOINT: f32 = 1280.0;
}

// Showcase-specific component sizes
mod showcase_sizes {
    /// Standard demo card height
    #[allow(dead_code)]
    pub const DEMO_CARD_HEIGHT: f32 = 300.0;
    /// Tall demo card for complex examples
    #[allow(dead_code)]
    pub const DEMO_CARD_HEIGHT_TALL: f32 = 400.0;
    /// Grid gap between variant cards
    #[allow(dead_code)]
    pub const VARIANT_GRID_GAP: f32 = 16.0;
    /// Spacing between major sections
    #[allow(dead_code)]
    pub const SECTION_SPACING: f32 = 48.0;
    /// Minimum card width in grids
    #[allow(dead_code)]
    pub const CARD_MIN_WIDTH: f32 = 280.0;
    /// Header height
    #[allow(dead_code)]
    pub const HEADER_HEIGHT: f32 = 51.0;
    /// Search box min/max widths
    #[allow(dead_code)]
    pub const SEARCH_WIDTH_MIN: f32 = 150.0;
    #[allow(dead_code)]
    pub const SEARCH_WIDTH_MAX: f32 = 250.0;
    /// Glass panel blur amount
    pub const GLASS_BLUR: f32 = 10.0;
    /// Glass panel opacity
    pub const GLASS_OPACITY: f32 = 0.03;
    /// Content vertical spacing
    #[allow(dead_code)]
    pub const CONTENT_SPACING_TOP: f32 = 32.0;
    #[allow(dead_code)]
    pub const CONTENT_SPACING_BOTTOM: f32 = 64.0;
}

fn main() -> eframe::Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        wasm_bindgen_futures::spawn_local(async {
            use wasm_bindgen::JsCast;
            let document = web_sys::window()
                .expect("No window")
                .document()
                .expect("No document");
            let canvas = document
                .get_element_by_id("armas_canvas")
                .expect("Failed to find armas_canvas")
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .expect("armas_canvas is not a canvas");

            let web_options = eframe::WebOptions::default();
            eframe::WebRunner::new()
                .start(
                    canvas,
                    web_options,
                    Box::new(|cc| Ok(Box::new(ShowcaseApp::new(cc)))),
                )
                .await
                .expect("Failed to start eframe");
        });
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([layout::CONTENT_MAX_WIDTH, 800.0])
                .with_title("Armas - Component Library Showcase"),
            ..Default::default()
        };

        eframe::run_native(
            "Armas Showcase",
            native_options,
            Box::new(|cc| Ok(Box::new(ShowcaseApp::new(cc)))),
        )
    }
}

struct ShowcaseApp {
    app_theme: AppTheme,

    // UI state
    current_view: AppView, // Home or Components
    search_text: String,
    selected_page_index: usize,
    sidebar_open: bool,        // For mobile/tablet hamburger menu
    sidebar_just_opened: bool, // Prevent immediate close on same frame

    // Available pages from markdown
    pages: Vec<Page>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppView {
    Home,
    Components,
}

#[derive(Debug, Clone)]
enum AppTheme {
    Dark(Theme),
    Light(Theme),
}

impl AppTheme {
    fn dark() -> Self {
        let mut theme = Theme::dark();
        // Custom dark theme with pure black background
        theme.colors.background = [0, 0, 0];
        theme.colors.card = [10, 10, 10];
        theme.colors.muted = [20, 20, 20];
        theme.colors.primary = [255, 255, 255]; // White
        Self::Dark(theme)
    }

    fn light() -> Self {
        Self::Light(Theme::light())
    }

    fn theme(&self) -> &Theme {
        match self {
            Self::Dark(t) | Self::Light(t) => t,
        }
    }

    #[allow(dead_code)]
    fn is_dark(&self) -> bool {
        matches!(self, Self::Dark(_))
    }

    fn toggle(&self) -> Self {
        match self {
            Self::Dark(_) => Self::light(),
            Self::Light(_) => Self::dark(),
        }
    }
}

// Pages are now dynamically generated from markdown files

impl ShowcaseApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Detect system theme preference from egui's system theme detection
        let prefers_dark = cc.egui_ctx.style().visuals.dark_mode;

        // Use appropriate base theme
        let app_theme = if prefers_dark {
            AppTheme::dark()
        } else {
            AppTheme::light()
        };

        // Load Inter fonts using the builder
        let mut font_builder = armas::FontFamilyBuilder::new();
        font_builder
            .add_family(
                "Inter",
                include_bytes!("../../armas/fonts/Inter-Regular.otf"),
                Some(include_bytes!("../../armas/fonts/Inter-Medium.otf")),
                Some(include_bytes!("../../armas/fonts/Inter-SemiBold.otf")),
                Some(include_bytes!("../../armas/fonts/Inter-Bold.otf")),
            )
            .set_default("Inter");

        // Add FiraMono for code
        font_builder.add_family(
            "FiraMono",
            include_bytes!("../assets/fonts/FiraMono-Regular.ttf"),
            None,
            None,
            None,
        );

        // Install fonts - safe to do in CreationContext
        font_builder.install(&cc.egui_ctx, true);

        // Set theme in context
        cc.egui_ctx.set_armas_theme(app_theme.theme().clone());

        // Get all pages (flat list for index mapping)
        let pages = showcase_gen::get_pages();

        Self {
            app_theme,
            current_view: AppView::Home, // Start on home page
            search_text: String::new(),
            selected_page_index: 0,
            sidebar_open: false, // Closed by default on mobile
            sidebar_just_opened: false,
            pages,
        }
    }

    fn toggle_theme(&mut self, ctx: &egui::Context) {
        self.app_theme = self.app_theme.toggle();
        ctx.set_armas_theme(self.app_theme.theme().clone());
    }
}

impl eframe::App for ShowcaseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check if view should change (from hero button click)
        if let Some(view_str) = ctx.data(|d| d.get_temp::<String>(egui::Id::new("current_view"))) {
            if view_str == "components" {
                self.current_view = AppView::Components;
                // Clear the temp data
                ctx.data_mut(|d| d.remove::<String>(egui::Id::new("current_view")));
            }
        }

        // Check if theme toggle was requested (from code block theme button)
        let toggle_id = egui::Id::new("theme_toggle_request");
        if ctx.data(|d| d.get_temp::<bool>(toggle_id)).unwrap_or(false) {
            self.toggle_theme(ctx);
            // Clear the request
            ctx.data_mut(|d| d.remove::<bool>(toggle_id));
        }

        // Set the panel background based on theme
        let mut style = (*ctx.style()).clone();
        let bg_color = self.app_theme.theme().background();
        style.visuals.panel_fill = bg_color;
        style.visuals.window_fill = bg_color;
        style.visuals.extreme_bg_color = bg_color;
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;

                // Show header on all pages
                self.show_header(ui);

                // Show different content based on current view
                match self.current_view {
                    AppView::Home => {
                        Hero::show(ui);
                    }
                    AppView::Components => {
                        self.show_components_layout(ui);
                    }
                }
            });
        });

        ctx.request_repaint();
    }
}

impl ShowcaseApp {
    fn show_header(&mut self, ui: &mut egui::Ui) {
        if Header::show(ui, &mut self.search_text, &mut self.sidebar_open) {
            // Header returned true if hamburger was clicked
            self.sidebar_just_opened = self.sidebar_open;
        }
    }

    fn show_components_layout(&mut self, ui: &mut egui::Ui) {
        let available_rect = ui.available_rect_before_wrap();
        let screen_width = available_rect.width();

        // Responsive layout based on screen width
        let is_mobile = screen_width < layout::MOBILE_BREAKPOINT;
        let is_tablet =
            (layout::MOBILE_BREAKPOINT..layout::TABLET_BREAKPOINT).contains(&screen_width);

        if is_mobile {
            // Mobile: Full-screen sidebar with hamburger menu
            if !self.sidebar_open {
                // Only show content when sidebar is closed
                self.show_content(ui);
            }

            // Show sidebar as full-screen overlay when open
            if self.sidebar_open {
                // Full-screen backdrop
                let backdrop_rect = available_rect;
                ui.painter().rect_filled(
                    backdrop_rect,
                    0.0,
                    egui::Color32::from_black_alpha(200), // Darker backdrop for full-screen
                );

                // Full-screen sidebar
                let drawer_rect = available_rect;

                let page_was_clicked = ui
                    .scope_builder(egui::UiBuilder::new().max_rect(drawer_rect), |ui| {
                        self.show_sidebar(ui)
                    })
                    .inner;

                // Close sidebar if a page was clicked
                if page_was_clicked {
                    self.sidebar_open = false;
                }

                // Close drawer if clicked outside (but not if it just opened this frame)
                if !self.sidebar_just_opened {
                    let click_outside = ui.input(|i| {
                        i.pointer.primary_clicked()
                            && i.pointer
                                .interact_pos()
                                .is_some_and(|pos| !drawer_rect.contains(pos))
                    });

                    if click_outside {
                        self.sidebar_open = false;
                    }
                }

                // Reset the flag for next frame
                self.sidebar_just_opened = false;
            }
        } else if is_tablet {
            // Tablet: Hamburger menu with floating sidebar overlay (not full-screen)
            // Always show content
            self.show_content(ui);

            // Show sidebar as floating overlay when open
            if self.sidebar_open {
                let sidebar_width = layout::SIDEBAR_WIDTH_TABLET;

                // Semi-transparent backdrop
                let backdrop_rect = available_rect;
                let backdrop_response = ui.allocate_rect(backdrop_rect, egui::Sense::click());
                ui.painter()
                    .rect_filled(backdrop_rect, 0.0, egui::Color32::from_black_alpha(150));

                // Floating sidebar on the left
                let sidebar_rect = egui::Rect::from_min_size(
                    available_rect.min,
                    egui::vec2(sidebar_width, available_rect.height()),
                );

                // Draw sidebar background with glass effect
                ui.painter()
                    .rect_filled(sidebar_rect, 0.0, egui::Color32::from_rgb(10, 10, 10));

                let page_was_clicked = ui
                    .scope_builder(egui::UiBuilder::new().max_rect(sidebar_rect), |ui| {
                        self.show_sidebar(ui)
                    })
                    .inner;

                // Close sidebar if a page was clicked
                if page_was_clicked {
                    self.sidebar_open = false;
                }

                // Close if clicked outside sidebar (but not if it just opened this frame)
                if !self.sidebar_just_opened && backdrop_response.clicked() {
                    self.sidebar_open = false;
                }

                // Reset the flag for next frame
                self.sidebar_just_opened = false;
            }
        } else {
            // Desktop: Sidebar + Content side by side
            let sidebar_width = layout::SIDEBAR_WIDTH_EXPANDED;
            let gap = 16.0; // Gap between sidebar and content

            // Define sidebar and content rects
            let sidebar_rect = egui::Rect::from_min_size(
                available_rect.min,
                egui::vec2(sidebar_width, available_rect.height()),
            );

            let content_rect = egui::Rect::from_min_size(
                egui::pos2(
                    available_rect.min.x + sidebar_width + gap,
                    available_rect.min.y,
                ),
                egui::vec2(
                    available_rect.width() - sidebar_width - gap,
                    available_rect.height(),
                ),
            );

            // Render sidebar (ignore return value on desktop - sidebar always visible)
            let _ = ui.scope_builder(egui::UiBuilder::new().max_rect(sidebar_rect), |ui| {
                self.show_sidebar(ui)
            });

            // Render content
            let _ = ui.scope_builder(egui::UiBuilder::new().max_rect(content_rect), |ui| {
                self.show_content(ui);
            });
        }
    }

    fn show_sidebar(&mut self, ui: &mut egui::Ui) -> bool {
        let theme = self.app_theme.theme().clone();
        let mut page_clicked = false;
        let mut clicked_page_name: Option<String> = None;

        // Get nested sections for building the sidebar
        let nested_sections = showcase_gen::get_nested_sections();
        let search_text = self.search_text.trim().to_lowercase();
        let has_search = !search_text.is_empty();

        // Fuzzy matcher for search
        let matcher = if has_search {
            Some(SkimMatcherV2::default())
        } else {
            None
        };

        // Wrap sidebar in GlassPanel with independent scrolling
        GlassPanel::new()
            .blur(showcase_sizes::GLASS_BLUR)
            .opacity(showcase_sizes::GLASS_OPACITY)
            .inner_margin(8.0)
            .show(ui, &theme, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("sidebar_scroll")
                    .show(ui, |ui| {
                        let response = armas::Sidebar::new()
                            .collapsible(false)
                            .show_icons(false)
                            .show(ui, |sidebar| {
                                // Build sidebar content with nested sections
                                for (parent_name, subsections) in nested_sections.iter() {
                                    // Check if any subsection has matching pages
                                    let has_matching_pages =
                                        subsections.iter().any(|(_, pages)| {
                                            if let Some(ref matcher) = matcher {
                                                pages.iter().any(|(page_name, _)| {
                                                    matcher
                                                        .fuzzy_match(page_name, &search_text)
                                                        .is_some()
                                                })
                                            } else {
                                                !pages.is_empty()
                                            }
                                        });

                                    if has_matching_pages {
                                        // Check if this is a flat section (single subsection with pages directly)
                                        let is_flat =
                                            subsections.len() == 1 && !subsections[0].1.is_empty();

                                        if is_flat {
                                            // Flat section - show pages directly under parent
                                            sidebar.group("", parent_name, |parent_group| {
                                                let (_, section_pages) = &subsections[0];

                                                // Filter pages if search is active
                                                let filtered_pages: Vec<_> =
                                                    if let Some(ref matcher) = matcher {
                                                        section_pages
                                                            .iter()
                                                            .filter(|(page_name, _)| {
                                                                matcher
                                                                    .fuzzy_match(
                                                                        page_name,
                                                                        &search_text,
                                                                    )
                                                                    .is_some()
                                                            })
                                                            .collect()
                                                    } else {
                                                        section_pages.iter().collect()
                                                    };

                                                for (page_name, _) in filtered_pages {
                                                    parent_group.item("", page_name);
                                                }
                                            });
                                        } else {
                                            // Nested section - show subsections
                                            sidebar.group("", parent_name, |parent_group| {
                                                for (section_name, section_pages) in
                                                    subsections.iter()
                                                {
                                                    // Filter pages if search is active
                                                    let filtered_pages: Vec<_> =
                                                        if let Some(ref matcher) = matcher {
                                                            section_pages
                                                                .iter()
                                                                .filter(|(page_name, _)| {
                                                                    matcher
                                                                        .fuzzy_match(
                                                                            page_name,
                                                                            &search_text,
                                                                        )
                                                                        .is_some()
                                                                })
                                                                .collect()
                                                        } else {
                                                            section_pages.iter().collect()
                                                        };

                                                    // Only show subsection if it has pages
                                                    if !filtered_pages.is_empty() {
                                                        parent_group.group(
                                                            "",
                                                            section_name,
                                                            |subgroup| {
                                                                for (page_name, _) in filtered_pages
                                                                {
                                                                    subgroup.item("", page_name);
                                                                }
                                                            },
                                                        );
                                                    }
                                                }
                                            });
                                        }
                                    }
                                }
                            });

                        // Handle clicks
                        if let Some(clicked_id) = response.clicked {
                            // Extract the page name from the clicked ID
                            // The ID format is "item_depth_pagename"
                            if let Some(label_start) = clicked_id.rfind('_') {
                                let page_name = &clicked_id[label_start + 1..];
                                clicked_page_name = Some(page_name.to_string());
                            }
                        }
                    })
                    .inner
            });

        // If a page was clicked, find its index in the flat pages list
        if let Some(page_name) = clicked_page_name {
            for (index, (name, _)) in self.pages.iter().enumerate() {
                if *name == page_name {
                    self.selected_page_index = index;
                    page_clicked = true;
                    break;
                }
            }
        }

        page_clicked
    }

    fn show_content(&mut self, ui: &mut egui::Ui) {
        let theme = self.app_theme.theme().clone();

        GlassPanel::new()
            .blur(showcase_sizes::GLASS_BLUR)
            .opacity(showcase_sizes::GLASS_OPACITY)
            .inner_margin(0.0)
            .show(ui, &theme, |ui| {
                // Responsive outer padding
                let screen_width = ui.ctx().viewport_rect().width();
                let outer_padding = if screen_width < layout::MOBILE_BREAKPOINT {
                    16.0 // Minimal padding on mobile
                } else {
                    24.0 // Medium padding on tablet/desktop
                };

                let padding_frame = egui::Frame::NONE.inner_margin(outer_padding);

                padding_frame.show(ui, |ui| {
                    // Clean background with scroll area
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let available_width = ui.available_width();

                        // Responsive content width and padding
                        let (content_width, padding) = if screen_width < layout::MOBILE_BREAKPOINT {
                            // Mobile: Full width, no additional constraints
                            (available_width, 16.0)
                        } else if screen_width < layout::TABLET_BREAKPOINT {
                            // Tablet: Slightly narrower with more padding
                            (available_width.min(layout::CONTENT_MIN_WIDTH), 24.0)
                        } else {
                            // Desktop: Max width with optimal reading size
                            (available_width.min(layout::CONTENT_MAX_WIDTH), 32.0)
                        };

                        let margin = (available_width - content_width) / 2.0;

                        ui.add_space(margin.max(0.0));

                        ui.vertical(|ui| {
                            ui.set_max_width(content_width);
                            ui.add_space(padding);

                            // Show selected page content
                            if let Some((_, show_fn)) = self.pages.get(self.selected_page_index) {
                                show_fn(ui);
                            }

                            ui.add_space(padding * 2.0);
                        });
                    });
                });
            });
    }
}
