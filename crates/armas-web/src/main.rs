#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod components;
mod markdown;
mod showcase_gen;
pub mod syntax;

use armas::*;
use components::*;
use eframe::egui;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

// Type aliases for complex types
type PageShowFn = fn(&mut egui::Ui);
type Page = (&'static str, PageShowFn);

// Layout constants for the showcase website
mod layout {
    /// Sidebar width when expanded
    pub const SIDEBAR_WIDTH_EXPANDED: f32 = 240.0;
    /// Sidebar width when collapsed (icon-only)
    #[allow(dead_code)]
    pub const SIDEBAR_WIDTH_COLLAPSED: f32 = 70.0;
    /// Maximum content width for optimal reading
    pub const CONTENT_MAX_WIDTH: f32 = 1200.0;
    /// Minimum content width before layout breaks
    #[allow(dead_code)]
    pub const CONTENT_MIN_WIDTH: f32 = 600.0;
    /// Mobile breakpoint - collapse sidebar, single column
    #[allow(dead_code)]
    pub const MOBILE_BREAKPOINT: f32 = 768.0;
    /// Tablet breakpoint - 2 column grids
    #[allow(dead_code)]
    pub const TABLET_BREAKPOINT: f32 = 1024.0;
    /// Desktop breakpoint - 3 column grids
    #[allow(dead_code)]
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
    theme: Theme,

    // UI state
    current_view: AppView, // Home or Components
    search_text: String,
    selected_page_index: usize,
    sidebar_open: bool, // For mobile/tablet hamburger menu
    sidebar_just_opened: bool, // Prevent immediate close on same frame

    // Available pages from markdown
    pages: Vec<Page>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppView {
    Home,
    Components,
}

// Pages are now dynamically generated from markdown files

impl ShowcaseApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Custom showcase theme with pure black background
        let mut theme = Theme::dark();
        theme.colors.background = [0, 0, 0];
        theme.colors.surface = [10, 10, 10];
        theme.colors.surface_variant = [20, 20, 20];
        theme.colors.primary = [255, 255, 255]; // White

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
            include_bytes!("../../../web/FiraMono-Regular.ttf"),
            None,
            None,
            None,
        );

        // Install fonts - safe to do in CreationContext
        font_builder.install(&cc.egui_ctx, true);

        // Set theme in context
        cc.egui_ctx.set_armas_theme(theme.clone());

        // Get all pages (flat list for index mapping)
        let pages = showcase_gen::get_pages();

        Self {
            theme,
            current_view: AppView::Home, // Start on home page
            search_text: String::new(),
            selected_page_index: 0,
            sidebar_open: false, // Closed by default on mobile
            sidebar_just_opened: false,
            pages,
        }
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

        // Set the panel background to pure black
        let mut style = (*ctx.style()).clone();
        style.visuals.panel_fill = egui::Color32::from_rgb(0, 0, 0);
        style.visuals.window_fill = egui::Color32::from_rgb(0, 0, 0);
        style.visuals.extreme_bg_color = egui::Color32::from_rgb(0, 0, 0);
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
        let is_tablet = screen_width >= layout::MOBILE_BREAKPOINT
            && screen_width < layout::TABLET_BREAKPOINT;

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

                let page_was_clicked = ui.scope_builder(egui::UiBuilder::new().max_rect(drawer_rect), |ui| {
                    self.show_sidebar(ui)
                }).inner;

                // Close sidebar if a page was clicked
                if page_was_clicked {
                    self.sidebar_open = false;
                }

                // Close drawer if clicked outside (but not if it just opened this frame)
                if !self.sidebar_just_opened {
                    let click_outside = ui.input(|i| {
                        i.pointer.primary_clicked()
                            && i.pointer.interact_pos().is_some_and(|pos| !drawer_rect.contains(pos))
                    });

                    if click_outside {
                        self.sidebar_open = false;
                    }
                }

                // Reset the flag for next frame
                self.sidebar_just_opened = false;
            }
        } else if is_tablet {
            // Tablet: Sidebar + Content side by side (like desktop, no hamburger)
            let sidebar_width = layout::SIDEBAR_WIDTH_EXPANDED;

            // Define sidebar and content rects
            let sidebar_rect = egui::Rect::from_min_size(
                available_rect.min,
                egui::vec2(sidebar_width, available_rect.height()),
            );

            let content_rect = egui::Rect::from_min_size(
                egui::pos2(available_rect.min.x + sidebar_width, available_rect.min.y),
                egui::vec2(
                    available_rect.width() - sidebar_width,
                    available_rect.height(),
                ),
            );

            // Render sidebar (ignore return value on tablet - sidebar always visible)
            let _ = ui.scope_builder(egui::UiBuilder::new().max_rect(sidebar_rect), |ui| {
                self.show_sidebar(ui)
            });

            // Render content
            let _ = ui.scope_builder(egui::UiBuilder::new().max_rect(content_rect), |ui| {
                self.show_content(ui);
            });
        } else {
            // Desktop: Sidebar + Content side by side
            let sidebar_width = layout::SIDEBAR_WIDTH_EXPANDED;

            // Define sidebar and content rects
            let sidebar_rect = egui::Rect::from_min_size(
                available_rect.min,
                egui::vec2(sidebar_width, available_rect.height()),
            );

            let content_rect = egui::Rect::from_min_size(
                egui::pos2(available_rect.min.x + sidebar_width, available_rect.min.y),
                egui::vec2(
                    available_rect.width() - sidebar_width,
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

    fn build_filtered_sidebar_items(&self, ctx: &egui::Context) -> Vec<SidebarItem> {
        let sections = showcase_gen::get_sections();

        // If no search text, return all items
        if self.search_text.trim().is_empty() {
            return sections
                .iter()
                .enumerate()
                .map(|(section_idx, (section_name, section_pages))| {
                    let children: Vec<SidebarItem> = section_pages
                        .iter()
                        .map(|(page_name, _)| SidebarItem::new("", *page_name))
                        .collect();

                    // Load expanded state from memory for this section (default: collapsed)
                    let section_id = egui::Id::new(format!("sidebar_section_{}", section_idx));
                    let expanded = ctx.data_mut(|d| d.get_temp::<bool>(section_id).unwrap_or(false));

                    SidebarItem::new("", *section_name)
                        .with_children(children)
                        .expanded(expanded)
                })
                .collect();
        }

        // Fuzzy search filtering
        let matcher = SkimMatcherV2::default();
        let search_text = self.search_text.trim().to_lowercase();

        sections
            .iter()
            .filter_map(|(section_name, section_pages)| {
                let filtered_children: Vec<SidebarItem> = section_pages
                    .iter()
                    .filter_map(|(page_name, _)| {
                        // Try fuzzy match
                        if matcher.fuzzy_match(page_name, &search_text).is_some() {
                            Some(SidebarItem::new("", *page_name))
                        } else {
                            None
                        }
                    })
                    .collect();

                // Only include section if it has matching children
                if !filtered_children.is_empty() {
                    Some(
                        SidebarItem::new("", *section_name)
                            .with_children(filtered_children)
                            .expanded(true),
                    )
                } else {
                    None
                }
            })
            .collect()
    }

    fn show_sidebar(&mut self, ui: &mut egui::Ui) -> bool {
        let theme = self.theme.clone();
        let mut page_clicked = false;

        // Build filtered sidebar items based on search
        let filtered_items = self.build_filtered_sidebar_items(ui.ctx());
        let mut temp_sidebar = Sidebar::new(filtered_items)
            .expanded(true)
            .collapsible(false)
            .show_icons(false)
            .use_buttons(true)
            .button_variant(armas::ButtonVariant::Text)
            .persist_state("sidebar_section");

        // Wrap sidebar in GlassPanel
        GlassPanel::new()
            .blur(showcase_sizes::GLASS_BLUR)
            .opacity(showcase_sizes::GLASS_OPACITY)
            .inner_margin(8.0)
            .show(ui, &theme, |ui| {
                let response = temp_sidebar.show(ui);

                // Handle sidebar clicks (child items only, parents are just sections)
                if let Some((parent_idx, child_idx)) = response.clicked_child {
                    // Calculate flat index from parent/child indices based on FILTERED results
                    let sections = showcase_gen::get_sections();
                    let filtered_sections = self.build_filtered_sidebar_items(ui.ctx());

                    // Find which section this is in the original list
                    if let Some(clicked_section) = filtered_sections.get(parent_idx) {
                        let children = &clicked_section.children;
                        if let Some(clicked_item) = children.get(child_idx) {
                            let item_title = &clicked_item.label;

                            // Find the global index
                            let mut flat_index = 0;
                            'outer: for (_, section_pages) in sections.iter() {
                                for (page_name, _) in section_pages {
                                    if page_name == item_title {
                                        break 'outer;
                                    }
                                    flat_index += 1;
                                }
                            }

                            self.selected_page_index = flat_index;
                            page_clicked = true;
                        }
                    }
                }
            });

        page_clicked
    }

    fn show_content(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();

        GlassPanel::new()
            .blur(showcase_sizes::GLASS_BLUR)
            .opacity(showcase_sizes::GLASS_OPACITY)
            .inner_margin(0.0)
            .show(ui, &theme, |ui| {
                // Draw left border inside the panel
                let panel_rect = ui.available_rect_before_wrap();
                ui.painter().vline(
                    panel_rect.left(),
                    panel_rect.top()..=panel_rect.bottom(),
                    egui::Stroke::new(1.0, theme.outline()),
                );

                // Add padding inside the panel
                let padding_frame = egui::Frame::NONE
                    .inner_margin(32.0);

                padding_frame.show(ui, |ui| {
                // Clean background with scroll area
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let available_width = ui.available_width();
                    let screen_width = ui.ctx().viewport_rect().width();

                    // Responsive content width and padding
                    let (content_width, padding) = if screen_width < layout::MOBILE_BREAKPOINT {
                        // Mobile: Full width with minimal padding
                        (available_width - 32.0, 16.0)
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
