use armas::layout::Grid;

fn main() -> eframe::Result<()> {
    eframe::run_simple_native("Grid Test", eframe::NativeOptions::default(), |ctx, _| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Grid Test - Simple Labels");

            Grid::new(3).gap(8.0).show(ui, |grid| {
                grid.cell(|ui| ui.label("Cell 1"));
                grid.cell(|ui| ui.label("Cell 2"));
                grid.cell(|ui| ui.label("Cell 3"));
                grid.cell(|ui| ui.label("Cell 4"));
                grid.cell(|ui| ui.label("Cell 5"));
                grid.cell(|ui| ui.label("Cell 6"));
            });
        });
    })
}
