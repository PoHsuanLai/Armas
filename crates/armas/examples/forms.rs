//! Form & Input Components Showcase
//!
//! Demonstrates Phase 6 form and input components

use armas::{
    Input, InputState, InputVariant, SearchInput, Select, SelectOption, Theme, Toggle, ToggleGroup,
    ToggleSize, ToggleVariant,
};
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_title("Armas - Form Components"),
        ..Default::default()
    };

    eframe::run_native(
        "Forms",
        options,
        Box::new(|_cc| Ok(Box::new(FormsApp::new()))),
    )
}

struct FormsApp {
    theme: Theme,
    // Input states
    text_input: String,
    email_input: String,
    password_input: String,
    search_input: String,
    error_input: String,
    // Select states
    country_select: Select,
    framework_select: Select,
    // Toggle states
    notifications_toggle: Toggle,
    notifications_enabled: bool,
    dark_mode_toggle: Toggle,
    dark_mode_enabled: bool,
    wifi_toggle: Toggle,
    wifi_enabled: bool,
    bluetooth_toggle: Toggle,
    bluetooth_enabled: bool,
    // Checkbox states
    accept_terms_toggle: Toggle,
    accept_terms: bool,
    subscribe_toggle: Toggle,
    subscribe_newsletter: bool,
    remember_me_toggle: Toggle,
    remember_me: bool,
    // Toggle groups
    privacy_settings: ToggleGroup,
    feature_flags: ToggleGroup,
}

impl FormsApp {
    fn new() -> Self {
        let theme = Theme::dark();

        // Country select
        let countries = vec![
            SelectOption::new("us", "United States"),
            SelectOption::new("uk", "United Kingdom"),
            SelectOption::new("ca", "Canada"),
            SelectOption::new("au", "Australia"),
            SelectOption::new("de", "Germany"),
            SelectOption::new("fr", "France"),
            SelectOption::new("jp", "Japan"),
            SelectOption::new("cn", "China"),
        ];

        // Framework select with descriptions
        let frameworks = vec![
            SelectOption::new("react", "React")
                .with_description("A JavaScript library for building user interfaces"),
            SelectOption::new("vue", "Vue.js")
                .with_description("The Progressive JavaScript Framework"),
            SelectOption::new("angular", "Angular")
                .with_description("Platform for building mobile and desktop web applications"),
            SelectOption::new("svelte", "Svelte")
                .with_description("Cybernetically enhanced web apps"),
            SelectOption::new("solid", "SolidJS")
                .with_description("Simple and performant reactivity for building user interfaces"),
            SelectOption::new("other", "Other").disabled(true),
        ];

        // Privacy settings toggle group
        let privacy_settings = ToggleGroup::new()
            .with_label("Privacy Settings")
            .add_toggle(
                "analytics",
                Toggle::new()
                    .variant(ToggleVariant::Switch)
                    .with_label("Analytics")
                    .with_description("Help improve the app by sharing usage data"),
                false,
            )
            .add_toggle(
                "personalization",
                Toggle::new()
                    .variant(ToggleVariant::Switch)
                    .with_label("Personalization")
                    .with_description("Personalize your experience based on activity"),
                true,
            )
            .add_toggle(
                "tracking",
                Toggle::new()
                    .variant(ToggleVariant::Switch)
                    .with_label("Cross-site Tracking")
                    .with_description("Allow tracking across different websites")
                    .disabled(true),
                false,
            );

        // Feature flags toggle group
        let feature_flags = ToggleGroup::new()
            .with_label("Experimental Features")
            .add_toggle(
                "beta_ui",
                Toggle::new()
                    .variant(ToggleVariant::Checkbox)
                    .size(ToggleSize::Small)
                    .with_label("Beta UI Components"),
                false,
            )
            .add_toggle(
                "advanced_search",
                Toggle::new()
                    .variant(ToggleVariant::Checkbox)
                    .size(ToggleSize::Small)
                    .with_label("Advanced Search"),
                true,
            )
            .add_toggle(
                "offline_mode",
                Toggle::new()
                    .variant(ToggleVariant::Checkbox)
                    .size(ToggleSize::Small)
                    .with_label("Offline Mode"),
                false,
            );

        Self {
            theme: theme.clone(),
            text_input: String::new(),
            email_input: String::new(),
            password_input: String::new(),
            search_input: String::new(),
            error_input: String::new(),
            country_select: Select::new(countries)
                .with_label("Country")
                .with_placeholder("Select your country...")
                .with_width(300.0),
            framework_select: Select::new(frameworks)
                .with_label("Preferred Framework")
                .with_placeholder("Choose a framework...")
                .with_width(400.0)
                .with_max_height(300.0),
            notifications_toggle: Toggle::new()
                .with_label("Enable Notifications")
                .with_description("Receive push notifications for updates"),
            notifications_enabled: true,
            dark_mode_toggle: Toggle::new()
                .size(ToggleSize::Large)
                .with_label("Dark Mode")
                .with_description("Use dark theme across the application"),
            dark_mode_enabled: true,
            wifi_toggle: Toggle::new().size(ToggleSize::Small).with_label("Wi-Fi"),
            wifi_enabled: true,
            bluetooth_toggle: Toggle::new()
                .size(ToggleSize::Small)
                .with_label("Bluetooth"),
            bluetooth_enabled: false,
            accept_terms_toggle: Toggle::new()
                .variant(ToggleVariant::Checkbox)
                .with_label("I accept the terms and conditions"),
            accept_terms: false,
            subscribe_toggle: Toggle::new()
                .variant(ToggleVariant::Checkbox)
                .with_label("Subscribe to newsletter"),
            subscribe_newsletter: true,
            remember_me_toggle: Toggle::new()
                .variant(ToggleVariant::Checkbox)
                .size(ToggleSize::Large)
                .with_label("Remember me"),
            remember_me: false,
            privacy_settings,
            feature_flags,
        }
    }
}

