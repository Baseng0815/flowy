use std::fmt::Display;

pub struct StaggeredMACGrid {
    pub cell_size: f64,
    pub velocities_x: Vec<f64>
}

impl StaggeredMACGrid {
    pub fn new(cell_size: f64, cell_count: u32) -> StaggeredMACGrid {
        StaggeredMACGrid {
            cell_size,
            velocities_x: vec![0.0; cell_count as usize + 1]
        }
    }

    pub fn draw() {
        unimplemented!();
    }

    pub fn get_velocity_at(x: u32, y: u32) {
        unimplemented!()
    }
}

impl Display for StaggeredMACGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.velocities_x.len() {
            for x in 0..self.velocities_x.len() {

            }
        }
    }
}
