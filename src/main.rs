use eframe::egui;

use simulator::grid::StaggeredMACGrid;

use crate::{visualize::FlowyApp, simulator::simulator::Simulator};

mod simulator;
mod visualize;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "flowy",
        options,
        Box::new(|cc| {
            let mut grid = StaggeredMACGrid::new(1.0, 64);
            for (i, vx) in grid.velocities_x.iter_mut().enumerate() {
                *vx = i as f64 / grid.cell_count as f64;
            }

            for (i, vy) in grid.velocities_y.iter_mut().enumerate() {
                *vy = i as f64 / grid.cell_count as f64;
            }

            let simulator = Simulator::new(grid);
            let app = FlowyApp::new(simulator);

            Box::new(app)
        }),
    )
}