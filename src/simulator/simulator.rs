use chrono::{NaiveTime, Local};

use super::{grid::StaggeredMACGrid, math::{Vector2, vector2}};


pub struct Simulator
{
    pub grid: StaggeredMACGrid,
    pub current_time_step: u32,
    pub last_stepped: NaiveTime
}

impl Simulator {
    pub fn new(grid: StaggeredMACGrid) -> Self {
        Self {
            grid,
            current_time_step: 0,
            last_stepped: Local::now().time()
        }
    }

    pub fn advect(&mut self, dt: f64) {
        let mut grid_new = self.grid.clone();

        // advect velocities
        for row in 0..self.grid.cell_count {
            for col in 0..=self.grid.cell_count {
                // x velocities
                let xp = vector2(col as f64, row as f64 + 0.5);
                let xg = self.trace_back(dt, xp);

                let clamped = xg.clamp(0.0, self.grid.cell_count as f64);
                let v_new = self.grid.vel(clamped);
                *grid_new.vel_x_grid_mut(col, row) = v_new.x;

                // y velocities
                let xp = vector2(row as f64 + 0.5, col as f64);
                let xg = self.trace_back(dt, xp);

                let clamped = xg.clamp(0.0, self.grid.cell_count as f64);
                let v_new = self.grid.vel(clamped);
                *grid_new.vel_y_grid_mut(row, col) = v_new.y;
            }
        }

        // advect temperature
        for y in 0..self.grid.cell_count {
            for x in 0..self.grid.cell_count {
                let xp = vector2(x as f64 + 0.5, y as f64 + 0.5);
                let xg = self.trace_back(dt, xp);

                let temp_new = self.grid.temp(xg);
                *grid_new.temp_grid_mut(x, y) = temp_new;
            }
        }

        self.grid = grid_new;

        self.last_stepped = Local::now().time();
        self.current_time_step += 1;
    }

    fn trace_back(&self, dt: f64, pos: Vector2) -> Vector2 {
        // forward Euler (TODO replace with second-order Runge-Kutta, make generic)
        let u = self.grid.vel(pos);

        pos - dt * u
    }
}
