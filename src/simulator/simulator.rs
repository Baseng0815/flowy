use super::grid::StaggeredMACGrid;

pub struct Simulator {
    pub grid: StaggeredMACGrid
}

impl Simulator {
    pub fn new(grid: StaggeredMACGrid) -> Self {
        Self {
            grid
        }
    }
}
