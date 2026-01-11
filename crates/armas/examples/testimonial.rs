use armas::ext::ArmasContextExt;
use armas::layout::{Container, ContainerSize, VStack};
use armas::{TestimonialGrid, TestimonialItem, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Testimonial Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "Testimonial Demo",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(TestimonialApp::new()))
        }),
    )
}

struct TestimonialApp {
}

impl TestimonialApp {
    fn new() -> Self {
        Self {
        }
    }
}

impl eframe::App for TestimonialApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let theme = ctx.armas_theme();
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                Container::new(ContainerSize::Large)
                    .padding(32.0)
                    .show(ui, &theme, |ui| {
                        VStack::new(20.0).show(ui, |ui| {
                            ui.heading("Testimonials");

                            let items = vec![
                    TestimonialItem::new(
                        "This component library is amazing! It saved us weeks of development time.",
                        "Sarah Johnson",
                        "CTO, TechCorp"
                    )
                    .avatar("SJ")
                    .avatar_color(egui::Color32::from_rgb(59, 130, 246))
                    .rating(5),

                    TestimonialItem::new(
                        "Clean API, beautiful components, and excellent documentation. Highly recommended!",
                        "Michael Chen",
                        "Lead Developer, StartupXYZ"
                    )
                    .avatar("MC")
                    .avatar_color(egui::Color32::from_rgb(34, 197, 94))
                    .rating(5),

                    TestimonialItem::new(
                        "The animation system is incredibly smooth and the theming is perfect.",
                        "Emily Rodriguez",
                        "UI Designer, DesignStudio"
                    )
                    .avatar("ER")
                    .avatar_color(egui::Color32::from_rgb(168, 85, 247))
                    .rating(4),

                    TestimonialItem::new(
                        "Best egui component library I've used. The gradient cards are stunning!",
                        "David Kim",
                        "Indie Developer"
                    )
                    .avatar("DK")
                    .avatar_color(egui::Color32::from_rgb(251, 191, 36))
                    .rating(5),

                    TestimonialItem::new(
                        "Great performance and zero bloat. Exactly what we needed for our project.",
                        "Lisa Wang",
                        "Senior Engineer, BigTech"
                    )
                    .avatar("LW")
                    .avatar_color(egui::Color32::from_rgb(236, 72, 153))
                    .rating(5),

                    TestimonialItem::new(
                        "The builder pattern API makes it a joy to work with. Very intuitive!",
                        "James Brown",
                        "Full Stack Developer"
                    )
                    .avatar("JB")
                    .avatar_color(egui::Color32::from_rgb(239, 68, 68))
                    .rating(4),
                            ];

                            TestimonialGrid::new(items)
                                .columns(3)
                                .gap(20.0)
                                .show(ui);
                        });
                    });
            });
        });
    }
}
