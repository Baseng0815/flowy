use eframe::egui;

use simulator::grid::StaggeredMACGrid;

use crate::{visualize::FlowyApp, simulator::{simulator::Simulator, math::vector2}};

mod simulator;
mod visualize;
mod tests;

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

            for (i, vx) in grid.velocities_x.iter_mut(). enumerate() {
                *vx = i as f64 / tcc as f64;
            }

            for (i, vy) in grid.velocities_y.iter_mut().enumerate() {
                *vy = i as f64 / tcc as f64;
            }

            for col in -1..grid.cell_count + 2 {
                *grid.vel_x_grid_mut(col as i32, -1) = 0.0;
                *grid.vel_x_grid_mut(col as i32, grid.cell_count) = 0.0;
                *grid.vel_y_grid_mut(-1, col as i32) = 0.0;
                *grid.vel_y_grid_mut(grid.cell_count, col as i32) = 0.0;
            }

            for row in -1..grid.cell_count + 1 {
                *grid.vel_x_grid_mut(-1, row as i32) = 0.0;
                *grid.vel_x_grid_mut(grid.cell_count + 1, row as i32) = 0.0;
                *grid.vel_y_grid_mut(row as i32, -1) = 0.0;
                *grid.vel_y_grid_mut(row as i32, grid.cell_count + 1) = 0.0;
            }

            *grid.vel_y_grid_mut(19, 0) = 10.0;

            let simulator = Simulator::new(grid);
            let app = FlowyApp::new(simulator);

            Box::new(app)
        }),
    )
}
