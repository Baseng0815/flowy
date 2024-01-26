use std::fmt::Display;

use super::math::vector2;

pub struct StaggeredMACGrid {
    pub cell_size: f64,
    pub cell_count: u32,
    pub velocities_x: Vec<f64>,
    pub velocities_y: Vec<f64>
}

impl StaggeredMACGrid {
    pub fn new(cell_size: f64, cell_count: u32) -> StaggeredMACGrid {
        StaggeredMACGrid {
            cell_size,
            cell_count,
            velocities_x: vec![0.0; (cell_count * (cell_count + 1)) as usize],
            velocities_y: vec![0.0; (cell_count * (cell_count + 1)) as usize]
        }
    }

    pub fn vel_x(&self, x: u32, y: u32) -> f64 {
        self.velocities_x[(x + y * (self.cell_count + 1)) as usize]
    }

    pub fn vel_x_mut(&mut self, x: u32, y: u32) -> &mut f64 {
        &mut self.velocities_x[(x + y * (self.cell_count + 1)) as usize]
    }

    pub fn vel_y(&self, x: u32, y: u32) -> f64 {
        self.velocities_y[(y + x * (self.cell_count + 1)) as usize]
    }

    pub fn vel_y_mut(&mut self, x: u32, y: u32) -> &mut f64 {
        &mut self.velocities_y[(y + x * (self.cell_count + 1)) as usize]
    }

    // TODO? remove checks
    pub fn velocity_gradient(&self, x: u32, y: u32) {
        let vxl = self.velocities_x[x as usize];
        let vxr = self.velocities_x[(x + 1) as usize];

        let vyu = self.velocities_y[y as usize];
        let vyd = self.velocities_y[(y + 1) as usize];

        vector2(vxl - vxr, vyu - vyd).len();
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
