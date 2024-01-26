use super::{grid::StaggeredMACGrid, interpolation::{Interpolation, CubicInterpolation}};

pub trait Simulator {
    fn new(grid: StaggeredMACGrid) -> Self;
    type InterpolationT: Interpolation;
}

pub struct StandardSimulator
{
    pub grid: StaggeredMACGrid
}

impl Simulator for StandardSimulator {
    type InterpolationT = CubicInterpolation;

    fn new(grid: StaggeredMACGrid) -> Self {
        Self {
            grid
        }
    }
}
