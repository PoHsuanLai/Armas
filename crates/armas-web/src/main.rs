#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use armas::*;
use eframe::egui;

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
    current_page: Page,
    selected_category: Category,
    selected_component: Option<String>,

    // Component states
    spinner: Spinner,
    loading_dots: LoadingDots,
    skeleton: Skeleton,
    circular_progress: CircularProgress,
    scrolling_banner: ScrollingBanner,
    spotlight: Spotlight,
    gradient_card: GradientCard,
    accordion_items: Vec<AccordionItem>,
    tabs: AnimatedTabs,
    fader_value: f32,
    progress_value: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Page {
    Home,
    Docs,
    Components,
    Examples,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Category {
    GettingStarted,
    Animations,
    Cards,
    Buttons,
    Loading,
    Navigation,
    DataDisplay,
    Layout,
}

impl Category {
    fn all() -> Vec<Self> {
        vec![
            Self::GettingStarted,
            Self::Animations,
            Self::Cards,
            Self::Buttons,
            Self::Loading,
            Self::Navigation,
            Self::DataDisplay,
            Self::Layout,
        ]
    }

    fn label(&self) -> &str {
        match self {
            Self::GettingStarted => "Getting Started",
            Self::Animations => "Animations",
            Self::Cards => "Cards",
            Self::Buttons => "Buttons",
            Self::Loading => "Loading",
            Self::Navigation => "Navigation",
            Self::DataDisplay => "Data Display",
            Self::Layout => "Layout",
        }
    }
}

impl ShowcaseApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let theme = Theme::dark();
        Self {
            spinner: Spinner::new(&theme).size(40.0),
            loading_dots: LoadingDots::new(&theme),
            skeleton: Skeleton::new(300.0, 20.0),
            circular_progress: CircularProgress::new(&theme),
            scrolling_banner: ScrollingBanner::new()
                .speed(50.0)
                .direction(ScrollDirection::Left)
                .pause_on_hover(true),
            spotlight: Spotlight::new(&theme).radius(200.0).smoothing(0.15),
            theme,
            current_page: Page::Home,
            selected_category: Category::GettingStarted,
            selected_component: None,
            gradient_card: GradientCard::rainbow().width(300.0).height(200.0),
            accordion_items: vec![
                AccordionItem::new("Section 1").open(false),
                AccordionItem::new("Section 2").open(false),
                AccordionItem::new("Section 3").open(false),
            ],
            tabs: AnimatedTabs::new(vec!["Tab 1".to_string(), "Tab 2".to_string(), "Tab 3".to_string()]),
            fader_value: 0.75,
            progress_value: 0.6,
        }
    }
}

impl eframe::App for ShowcaseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            VStack::new(0.0).show(ui, |ui| {
                // Header (fixed height)
                self.show_header(ui);

                // Main layout: Sidebar + Content with flex sizing
                if self.current_page == Page::Components {
                    self.show_components_layout(ui);
                } else {
                    self.show_content(ui);
                }
            });
        });

        ctx.request_repaint();
    }
}

impl ShowcaseApp {
    fn show_header(&mut self, ui: &mut egui::Ui) {
        Card::new().inner_margin(16.0).show(ui, &self.theme, |ui| {
            ui.set_min_height(70.0);

            HStack::new(24.0).show(ui, |ui| {
                    // Logo and brand
                    VStack::new(2.0).show(ui, |ui| {
                        ui.heading(egui::RichText::new("Armas").size(24.0).strong());
                        ui.label(egui::RichText::new("Component Library").size(12.0).color(self.theme.on_surface()));
                    });

                    Spacer::new().show(ui);

                    // Navigation links (center)
                    HStack::new(32.0).show(ui, |ui| {
                        let nav_items = [
                            ("Home", Page::Home),
                            ("Docs", Page::Docs),
                            ("Components", Page::Components),
                            ("Examples", Page::Examples),
                        ];

                        for (label, page) in nav_items {
                            let is_active = self.current_page == page;
                            if ui.selectable_label(is_active, label).clicked() {
                                self.current_page = page;
                            }
                        }
                    });

                    Spacer::new().show(ui);

                    // Right side: Search + GitHub + Theme toggle
                    HStack::new(12.0).show(ui, |ui| {
                        // Search placeholder (can be replaced with actual search later)
                        Badge::new("🔍 Search").variant(BadgeVariant::Outlined).show(ui, &self.theme);

                        // GitHub button
                        if Button::new("GitHub")
                            .variant(ButtonVariant::Outlined)
                            .show(ui, &self.theme)
                            .clicked()
                        {
                            #[cfg(target_arch = "wasm32")]
                            {
                                if let Some(window) = web_sys::window() {
                                    let _ = window.open_with_url("https://github.com/your-repo/armas");
                                }
                            }
                        }

                        // Theme toggle button
                        if Button::new("🌙")
                            .variant(ButtonVariant::Filled)
                            .show(ui, &self.theme)
                            .clicked()
                        {
                            // TODO: Toggle theme
                        }
                    });
                });
        });

        Divider::horizontal().show(ui, &self.theme);
    }

