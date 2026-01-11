use armas::ext::ArmasContextExt;
use armas::{Pagination, Theme};
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Pagination Component Example",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_armas_theme(Theme::dark());
            Ok(Box::new(PaginationExample::default()))
        }),
    )
}

struct PaginationExample {
    current_page_basic: usize,
    current_page_many: usize,
    current_page_compact: usize,
    current_page_simple: usize,
    items_per_page: usize,
    total_items: usize,
}

impl Default for PaginationExample {
    fn default() -> Self {
        Self {
            current_page_basic: 1,
            current_page_many: 15,
            current_page_compact: 1,
            current_page_simple: 1,
            items_per_page: 10,
            total_items: 245,
        }
    }
}

impl PaginationExample {
    fn total_pages(&self) -> usize {
        (self.total_items + self.items_per_page - 1) / self.items_per_page
    }
}

impl eframe::App for PaginationExample {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Pagination Component Examples");
            ui.add_space(20.0);

            // Basic pagination
            ui.label("Basic Pagination (10 pages):");
            ui.add_space(10.0);

            let response = Pagination::new(self.current_page_basic, 10).show(ui);
            if let Some(page) = response.page_changed {
                self.current_page_basic = page;
            }

            ui.add_space(10.0);
            ui.label(format!("Current page: {}/10", self.current_page_basic));
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Many pages with ellipsis
            ui.label("Many Pages (50 pages with ellipsis):");
            ui.add_space(10.0);

            let response = Pagination::new(self.current_page_many, 50)
                .max_visible_pages(7)
                .show(ui);
            if let Some(page) = response.page_changed {
                self.current_page_many = page;
            }

            ui.add_space(10.0);
            ui.label(format!("Current page: {}/50", self.current_page_many));
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Compact pagination
            ui.label("Compact (without first/last buttons):");
            ui.add_space(10.0);

            let response = Pagination::new(self.current_page_compact, 10)
                .show_first_last(false)
                .show(ui);
            if let Some(page) = response.page_changed {
                self.current_page_compact = page;
            }

            ui.add_space(10.0);
            ui.label(format!("Current page: {}/10", self.current_page_compact));
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Simple pagination
            ui.label("Simple (only prev/next):");
            ui.add_space(10.0);

            let response = Pagination::new(self.current_page_simple, 10)
                .show_first_last(false)
                .max_visible_pages(1)
                .show(ui);
            if let Some(page) = response.page_changed {
                self.current_page_simple = page;
            }

            ui.add_space(10.0);
            ui.label(format!("Current page: {}/10", self.current_page_simple));
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Dynamic pagination based on total items
            ui.label("Dynamic Pagination (based on items count):");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Total items:");
                ui.add(egui::Slider::new(&mut self.total_items, 1..=500));
            });

            ui.horizontal(|ui| {
                ui.label("Items per page:");
                ui.add(egui::Slider::new(&mut self.items_per_page, 5..=50));
            });

            ui.add_space(10.0);

            let total_pages = self.total_pages();
            let mut current_page_dynamic = self.current_page_basic.min(total_pages);

            let response = Pagination::new(current_page_dynamic, total_pages)
                .max_visible_pages(9)
                .show(ui);

            if let Some(page) = response.page_changed {
                current_page_dynamic = page;
                self.current_page_basic = page;
            }

            ui.add_space(10.0);
            ui.label(format!(
                "Showing items {}-{} of {} (Page {}/{})",
                (current_page_dynamic - 1) * self.items_per_page + 1,
                (current_page_dynamic * self.items_per_page).min(self.total_items),
                self.total_items,
                current_page_dynamic,
                total_pages
            ));

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // Custom spacing
            ui.label("Custom Spacing:");
            ui.add_space(10.0);

            let response = Pagination::new(1, 5).spacing(12.0).show(ui);

            ui.add_space(20.0);

            ui.label("💡 Tip: Click page numbers to navigate, use arrow buttons for prev/next");
        });
    }
}
