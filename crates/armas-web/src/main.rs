#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod components;
mod examples;
mod markdown;
mod showcase_gen;
pub mod syntax;

use armas::*;
use components::*;
use eframe::egui;

// Layout constants for the showcase website
mod layout {
    /// Sidebar width when expanded
    pub const SIDEBAR_WIDTH_EXPANDED: f32 = 240.0;
    /// Sidebar width when collapsed (icon-only)
    pub const SIDEBAR_WIDTH_COLLAPSED: f32 = 70.0;
    /// Maximum content width for optimal reading
    pub const CONTENT_MAX_WIDTH: f32 = 1200.0;
    /// Minimum content width before layout breaks
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
    pub const DEMO_CARD_HEIGHT: f32 = 300.0;
    /// Tall demo card for complex examples
    pub const DEMO_CARD_HEIGHT_TALL: f32 = 400.0;
    /// Grid gap between variant cards
    pub const VARIANT_GRID_GAP: f32 = 16.0;
    /// Spacing between major sections
    pub const SECTION_SPACING: f32 = 48.0;
    /// Minimum card width in grids
    pub const CARD_MIN_WIDTH: f32 = 280.0;
    /// Header height
    pub const HEADER_HEIGHT: f32 = 51.0;
    /// Search box min/max widths
    pub const SEARCH_WIDTH_MIN: f32 = 150.0;
    pub const SEARCH_WIDTH_MAX: f32 = 250.0;
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
                .with_inner_size([1200.0, 800.0])
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
    search_text: String,
    selected_page_index: usize,

    // Available pages from markdown
    pages: Vec<(&'static str, fn(&mut egui::Ui))>,

    // UI Components
    sidebar: Sidebar,
}

// Pages are now dynamically generated from markdown files

impl ShowcaseApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Custom showcase theme with pure black background
        let mut theme = Theme::dark();
        theme.colors.background = [0, 0, 0];
        theme.colors.surface = [10, 10, 10];
        theme.colors.surface_variant = [20, 20, 20];
        theme.colors.primary = [59, 130, 246]; // Blue instead of purple

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

        // Install fonts - safe to do in CreationContext
        font_builder.install(&cc.egui_ctx, true);

        // Set theme in context
        cc.egui_ctx.set_armas_theme(theme.clone());

        // Get all pages (flat list for index mapping)
        let pages = showcase_gen::get_pages();

        // Get sections (grouped for sidebar)
        let sections = showcase_gen::get_sections();

        // Build sidebar items with sections as parents and pages as children
        let sidebar_items: Vec<SidebarItem> = sections
            .iter()
            .map(|(section_name, section_pages)| {
                let children: Vec<SidebarItem> = section_pages
                    .iter()
                    .map(|(page_name, _)| SidebarItem::new("", *page_name))
                    .collect();

                SidebarItem::new("", *section_name)
                    .with_children(children)
                    .expanded(true) // Expand sections by default
            })
            .collect();

        Self {
            theme,
            search_text: String::new(),
            selected_page_index: 0,
            pages,
            sidebar: Sidebar::new(sidebar_items)
                .expanded(true)
                .collapsible(false)
                .show_icons(false)
                .use_buttons(true)
                .button_variant(armas::ButtonVariant::Text),
        }
    }
}

impl eframe::App for ShowcaseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set the panel background to pure black
        let mut style = (*ctx.style()).clone();
        style.visuals.panel_fill = egui::Color32::from_rgb(0, 0, 0);
        style.visuals.window_fill = egui::Color32::from_rgb(0, 0, 0);
        style.visuals.extreme_bg_color = egui::Color32::from_rgb(0, 0, 0);
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                // Header (fixed height)
                self.show_header(ui);

                // Main layout: Sidebar + Content (always showing components)
                self.show_components_layout(ui);
            });
        });

        ctx.request_repaint();
    }
}

impl ShowcaseApp {
    fn show_header(&mut self, ui: &mut egui::Ui) {
        Header::show(ui, &mut self.search_text);
    }

    fn show_components_layout(&mut self, ui: &mut egui::Ui) {
        let available_rect = ui.available_rect_before_wrap();
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

        // Render sidebar
        let _ = ui.scope_builder(egui::UiBuilder::new().max_rect(sidebar_rect), |ui| {
            self.show_sidebar(ui);
        });

        // Render content
        let _ = ui.scope_builder(egui::UiBuilder::new().max_rect(content_rect), |ui| {
            self.show_content(ui);
        });
    }

    fn show_sidebar(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();

        // Wrap sidebar in GlassPanel
        GlassPanel::new()
            .blur(10.0)
            .opacity(0.03)
            .show(ui, &theme, |ui| {
                let response = self.sidebar.show(ui);

                // Handle sidebar clicks (child items only, parents are just sections)
                if let Some((parent_idx, child_idx)) = response.clicked_child {
                    // Calculate flat index from parent/child indices
                    let sections = showcase_gen::get_sections();
                    let mut flat_index = 0;
                    for (i, (_, section_pages)) in sections.iter().enumerate() {
                        if i < parent_idx {
                            flat_index += section_pages.len();
                        } else if i == parent_idx {
                            flat_index += child_idx;
                            break;
                        }
                    }
                    self.selected_page_index = flat_index;
                }
            });
    }

    fn show_content(&mut self, ui: &mut egui::Ui) {

        // Clean background with scroll area
        egui::ScrollArea::vertical().show(ui, |ui| {
            // Max width container for optimal reading
            let max_width = 1200.0;
            let available_width = ui.available_width();
            let content_width = available_width.min(max_width);
            let margin = (available_width - content_width) / 2.0;

            ui.add_space(margin);

            ui.vertical(|ui| {
                ui.set_max_width(content_width);
                ui.add_space(32.0);

                // Show selected page content
                if let Some((_, show_fn)) = self.pages.get(self.selected_page_index) {
                    show_fn(ui);
                }

                ui.add_space(64.0);
            });
        });
    }
}