    fn show_components_layout(&mut self, ui: &mut egui::Ui) {
        // Use horizontal layout with proper sizing
        ui.horizontal(|ui| {
            // Sidebar with fixed width
            ui.allocate_ui(egui::vec2(240.0, ui.available_height()), |ui| {
                self.show_sidebar(ui);
            });

            // Content takes remaining space
            ui.allocate_ui(egui::vec2(ui.available_width(), ui.available_height()), |ui| {
                self.show_content(ui);
            });
        });
    }

    fn show_sidebar(&mut self, ui: &mut egui::Ui) {
        Card::new().inner_margin(16.0).show(ui, &self.theme, |ui| {
            ui.set_min_height(ui.available_height());

            VStack::new(0.0).show(ui, |ui| {
                // Sidebar header
                Container::new(ContainerSize::Full).show(ui, &self.theme, |ui| {
                    ui.add_space(16.0);
                    ui.strong(egui::RichText::new("Components").size(16.0));
                    ui.add_space(8.0);
                });

                Divider::horizontal().show(ui, &self.theme);

                // Scrollable category list
                ScrollView::vertical().show(ui, |ui| {
                    VStack::new(4.0).show(ui, |ui| {
                        ui.add_space(8.0);

                        for category in Category::all() {
                            let is_selected = self.selected_category == category;

                            // Custom styled category button
                            ui.horizontal(|ui| {
                                ui.add_space(12.0);

                                let button = if is_selected {
                                    Button::new(category.label()).variant(ButtonVariant::Filled)
                                } else {
                                    Button::new(category.label()).variant(ButtonVariant::Text)
                                };

                                if button.show(ui, &self.theme).clicked() {
                                    self.selected_category = category;
                                    self.selected_component = None;
                                }
                            });
                        }

                        ui.add_space(16.0);
                    });
                });
            });
        });
    }

    fn show_content(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();
        egui::ScrollArea::vertical().show(ui, |ui| {
            VStack::new(16.0).show(ui, |ui| {
                    Spacer::fixed(16.0).show(ui);

                    // Route to different pages
                    match self.current_page {
                        Page::Home => self.show_home_page(ui),
                        Page::Docs => self.show_docs_page(ui),
                        Page::Components => {
                            // Show component category content
                            match self.selected_category {
                                Category::GettingStarted => self.show_getting_started(ui),
                                Category::Animations => self.show_animations(ui),
                                Category::Cards => self.show_cards(ui),
                                Category::Buttons => self.show_buttons(ui),
                                Category::Loading => self.show_loading(ui),
                                Category::Navigation => self.show_navigation(ui),
                                Category::DataDisplay => self.show_data_display(ui),
                                Category::Layout => self.show_layout(ui),
                            }
                        }
                        Page::Examples => self.show_examples_page(ui),
                    }

                    Spacer::fixed(32.0).show(ui);
                });
        });
    }

