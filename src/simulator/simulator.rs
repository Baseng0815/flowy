use super::{grid::StaggeredMACGrid, interpolation::{Interpolation, CubicInterpolation}, math::Vector2};


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

    }

    fn trace_back(&self, dt: f64, pos: Vector2) -> Vector2 {
        let u = self.grid.vel(pos);

        pos - dt * u
    }
}
