use std::fmt::Display;

use super::{math::{vector2, Vector2}, interpolation::{Interpolation, CubicInterpolation, LinearInterpolation}};

#[derive(Clone, PartialEq)]
pub struct StaggeredMACGrid {
    pub cell_count: u32,
    pub velocities_x: Vec<f64>,
    pub velocities_y: Vec<f64>,

    // for now only a mock quantity for advection tests
    pub temperature: Vec<f64>,
}

impl StaggeredMACGrid {
    pub fn new(cell_count: u32) -> Self {
        Self {
            cell_count,
            velocities_x: vec![0.0; (cell_count * (cell_count + 1)) as usize],
            velocities_y: vec![0.0; (cell_count * (cell_count + 1)) as usize],
            temperature: vec![0.0; (cell_count * cell_count) as usize]
        }
    }

    // accessors for sampled grid values
    pub fn vel_x_grid(&self, x: u32, y: u32) -> f64 {
        self.velocities_x[(x + y * (self.cell_count + 1)) as usize]
    }

    pub fn vel_x_grid_mut(&mut self, x: u32, y: u32) -> &mut f64 {
        &mut self.velocities_x[(x + y * (self.cell_count + 1)) as usize]
    }

    pub fn vel_y_grid(&self, x: u32, y: u32) -> f64 {
        self.velocities_y[(y + x * (self.cell_count + 1)) as usize]
    }

    pub fn vel_y_grid_mut(&mut self, x: u32, y: u32) -> &mut f64 {
        &mut self.velocities_y[(y + x * (self.cell_count + 1)) as usize]
    }

    pub fn temp_grid(&self, x: u32, y: u32) -> f64 {
        self.temperature[(x + y * (self.cell_count)) as usize]
    }

    pub fn temp_grid_mut(&mut self, x: u32, y: u32) -> &mut f64 {
        &mut self.temperature[(x + y * (self.cell_count)) as usize]
    }

    // interpolated values (readonly)
    pub fn temp(&self, pos: Vector2) -> f64 {
        // TODO use cubic interpolation
        let cc = self.cell_count as usize;

        let start = (pos.y - 0.5) as usize * cc;
        // TODO proper boundary condition
        let zero = vec![0f64; cc];
        let row_above = self.temperature.get(start..(start + cc)).unwrap_or(&zero);
        let row_below = self.temperature.get((start + cc)..(start + 2 * cc)).unwrap_or(&zero);

        let value_above = LinearInterpolation::interpolate(row_above, pos.x - 0.5);
        let value_below = LinearInterpolation::interpolate(row_below, pos.x - 0.5);

        LinearInterpolation::interpolate(&[value_above, value_below], (pos.y - 0.5).fract().abs())
    }

    pub fn vel(&self, pos: Vector2) -> Vector2 {
        let cc1 = self.cell_count as usize + 1;

        // TODO make generic
        // TODO proper boundary condition
        let zero = vec![0f64; cc1];
        let iy = cc1 * (pos.y as usize);
        let slice_x = &self.velocities_x.get(iy..iy + cc1 as usize).unwrap_or(&zero);

        let ix = cc1 as usize * (pos.x as usize);
        let slice_y = &self.velocities_y.get(ix..ix + cc1 as usize).unwrap_or(&zero);

        let vx = CubicInterpolation::interpolate(slice_x, pos.x);
        let vy = CubicInterpolation::interpolate(slice_y, pos.y);
        vector2(vx, vy)
    }
}

impl Display for StaggeredMACGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.velocities_x.len() {
            for x in 0..self.velocities_x.len() {
                let vy = self.velocities_y[y];
                let vx = self.velocities_x[y];
                let l = (vx * vx + vy * vy).sqrt();
                print!("{l:.2} ");
            }

            println!("");
        }

        Ok(())
    }
}