    fn show_getting_started(&mut self, ui: &mut egui::Ui) {
        ui.heading("Welcome to Armas");
        ui.add_space(8.0);
        ui.label("A modern component library for egui with Material Design theming.");
        ui.add_space(16.0);

        ui.heading("Features");
        ui.add_space(8.0);
        ui.label("- Material Design inspired theming");
        ui.label("- Smooth animations and transitions");
        ui.label("- Rich collection of components");
        ui.label("- Builder pattern API");
        ui.label("- Serializable themes");
        ui.add_space(16.0);

        ui.heading("Quick Start");
        ui.add_space(8.0);
        ui.code("cargo add armas");
        ui.add_space(8.0);

        ui.label("Example usage:");
        ui.code_editor(&mut r#"use armas::*;

let theme = Theme::ocean();
Button::new("Click me")
    .variant(ButtonVariant::Filled)
    .show(ui, &theme);"#.to_string());
    }

    fn show_animations(&mut self, ui: &mut egui::Ui) {
        ui.heading("Animations");
        ui.add_space(16.0);

        ui.heading("Scrolling Banner");
        ui.add_space(8.0);
        self.scrolling_banner.show(ui, &self.theme, |ui, index| {
            ui.horizontal(|ui| {
                ui.label(format!("Item {}", index + 1));
                ui.separator();
            });
        });
        ui.add_space(16.0);

        ui.heading("Spotlight Effect");
        ui.add_space(8.0);
        self.spotlight.show(ui, &self.theme, |ui| {
            ui.set_min_size(egui::vec2(400.0, 200.0));
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);
                ui.heading("Hover to see the spotlight");
                ui.label("Mouse tracking radial gradient");
            });
        });
        ui.add_space(16.0);

        ui.heading("Gradient Card");
        ui.add_space(8.0);
        self.gradient_card.show(ui, &self.theme, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);
                ui.heading("Premium Content");
                ui.label("Animated gradient border");
            });
        });
    }

    fn show_cards(&mut self, ui: &mut egui::Ui) {
        ui.heading("Cards");
        ui.add_space(16.0);

        ui.heading("Basic Card");
        ui.add_space(8.0);
        Card::new().width(300.0).show(ui, &self.theme, |ui| {
            ui.set_min_height(150.0);
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.heading("Card Title");
                ui.label("Card content goes here");
            });
        });
        ui.add_space(16.0);

        ui.heading("Glass Panel");
        ui.add_space(8.0);
        GlassPanel::new().width(300.0).show(ui, &self.theme, |ui| {
            ui.set_min_height(150.0);
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.heading("Glass Effect");
                ui.label("Glassmorphism style");
            });
        });
    }

    fn show_buttons(&mut self, ui: &mut egui::Ui) {
        ui.heading("Buttons");
        ui.add_space(16.0);

        ui.label("Button Variants:");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            Button::new("Filled").variant(ButtonVariant::Filled).show(ui, &self.theme);
            Button::new("Outlined").variant(ButtonVariant::Outlined).show(ui, &self.theme);
            Button::new("Text").variant(ButtonVariant::Text).show(ui, &self.theme);
        });

        ui.add_space(16.0);
        ui.label("Badge:");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            Badge::new("New").variant(BadgeVariant::Filled).show(ui, &self.theme);
            Badge::new("Hot").color(BadgeColor::Error).show(ui, &self.theme);
            NotificationBadge::new(5, &self.theme).show(ui);
        });
    }

    fn show_loading(&mut self, ui: &mut egui::Ui) {
        ui.heading("Loading Indicators");
        ui.add_space(16.0);

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Spinner");
                self.spinner.show(ui, &self.theme);
            });

            ui.add_space(32.0);

            ui.vertical(|ui| {
                ui.label("Loading Dots");
                self.loading_dots.show(ui, &self.theme);
            });

            ui.add_space(32.0);

            ui.vertical(|ui| {
                ui.label("Circular Progress");
                self.circular_progress.show(ui, &self.theme);
            });
        });

        ui.add_space(16.0);
        ui.label("Skeleton");
        ui.add_space(8.0);
        self.skeleton.show(ui, &self.theme);

        ui.add_space(16.0);
        ui.label("Progress Bars");
        ui.add_space(8.0);
        LinearProgress::new(self.progress_value).width(300.0).show(ui, &self.theme);
        ui.add_space(8.0);
        RingProgress::new(self.progress_value).size(60.0).show(ui, &self.theme);
    }

    fn show_navigation(&mut self, ui: &mut egui::Ui) {
        ui.heading("Navigation");
        ui.add_space(16.0);

        ui.label("Tabs:");
        ui.add_space(8.0);
        if let Some(new_index) = self.tabs.show(ui, &self.theme) {
            ui.label(format!("Tab changed to: {}", new_index));
        } else {
            ui.label(format!("Current tab: {}", self.tabs.active_index));
        }

        ui.add_space(16.0);
        ui.label("Accordion:");
        ui.add_space(8.0);
        for item in &mut self.accordion_items {
            item.show(ui, &self.theme, |ui| {
                ui.label("Accordion item content goes here");
            });
        }
    }

    fn show_data_display(&mut self, ui: &mut egui::Ui) {
        ui.heading("Data Display");
        ui.add_space(16.0);

        ui.label("Timeline:");
        ui.add_space(8.0);
        let items = vec![
            TimelineItem::new("Started Project", "Initial setup and planning"),
            TimelineItem::new("Built Components", "Created core components"),
            TimelineItem::new("Added Animations", "Smooth animations system"),
        ];
        Timeline::new(items).show(ui, &self.theme);

        ui.add_space(16.0);
        ui.label("Fader:");
        ui.add_space(8.0);
        let fader = Fader::new(self.fader_value);
        let (_response, new_value) = fader.show(ui, &self.theme);
        self.fader_value = new_value;
        ui.label(format!("Value: {:.2}", self.fader_value));
    }

    fn show_layout(&mut self, ui: &mut egui::Ui) {
        ui.heading("Layout");
        ui.add_space(16.0);

        ui.label("Feature Grid:");
        ui.add_space(8.0);
        FeatureGrid::new(vec![
            FeatureItem::new("F", "Fast", "Lightning fast performance"),
            FeatureItem::new("M", "Modern", "Modern design language"),
            FeatureItem::new("X", "Flexible", "Highly customizable"),
            FeatureItem::new("S", "Simple", "Easy to use API"),
        ]).show(ui, &self.theme);

        ui.add_space(16.0);
        ui.label("Testimonial Grid:");
        ui.add_space(8.0);
        TestimonialGrid::new(vec![
            TestimonialItem::new(
                "Great library!",
                "Makes building UIs so much easier",
                "User 1",
            ),
            TestimonialItem::new(
                "Love it",
                "Beautiful components out of the box",
                "User 2",
            ),
        ]).show(ui, &self.theme);
    }

    fn show_home_page(&mut self, ui: &mut egui::Ui) {
        VStack::new(32.0).show(ui, |ui| {
            // Hero section
            ui.vertical_centered(|ui| {
                ui.add_space(48.0);
                ui.heading(egui::RichText::new("Armas").size(48.0).strong());
                ui.add_space(8.0);
                ui.label(egui::RichText::new("Modern Component Library for egui").size(20.0));
                ui.add_space(16.0);
                ui.label("Build beautiful UIs with Material Design inspired components");
                ui.add_space(32.0);

                HStack::new(16.0).show(ui, |ui| {
                    if Button::new("Get Started")
                        .variant(ButtonVariant::Filled)
                        .show(ui, &self.theme)
                        .clicked()
                    {
                        self.current_page = Page::Components;
                    }

                    if Button::new("View Docs")
                        .variant(ButtonVariant::Outlined)
                        .show(ui, &self.theme)
                        .clicked()
                    {
                        self.current_page = Page::Docs;
                    }
                });
            });

            ui.add_space(64.0);

            // Features grid
            ui.heading("Features");
            ui.add_space(16.0);

            Grid::new(3).gap(24.0).id_source("features_grid").show(ui, |grid| {
                for (title, desc) in [
                    ("🎨 Material Design", "Beautiful, modern design inspired by Material Design"),
                    ("✨ Animations", "Smooth transitions and eye-catching effects"),
                    ("🧩 Rich Components", "30+ components ready to use"),
                    ("🎯 Builder Pattern", "Clean, declarative API"),
                    ("💾 Themeable", "Serializable themes with dark/light modes"),
                    ("📱 Responsive", "Layout components for any screen size"),
                ] {
                    grid.cell(|ui| {
                        Card::new().show(ui, &self.theme, |ui| {
                            VStack::new(8.0).show(ui, |ui| {
                                ui.strong(title);
                                ui.label(desc);
                            });
                        });
                    });
                }
            });
        });
    }

    fn show_docs_page(&mut self, ui: &mut egui::Ui) {
        VStack::new(24.0).show(ui, |ui| {
            ui.heading("Documentation");
            ui.add_space(8.0);
            ui.label("Learn how to use Armas components in your egui applications.");

            Divider::horizontal().show(ui, &self.theme);

            ui.heading("Installation");
            ui.add_space(8.0);
            ui.label("Add Armas to your Cargo.toml:");
            ui.code("armas = \"0.1.0\"");
            ui.add_space(16.0);

            ui.heading("Quick Start");
            ui.add_space(8.0);
            ui.label("Import the components you need:");
            ui.code("use armas::*;");
            ui.add_space(16.0);

            ui.heading("Basic Usage");
            ui.add_space(8.0);
            ui.label("Most components use a builder pattern:");
            Card::new().show(ui, &self.theme, |ui| {
                ui.code("Button::new(\"Click me\")\n    .variant(ButtonVariant::Filled)\n    .show(ui, &theme);");
            });
            ui.add_space(16.0);

            ui.heading("Theming");
            ui.add_space(8.0);
            ui.label("Create a theme and use it throughout your app:");
            Card::new().show(ui, &self.theme, |ui| {
                ui.code("let theme = Theme::dark();\n// or\nlet theme = Theme::light();");
            });
        });
    }

    fn show_examples_page(&mut self, ui: &mut egui::Ui) {
        VStack::new(24.0).show(ui, |ui| {
            ui.heading("Examples");
            ui.add_space(8.0);
            ui.label("Explore real-world examples and use cases.");

            Divider::horizontal().show(ui, &self.theme);

            // Example 1: Form
            ui.heading("Form Example");
            ui.add_space(8.0);
            Card::new().show(ui, &self.theme, |ui| {
                VStack::new(16.0).show(ui, |ui| {
                    ui.strong("User Profile Form");

                    FormLayout::new()
                        .id_source("example_form")
                        .show(ui, |form| {
                            form.row("Name:", |ui| {
                                ui.text_edit_singleline(&mut String::new());
                            });
                            form.row("Email:", |ui| {
                                ui.text_edit_singleline(&mut String::new());
                            });
                            form.row("Role:", |ui| {
                                Badge::new("User").show(ui, &self.theme);
                            });
                        });

                    HStack::new(8.0).show(ui, |ui| {
                        Button::new("Save").variant(ButtonVariant::Filled).show(ui, &self.theme);
                        Button::new("Cancel").variant(ButtonVariant::Outlined).show(ui, &self.theme);
                    });
                });
            });

            ui.add_space(24.0);

            // Example 2: Data Table
            ui.heading("Table Example");
            ui.add_space(8.0);
            Card::new().show(ui, &self.theme, |ui| {
                Table::new()
                    .striped(true)
                    .id_source("example_table")
                    .show(ui, &self.theme, |table| {
                        table.headers(&["Product", "Price", "Stock"]);

                        table.row(|row| {
                            row.col_label("Widget A");
                            row.col_label("$29.99");
                            row.col(|ui| {
                                Badge::new("In Stock").color(BadgeColor::Success).show(ui, &self.theme);
                            });
                        });

                        table.row(|row| {
                            row.col_label("Widget B");
                            row.col_label("$49.99");
                            row.col(|ui| {
                                Badge::new("Low Stock").color(BadgeColor::Warning).show(ui, &self.theme);
                            });
                        });

                        table.row(|row| {
                            row.col_label("Widget C");
                            row.col_label("$19.99");
                            row.col(|ui| {
                                Badge::new("Out of Stock").color(BadgeColor::Error).show(ui, &self.theme);
                            });
                        });
                    });
            });
        });
    }
}
