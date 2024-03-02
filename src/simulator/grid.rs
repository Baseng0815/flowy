use std::fmt::Display;

use super::{math::{vector2, Vector2}, interpolation::{Interpolation, CubicInterpolation, LinearInterpolation}};

#[derive(Clone, PartialEq)]
pub struct StaggeredMACGrid {
    pub cell_count: i32,
    pub velocities_x: Vec<f64>,
    pub velocities_y: Vec<f64>,

    // for now only a mock quantity for advection tests
    pub temperature: Vec<f64>,
}

impl StaggeredMACGrid {
    pub fn new(cell_count: i32) -> Self {
        let cc2 = cell_count + 2;

        Self {
            cell_count,
            velocities_x: vec![0.0; (cc2 * (cc2 + 1)) as usize],
            velocities_y: vec![0.0; (cc2 * (cc2 + 1)) as usize],
            temperature: vec![0.0; (cc2 * cc2) as usize]
        }
    }

    // accessors for sampled grid values
    pub fn vel_x_grid(&self, x: i32, y: i32) -> f64 {
        self.velocities_x[((x + 1) + (y + 1) * (self.cell_count + 3)) as usize]
    }

    pub fn vel_x_grid_mut(&mut self, x: i32, y: i32) -> &mut f64 {
        &mut self.velocities_x[((x + 1) + (y + 1) * (self.cell_count + 3)) as usize]
    }

    pub fn vel_y_grid(&self, x: i32, y: i32) -> f64 {
        self.velocities_y[((y + 1) + (x + 1) * (self.cell_count + 3)) as usize]
    }

    pub fn vel_y_grid_mut(&mut self, x: i32, y: i32) -> &mut f64 {
        &mut self.velocities_y[((y + 1) + (x + 1) * (self.cell_count + 3)) as usize]
    }

    pub fn temp_grid(&self, x: i32, y: i32) -> f64 {
        self.temperature[((x + 1) + (y + 1) * (self.cell_count + 2)) as usize]
    }

    pub fn temp_grid_mut(&mut self, x: i32, y: i32) -> &mut f64 {
        &mut self.temperature[((x + 1) + (y + 1) * (self.cell_count + 2)) as usize]
    }

    // interpolated values (readonly)
    pub fn temp(&self, pos: Vector2) -> f64 {
        let cc2 = self.cell_count as usize + 2;

        let start = (pos.y + 0.5) as usize * cc2;
        // TODO proper boundary condition
        let zero = vec![0f64; cc2];
        let row_above = self.temperature.get(start..(start + cc2)).unwrap_or(&zero);
        let row_below = self.temperature.get((start + cc2)..(start + 2 * cc2)).unwrap_or(&zero);

        // TODO use cubic interpolation
        let value_above = LinearInterpolation::interpolate(row_above, pos.x + 0.5);
        let value_below = LinearInterpolation::interpolate(row_below, pos.x + 0.5);

        LinearInterpolation::interpolate(&[value_above, value_below], (pos.y + 0.5).fract().abs())
    }

    pub fn temp_average(&self) -> f64 {
        self.temperature.iter().sum::<f64>() / self.temperature.len() as f64
    }

    pub fn vel(&self, pos: Vector2) -> Vector2 {
        let cc3 = self.cell_count as usize + 3;

        // TODO make generic
        // TODO proper boundary condition
        let zero = vec![0f64; cc3];
        let iy = cc3 * (pos.y + 1.0) as usize;
        let slice_x = &self.velocities_x.get(iy..iy + cc3 as usize).unwrap_or(&zero);

        let ix = cc3 * (pos.x + 1.0) as usize;
        let slice_y = &self.velocities_y.get(ix..ix + cc3 as usize).unwrap_or(&zero);

        // TODO use cubic interpolation
        let vx = LinearInterpolation::interpolate(slice_x, pos.x + 1.0);
        let vy = LinearInterpolation::interpolate(slice_y, pos.y + 1.0);
        vector2(vx, vy)
    }
}

impl Display for StaggeredMACGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.velocities_x.len() {
            for x in 0..self.velocities_x.len() {
                let vy = self.velocities_y[y];
                let vx = self.velocities_x[x];
                let l = (vx * vx + vy * vy).sqrt();
                print!("{l:.2} ");
            }

            println!("");
        }

        Ok(())
    }
}
