use std::fmt::Display;

use super::{math::{vector2, Vector2}, interpolation::{Interpolation, CubicInterpolation, LinearInterpolation}};

#[derive(Clone)]
pub struct StaggeredMACGrid {
    pub cell_count: u32,
    pub velocities_x: Vec<f64>,
    pub velocities_y: Vec<f64>,

    pub temperature: Vec<f64>,
}

impl StaggeredMACGrid {
    pub fn new(grid_size: f64, cell_count: u32) -> Self {
        Self {
            cell_count,
            velocities_x: vec![0.0; (cell_count * (cell_count + 1)) as usize],
            velocities_y: vec![0.0; (cell_count * (cell_count + 1)) as usize],
            temperature: vec![0.0; (cell_count * cell_count) as usize]
        }
    }

    pub fn get_vel_x(&self, x: u32, y: u32) -> f64 {
        self.velocities_x[(x + y * (self.cell_count + 1)) as usize]
    }

    pub fn set_vel_x(&mut self, x: u32, y: u32, v: f64) {
        self.velocities_x[(x + y * (self.cell_count + 1)) as usize] = v;
    }

    pub fn get_vel_y(&self, x: u32, y: u32) -> f64 {
        self.velocities_y[(y + x * (self.cell_count + 1)) as usize]
    }

    pub fn set_vel_y(&mut self, x: u32, y: u32, v: f64) {
        self.velocities_y[(y + x * (self.cell_count + 1)) as usize] = v;
    }

    pub fn temperature_center(&self, x: u32, y: u32) -> f64 {
        self.temperature[(x + y * (self.cell_count)) as usize]
    }

    pub fn temperature_center_mut(&mut self, x: u32, y: u32) -> &mut f64 {
        &mut self.temperature[(x + y * (self.cell_count)) as usize]
    }

    pub fn temperature(&self, pos: Vector2) -> f64 {
        // TODO use cubic interpolation
        todo!()
    }

    pub fn vel(&self, pos: Vector2) -> Vector2 {
        // TODO make generic
        let iy = (self.cell_count + 1) as usize * (pos.y as usize).clamp(0, (self.cell_count - 1) as usize);
        let slice_x = &self.velocities_x[iy..iy + (self.cell_count + 1) as usize];

        let ix = (self.cell_count + 1) as usize * (pos.x as usize).clamp(0, (self.cell_count - 1) as usize);
        let slice_y = &self.velocities_y[ix..ix + (self.cell_count + 1) as usize];

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
