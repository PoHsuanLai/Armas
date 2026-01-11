use armas::ext::ArmasContextExt;
use armas::{Avatar, AvatarShape, AvatarSize, BadgeColor, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Avatar Component Example",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(AvatarExample::default()))
        }),
    )
}

struct AvatarExample {
    clicked_avatar: Option<String>,
}

impl Default for AvatarExample {
    fn default() -> Self {
        Self {
            clicked_avatar: None,
        }
    }
}

impl eframe::App for AvatarExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let theme = ui.ctx().armas_theme();

            ui.heading("Avatar Component Examples");
            ui.add_space(20.0);

            if let Some(name) = &self.clicked_avatar {
                ui.label(format!("Last clicked: {}", name));
                ui.add_space(10.0);
            }

            // Different sizes
            ui.label("Different Sizes:");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                Avatar::new("XS")
                    .size(AvatarSize::XSmall)
                    .show(ui);

                ui.add_space(8.0);

                Avatar::new("SM")
                    .size(AvatarSize::Small)
                    .show(ui);

                ui.add_space(8.0);

                Avatar::new("MD")
                    .size(AvatarSize::Medium)
                    .show(ui);

                ui.add_space(8.0);

                Avatar::new("LG")
                    .size(AvatarSize::Large)
                    .show(ui);

                ui.add_space(8.0);

                Avatar::new("XL")
                    .size(AvatarSize::XLarge)
                    .show(ui);
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Different shapes
            ui.label("Different Shapes:");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                Avatar::new("JD")
                    .size(AvatarSize::Large)
                    .shape(AvatarShape::Circle)
                    .show(ui);

                ui.add_space(12.0);

                Avatar::new("AM")
                    .size(AvatarSize::Large)
                    .shape(AvatarShape::RoundedSquare)
                    .show(ui);

                ui.add_space(12.0);

                Avatar::new("KL")
                    .size(AvatarSize::Large)
                    .shape(AvatarShape::Square)
                    .show(ui);
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // With borders
            ui.label("With Borders:");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                Avatar::new("AB")
                    .size(AvatarSize::Large)
                    .with_border(true)
                    .show(ui);

                ui.add_space(12.0);

                Avatar::new("CD")
                    .size(AvatarSize::Large)
                    .shape(AvatarShape::RoundedSquare)
                    .with_border(true)
                    .show(ui);
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Custom colors
            ui.label("Custom Colors:");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                Avatar::new("PR")
                    .size(AvatarSize::Large)
                    .color(theme.primary())
                    .show(ui);

                ui.add_space(12.0);

                Avatar::new("ER")
                    .size(AvatarSize::Large)
                    .color(theme.error())
                    .show(ui);

                ui.add_space(12.0);

                Avatar::new("SC")
                    .size(AvatarSize::Large)
                    .color(theme.success())
                    .show(ui);

                ui.add_space(12.0);

                Avatar::new("WN")
                    .size(AvatarSize::Large)
                    .color(theme.warning())
                    .show(ui);
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // With status indicators
            ui.label("With Status Indicators:");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                Avatar::new("ON")
                    .size(AvatarSize::Large)
                    .status(BadgeColor::Success)
                    .show(ui);
                ui.label("Online");

                ui.add_space(20.0);

                Avatar::new("AW")
                    .size(AvatarSize::Large)
                    .status(BadgeColor::Warning)
                    .show(ui);
                ui.label("Away");

                ui.add_space(20.0);

                Avatar::new("DND")
                    .size(AvatarSize::Large)
                    .status(BadgeColor::Error)
                    .show(ui);
                ui.label("Do Not Disturb");

                ui.add_space(20.0);

                Avatar::new("OF")
                    .size(AvatarSize::Large)
                    .status(BadgeColor::Neutral)
                    .show(ui);
                ui.label("Offline");
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Clickable avatars
            ui.label("Clickable Avatars:");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if Avatar::new("JD")
                    .size(AvatarSize::Large)
                    .clickable()
                    .status(BadgeColor::Success)
                    .show(ui)
                    .clicked()
                {
                    self.clicked_avatar = Some("John Doe".to_string());
                }

                ui.add_space(12.0);

                if Avatar::new("AM")
                    .size(AvatarSize::Large)
                    .clickable()
                    .status(BadgeColor::Warning)
                    .show(ui)
                    .clicked()
                {
                    self.clicked_avatar = Some("Alice Miller".to_string());
                }

                ui.add_space(12.0);

                if Avatar::new("BW")
                    .size(AvatarSize::Large)
                    .clickable()
                    .status(BadgeColor::Success)
                    .show(ui)
                    .clicked()
                {
                    self.clicked_avatar = Some("Bob Wilson".to_string());
                }
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // User list example
            ui.label("User List Example:");
            ui.add_space(10.0);

            let users = vec![
                ("Sarah Johnson", "SJ", BadgeColor::Success),
                ("Mike Chen", "MC", BadgeColor::Success),
                ("Emma Davis", "ED", BadgeColor::Warning),
                ("James Brown", "JB", BadgeColor::Error),
                ("Lisa Wang", "LW", BadgeColor::Neutral),
            ];

            for (name, initials, status) in users {
                ui.horizontal(|ui| {
                    if Avatar::new(initials)
                        .size(AvatarSize::Medium)
                        .clickable()
                        .status(status)
                        .show(ui)
                        .clicked()
                    {
                        self.clicked_avatar = Some(name.to_string());
                    }

                    ui.add_space(12.0);
                    ui.label(name);
                });
                ui.add_space(8.0);
            }

            ui.add_space(20.0);

            ui.label("💡 Tip: Hover over clickable avatars to see the interaction effect!");
        });
    }
}
