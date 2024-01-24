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
            velocities_x: vec![0.0; cell_count as usize + 1],
            velocities_y: vec![0.0; cell_count as usize + 1]
        }
    }

    // TODO? remove checks
    pub fn velocity_gradient(&self, x: u32, y: u32) -> Option<f64> {
        let vxl = self.velocities_x.get(x as usize)?;
        let vxr = self.velocities_x.get((x + 1) as usize)?;

        let vyu = self.velocities_y.get(y as usize)?;
        let vyd = self.velocities_y.get((y + 1) as usize)?;

        eprintln!("vxl = {:#?}", vxl);
        eprintln!("vxr = {:#?}", vxr);

        let gradient = vector2(vxl - vxr, vyu - vyd).len();
        Some(gradient)
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