impl eframe::App for FormsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    ui.heading("Phase 6: Form & Input Components");
                    ui.add_space(10.0);
                    ui.label("Input Fields • Select Menus • Toggles & Switches");
                    ui.add_space(30.0);
                });

                // Main content in two columns
                ui.horizontal_top(|ui| {
                    ui.add_space(40.0);

                    // Left column
                    ui.vertical(|ui| {
                        ui.set_width(600.0);

                        // Input Fields Section
                        ui.heading("Input Fields");
                        ui.add_space(15.0);

                        // Basic input
                        Input::new("Enter your name")
                            .with_label("Name")
                            .with_width(400.0)
                            .show(ui, &mut self.text_input, &self.theme);
                        ui.add_space(12.0);

                        // Email input
                        Input::new("email@example.com")
                            .variant(InputVariant::Outlined)
                            .with_label("Email Address")
                            .with_left_icon("📧")
                            .with_width(400.0)
                            .show(ui, &mut self.email_input, &self.theme);
                        ui.add_space(12.0);

                        // Password input
                        Input::new("Enter password")
                            .variant(InputVariant::Filled)
                            .with_label("Password")
                            .with_left_icon("🔒")
                            .with_width(400.0)
                            .password(true)
                            .show(ui, &mut self.password_input, &self.theme);
                        ui.add_space(12.0);

                        // Error state input
                        let error_state = if self.error_input.is_empty() {
                            InputState::Normal
                        } else if self.error_input.len() < 3 {
                            InputState::Error
                        } else if self.error_input.len() < 6 {
                            InputState::Warning
                        } else {
                            InputState::Success
                        };

                        let helper_text = match error_state {
                            InputState::Error => "Username too short (min 3 characters)",
                            InputState::Warning => "Consider a longer username",
                            InputState::Success => "Username available!",
                            InputState::Normal => "Choose a unique username",
                        };

                        Input::new("Choose username")
                            .with_label("Username")
                            .with_left_icon("👤")
                            .state(error_state)
                            .with_helper_text(helper_text)
                            .with_width(400.0)
                            .show(ui, &mut self.error_input, &self.theme);
                        ui.add_space(12.0);

                        // Search input
                        ui.add_space(10.0);
                        SearchInput::new()
                            .with_placeholder("Search components...")
                            .with_width(400.0)
                            .show(ui, &mut self.search_input, &self.theme);

                        ui.add_space(30.0);

                        // Select Menus Section
                        ui.heading("Select Menus");
                        ui.add_space(15.0);

                        // Country select
                        let country_response = self.country_select.show(ui, &self.theme);
                        if country_response.changed {
                            if let Some(value) = country_response.selected_value {
                                println!("Selected country: {}", value);
                            }
                        }
                        ui.add_space(12.0);

                        // Framework select
                        let framework_response = self.framework_select.show(ui, &self.theme);
                        if framework_response.changed {
                            if let Some(value) = framework_response.selected_value {
                                println!("Selected framework: {}", value);
                            }
                        }
                    });

                    ui.add_space(80.0);

                    // Right column
                    ui.vertical(|ui| {
                        ui.set_width(500.0);

                        // Toggle Switches Section
                        ui.heading("Toggle Switches");
                        ui.add_space(15.0);

                        // Basic switches
                        self.notifications_toggle.show(
                            ui,
                            &mut self.notifications_enabled,
                            &self.theme,
                        );
                        ui.add_space(8.0);

                        self.dark_mode_toggle
                            .show(ui, &mut self.dark_mode_enabled, &self.theme);
                        ui.add_space(8.0);

                        // System toggles
                        ui.add_space(15.0);
                        ui.label(
                            egui::RichText::new("System Settings")
                                .size(14.0)
                                .strong()
                                .color(self.theme.on_surface()),
                        );
                        ui.add_space(8.0);

                        ui.horizontal(|ui| {
                            self.wifi_toggle
                                .show(ui, &mut self.wifi_enabled, &self.theme);
                            ui.add_space(40.0);
                            self.bluetooth_toggle.show(
                                ui,
                                &mut self.bluetooth_enabled,
                                &self.theme,
                            );
                        });

                        ui.add_space(30.0);

                        // Checkboxes Section
                        ui.heading("Checkboxes");
                        ui.add_space(15.0);

                        self.accept_terms_toggle
                            .show(ui, &mut self.accept_terms, &self.theme);
                        ui.add_space(8.0);

                        self.subscribe_toggle
                            .show(ui, &mut self.subscribe_newsletter, &self.theme);
                        ui.add_space(8.0);

                        self.remember_me_toggle
                            .show(ui, &mut self.remember_me, &self.theme);

                        ui.add_space(30.0);

                        // Toggle Groups Section
                        ui.heading("Toggle Groups");
                        ui.add_space(15.0);

                        let privacy_response = self.privacy_settings.show(ui, &self.theme);
                        if !privacy_response.changed.is_empty() {
                            for (id, state) in privacy_response.changed {
                                println!("Privacy setting '{}' changed to: {}", id, state);
                            }
                        }

                        ui.add_space(20.0);

                        let feature_response = self.feature_flags.show(ui, &self.theme);
                        if !feature_response.changed.is_empty() {
                            for (id, state) in feature_response.changed {
                                println!("Feature '{}' changed to: {}", id, state);
                            }
                        }
                    });
                });

                ui.add_space(30.0);
                ui.separator();
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.add_space(40.0);
                    ui.label("✅ Phase 6 Complete!");
                    ui.add_space(20.0);
                    ui.label("Components: Input • SearchInput • Select • Toggle • ToggleGroup");
                    ui.add_space(20.0);
                    ui.label("Features: Validation • Icons • Search • Animations");
                });

                ui.add_space(30.0);
            });
        });
    }
}
