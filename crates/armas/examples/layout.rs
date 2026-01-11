use armas::ext::ArmasContextExt;
use armas::layout::{
    AspectRatio, Container, ContainerSize, ContentMode, Divider, FormLayout, Grid, HStack,
    ScrollView, Spacer, Table, VStack, ZStack,
};
use armas::{Badge, BadgeColor, BadgeVariant, Button, ButtonVariant, Card, Theme};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_title("Armas Layout Components - Complete Demo"),
        ..Default::default()
    };

    eframe::run_simple_native("Layout Demo", native_options, move |ctx, _frame| {
        // Set theme once per frame (for run_simple_native pattern)
        ctx.set_armas_theme(Theme::dark());
        let theme = ctx.armas_theme();

        egui::CentralPanel::default().show(ctx, |ui| {

            ScrollView::vertical().show(ui, |ui| {
                Container::new(ContainerSize::Large).show(ui, &theme, |ui| {
                    VStack::new(theme.spacing.xl).show(ui, |ui| {
                        // Title
                        ui.heading("Armas Layout Components");
                        ui.label("SwiftUI-inspired layout system for egui");

                        Divider::horizontal().show(ui);

                        // VStack & HStack Demo
                        demo_section(ui, &theme, "VStack & HStack", |ui| {
                            Container::new(ContainerSize::Medium).show(ui, &theme, |ui| {
                                HStack::new(theme.spacing.lg).show(ui, |ui| {
                                    Card::new().width(300.0).show(ui, &theme, |ui| {
                                        VStack::new(theme.spacing.md).show(ui, |ui| {
                                            ui.strong("VStack");
                                            ui.label("Vertical spacing: 16px");
                                            Button::new("Button 1").show(ui);
                                            Button::new("Button 2").show(ui);
                                            Button::new("Button 3").show(ui);
                                        });
                                    });

                                    Card::new().width(300.0).show(ui, &theme, |ui| {
                                        VStack::new(theme.spacing.md).show(ui, |ui| {
                                            ui.strong("HStack");
                                            ui.label("Horizontal spacing: 8px");
                                            HStack::new(theme.spacing.sm).show(ui, |ui| {
                                                Button::new("A").show(ui);
                                                Button::new("B").show(ui);
                                                Button::new("C").show(ui);
                                            });
                                        });
                                    });
                                });
                            });
                        });

                        // Spacer Demo
                        demo_section(ui, &theme, "Spacer", |ui| {
                            Container::new(ContainerSize::Medium).show(ui, &theme, |ui| {
                                Card::new().show(ui, &theme, |ui| {
                                    VStack::new(theme.spacing.md).show(ui, |ui| {
                                        ui.strong("Flexible Spacer (pushes items apart)");
                                        HStack::new(0.0).show(ui, |ui| {
                                            Button::new("Left").show(ui);
                                            Spacer::new().show(ui);
                                            Button::new("Right").show(ui);
                                        });

                                        Spacer::md(&theme).show(ui);

                                        ui.strong("Theme Spacing");
                                        HStack::new(0.0).show(ui, |ui| {
                                            ui.label("xs");
                                            Spacer::xs(&theme).show(ui);
                                            ui.label("sm");
                                            Spacer::sm(&theme).show(ui);
                                            ui.label("md");
                                            Spacer::md(&theme).show(ui);
                                            ui.label("lg");
                                            Spacer::lg(&theme).show(ui);
                                            ui.label("xl");
                                        });
                                    });
                                });
                            });
                        });

                        // Divider Demo
                        demo_section(ui, &theme, "Divider", |ui| {
                            Container::new(ContainerSize::Medium).show(ui, &theme, |ui| {
                                HStack::new(theme.spacing.lg).show(ui, |ui| {
                                    Card::new().width(300.0).show(ui, &theme, |ui| {
                                        VStack::new(theme.spacing.md).show(ui, |ui| {
                                            ui.strong("Horizontal Dividers");
                                            ui.label("Section 1");
                                            Divider::horizontal().show(ui);
                                            ui.label("Section 2");
                                            Divider::horizontal().thickness(2.0).show(ui);
                                            ui.label("Section 3");
                                        });
                                    });

                                    Card::new().width(300.0).show(ui, &theme, |ui| {
                                        ui.set_min_height(150.0);
                                        ui.strong("Vertical Divider");
                                        HStack::new(theme.spacing.md).show(ui, |ui| {
                                            ui.label("Left");
                                            Divider::vertical().height(100.0).show(ui);
                                            ui.label("Middle");
                                            Divider::vertical().height(100.0).show(ui);
                                            ui.label("Right");
                                        });
                                    });
                                });
                            });
                        });

                        // Grid Demo
                        demo_section(ui, &theme, "Grid", |ui| {
                            Container::new(ContainerSize::Medium).show(ui, &theme, |ui| {
                                Card::new().show(ui, &theme, |ui| {
                                    VStack::new(theme.spacing.md).show(ui, |ui| {
                                        ui.strong("3-Column Grid");

                                        Grid::new(3)
                                            .gap(theme.spacing.md)
                                            .id_source("demo_grid")
                                            .show(ui, |grid| {
                                                for i in 1..=6 {
                                                    grid.cell(|ui| {
                                                        Card::new().show(
                                                            ui,
                                                            &theme,
                                                            |ui| {
                                                                ui.set_min_height(80.0);
                                                                ui.vertical_centered(|ui| {
                                                                    ui.add_space(20.0);
                                                                    ui.heading(format!(
                                                                        "Item {}",
                                                                        i
                                                                    ));
                                                                    Badge::new(format!("#{}", i))
                                                                        .color(BadgeColor::Info)
                                                                        .show(ui);
                                                                });
                                                            },
                                                        );
                                                    });
                                                }
                                            });
                                    });
                                });
                            });
                        });

                        // FormLayout Demo
                        demo_section(ui, &theme, "FormLayout\n(Easy label-value pairs)", |ui| {
                            Container::new(ContainerSize::Small).show(ui, &theme, |ui| {
                                Card::new().show(ui, &theme, |ui| {
                                    VStack::new(theme.spacing.md).show(ui, |ui| {
                                        ui.strong("User Profile");

                                        FormLayout::new()
                                            .gap(theme.spacing.md)
                                            .id_source("profile_form")
                                            .show(ui, |form| {
                                                form.row("Name:", |ui| {
                                                    ui.strong("John Doe");
                                                });
                                                form.row("Email:", |ui| {
                                                    ui.label("john@example.com");
                                                });
                                                form.row("Role:", |ui| {
                                                    Badge::new("Admin")
                                                        .color(BadgeColor::Success)
                                                        .show(ui);
                                                });
                                                form.row("Status:", |ui| {
                                                    Badge::new("Active")
                                                        .variant(BadgeVariant::Filled)
                                                        .color(BadgeColor::Info)
                                                        .show(ui);
                                                });
                                            });
                                    });
                                });
                            });
                        });

                        // Table Demo
                        demo_section(ui, &theme, "Table\n(Tabular data with headers)", |ui| {
                            Container::new(ContainerSize::Medium).show(ui, &theme, |ui| {
                                Card::new().show(ui, &theme, |ui| {
                                    VStack::new(theme.spacing.md).show(ui, |ui| {
                                        ui.strong("User Directory");

                                        Table::new().striped(true).id_source("user_table").show(
                                            ui,
                                            &theme,
                                            |table| {
                                                table.headers(&["Name", "Email", "Role", "Status"]);

                                                table.row(|row| {
                                                    row.col_strong("Alice Johnson");
                                                    row.col_label("alice@example.com");
                                                    row.col(|ui| {
                                                        Badge::new("Admin")
                                                            .color(BadgeColor::Success)
                                                            .show(ui);
                                                    });
                                                    row.col(|ui| {
                                                        Badge::new("Active")
                                                            .variant(BadgeVariant::Filled)
                                                            .color(BadgeColor::Info)
                                                            .show(ui);
                                                    });
                                                });

                                                table.row(|row| {
                                                    row.col_label("Bob Smith");
                                                    row.col_label("bob@example.com");
                                                    row.col(|ui| {
                                                        Badge::new("User")
                                                            .color(BadgeColor::Info)
                                                            .show(ui);
                                                    });
                                                    row.col(|ui| {
                                                        Badge::new("Active")
                                                            .variant(BadgeVariant::Filled)
                                                            .color(BadgeColor::Info)
                                                            .show(ui);
                                                    });
                                                });

                                                table.row(|row| {
                                                    row.col_label("Carol White");
                                                    row.col_label("carol@example.com");
                                                    row.col(|ui| {
                                                        Badge::new("Moderator")
                                                            .color(BadgeColor::Warning)
                                                            .show(ui);
                                                    });
                                                    row.col(|ui| {
                                                        Badge::new("Away")
                                                            .color(BadgeColor::Warning)
                                                            .show(ui);
                                                    });
                                                });

                                                table.row(|row| {
                                                    row.col_label("David Brown");
                                                    row.col_label("david@example.com");
                                                    row.col(|ui| {
                                                        Badge::new("User")
                                                            .color(BadgeColor::Info)
                                                            .show(ui);
                                                    });
                                                    row.col(|ui| {
                                                        Badge::new("Offline")
                                                            .color(BadgeColor::Error)
                                                            .show(ui);
                                                    });
                                                });
                                            },
                                        );
                                    });
                                });
                            });
                        });

                        // Container Demo
                        demo_section(ui, &theme, "Container", |ui| {
                            VStack::new(theme.spacing.md).show(ui, |ui| {
                                for (size, label) in [
                                    (ContainerSize::Small, "Small (600px)"),
                                    (ContainerSize::Medium, "Medium (960px)"),
                                ] {
                                    Container::new(size).show(ui, &theme, |ui| {
                                        Card::new().show(ui, &theme, |ui| {
                                            ui.label(label);
                                        });
                                    });
                                }
                            });
                        });

                        // ZStack Demo
                        demo_section(ui, &theme, "ZStack (Layering)", |ui| {
                            Container::new(ContainerSize::Small).show(ui, &theme, |ui| {
                                Card::new().width(400.0).show(ui, &theme, |ui| {
                                    ZStack::new().width(400.0).height(200.0).show(ui, |z| {
                                        // Background layer
                                        z.layer(|ui| {
                                            ui.vertical_centered(|ui| {
                                                ui.add_space(60.0);
                                                ui.heading("Background Layer");
                                            });
                                        });

                                        // Foreground badge
                                        z.layer(|ui| {
                                            ui.add_space(10.0);
                                            ui.horizontal(|ui| {
                                                ui.add_space(10.0);
                                                Badge::new("New")
                                                    .variant(BadgeVariant::Filled)
                                                    .color(BadgeColor::Error)
                                                    .show(ui);
                                            });
                                        });
                                    });
                                });
                            });
                        });

                        // AspectRatio Demo
                        demo_section(ui, &theme, "AspectRatio", |ui| {
                            Container::new(ContainerSize::Medium).show(ui, &theme, |ui| {
                                HStack::new(theme.spacing.lg).show(ui, |ui| {
                                    for (ratio, label) in [
                                        (AspectRatio::square(), "Square (1:1)"),
                                        (AspectRatio::widescreen(), "Widescreen (16:9)"),
                                        (AspectRatio::standard(), "Standard (4:3)"),
                                    ] {
                                        ui.vertical(|ui| {
                                            ui.set_width(200.0);
                                            ui.label(label);
                                            ui.add_space(theme.spacing.sm);
                                            ratio.content_mode(ContentMode::Fit).show(ui, |ui| {
                                                // Fill the allocated space with a visible background
                                                let rect = ui.max_rect();
                                                ui.painter().rect_filled(
                                                    rect,
                                                    4.0,
                                                    egui::Color32::from_rgb(50, 50, 70),
                                                );
                                                ui.centered_and_justified(|ui| {
                                                    ui.label(format!(
                                                        "{}x{}",
                                                        rect.width() as i32,
                                                        rect.height() as i32
                                                    ));
                                                });
                                            });
                                        });
                                    }
                                });
                            });
                        });

                        // Complex Layout Example
                        demo_section(ui, &theme, "Complex Layout Example", |ui| {
                            Container::new(ContainerSize::Medium).show(ui, &theme, |ui| {
                                Card::new().show(ui, &theme, |ui| {
                                    VStack::new(theme.spacing.lg).show(ui, |ui| {
                                        // Header
                                        HStack::new(theme.spacing.md).show(ui, |ui| {
                                            ui.heading("Profile Card");
                                            Spacer::new().show(ui);
                                            Badge::new("Pro")
                                                .color(BadgeColor::Success)
                                                .show(ui);
                                        });

                                        Divider::horizontal().show(ui);

                                        // Content Grid
                                        Grid::new(2)
                                            .gap(theme.spacing.md)
                                            .id_source("profile_grid")
                                            .show(ui, |grid| {
                                                grid.cell(|ui| {
                                                    VStack::new(theme.spacing.xs).show(ui, |ui| {
                                                        ui.label("Name:");
                                                        ui.strong("John Doe");
                                                    });
                                                });
                                                grid.cell(|ui| {
                                                    VStack::new(theme.spacing.xs).show(ui, |ui| {
                                                        ui.label("Email:");
                                                        ui.strong("john@example.com");
                                                    });
                                                });
                                            });

                                        Divider::horizontal().show(ui);

                                        // Footer Buttons
                                        HStack::new(theme.spacing.sm).show(ui, |ui| {
                                            Button::new("Edit")
                                                .variant(ButtonVariant::Filled)
                                                .show(ui);
                                            Button::new("Share")
                                                .variant(ButtonVariant::Outlined)
                                                .show(ui);
                                            Spacer::new().show(ui);
                                            Button::new("Delete")
                                                .variant(ButtonVariant::Text)
                                                .show(ui);
                                        });
                                    });
                                });
                            });
                        });

                        Spacer::xxl(&theme).show(ui);
                    });
                });
            });
        });
    })
}

fn demo_section(
    ui: &mut egui::Ui,
    theme: &Theme,
    title: &str,
    content: impl FnOnce(&mut egui::Ui),
) {
    VStack::new(theme.spacing.md).show(ui, |ui| {
        HStack::new(theme.spacing.sm).show(ui, |ui| {
            ui.heading(title);
            Badge::new("Layout").color(BadgeColor::Info).show(ui);
        });
        content(ui);
    });
}
