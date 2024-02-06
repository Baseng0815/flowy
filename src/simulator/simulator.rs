use super::{grid::StaggeredMACGrid, interpolation::{Interpolation, CubicInterpolation}, math::{Vector2, vector2}};


pub struct Simulator
{
    pub grid: StaggeredMACGrid
}

impl Simulator {
    pub fn new(grid: StaggeredMACGrid) -> Self {
        Self {
            grid
        }
    }

    pub fn advect(&mut self, dt: f64) {
        let mut grid_new = self.grid.clone();
        let half_grid = 1.0 / (self.grid.cell_count as f64 * 2.0);

        // advect velocities
        for row in 0..self.grid.cell_count {
            for col in 0..=self.grid.cell_count {
                // x velocities
                let xp = vector2(col as f64, row as f64 + half_grid);
                let xg = self.trace_back(dt, xp);

                let clamped = xg.clamp(0.0, self.grid.cell_count as f64);
                let v_new = self.grid.vel(clamped);
                grid_new.set_vel_x(col, row, v_new.x);

                // y velocities
                let xp = vector2(row as f64 + half_grid, col as f64);
                let xg = self.trace_back(dt, xp);

                let clamped = xg.clamp(0.0, self.grid.cell_count as f64);
                let v_new = self.grid.vel(clamped);
                grid_new.set_vel_y(row, col, v_new.y);
            }
        }

        self.grid = grid_new;
    }

    fn trace_back(&self, dt: f64, pos: Vector2) -> Vector2 {
        // forward Euler (TODO replace with second-order Runge-Kutta/make generic)
        let u = self.grid.vel(pos);

        pos - dt * u
    }
}
