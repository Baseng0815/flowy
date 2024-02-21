use eframe::egui;

use simulator::grid::StaggeredMACGrid;

use crate::{visualize::FlowyApp, simulator::{simulator::Simulator, math::vector2}};

mod simulator;
mod visualize;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "flowy",
        options,
        Box::new(|cc| {
            let mut grid = StaggeredMACGrid::new(20);
            *grid.temp_grid_mut(2, 5) = 10.0;

            let tcc = grid.velocities_x.len();

            for (i, vx) in grid.velocities_x.iter_mut().enumerate() {
                *vx = i as f64 / tcc as f64;
            }

            for (i, vy) in grid.velocities_y.iter_mut().enumerate() {
                *vy = i as f64 / tcc as f64;
            }

            let simulator = Simulator::new(grid);
            let app = FlowyApp::new(simulator);

            Box::new(app)
        }),
    )
}
