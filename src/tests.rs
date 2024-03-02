use crate::simulator::{grid::StaggeredMACGrid, interpolation::{LinearInterpolation, Interpolation, CubicInterpolation}};

#[test]
fn grid_vel_x() {
    let cc = 20;

    let mut grid = StaggeredMACGrid::new(cc);
    for x in 0..cc + 1 {
        *grid.vel_x_grid_mut(x, 0) = x as f64 * 2.0;
    }

    // first row zero
    for vx in grid.velocities_x.iter().take((cc + 3) as usize) {
        assert!(*vx == 0.0);
    }

    for x in 0..cc + 1 {
        assert!(grid.vel_x_grid(x, -1) == 0.0);
    }

    // second row initialized
    for (i, vx) in grid.velocities_x.iter().skip((cc + 3) as usize).take((cc + 3) as usize).enumerate() {
        if i == 0 || i == (cc + 2) as usize {
            assert!(*vx == 0.0)
        } else {
            assert!(*vx == (i - 1) as f64 * 2.0);
        }
    }

    for x in 0..cc + 1 {
        assert!(grid.vel_x_grid(x, 0) == x as f64 * 2.0);
    }
}

#[test]
fn grid_vel_y() {
    let cc = 20;

    let mut grid = StaggeredMACGrid::new(cc);
    for y in 0..cc + 1 {
        *grid.vel_y_grid_mut(0, y) = y as f64 * 2.0;
    }

    // first column zero
    for vy in grid.velocities_y.iter().take((cc + 3) as usize) {
        assert!(*vy == 0.0);
    }

    for y in 0..cc + 1 {
        assert!(grid.vel_y_grid(-1, y) == 0.0);
    }

    // second column initialized
    for (i, vy) in grid.velocities_y.iter().skip((cc + 3) as usize).take((cc + 3) as usize).enumerate() {
        if i == 0 || i == (cc + 2) as usize {
            assert!(*vy == 0.0)
        } else {
            assert!(*vy == (i - 1) as f64 * 2.0);
        }
    }

    for y in 0..cc + 1 {
        assert!(grid.vel_y_grid(0, y) == y as f64 * 2.0);
    }
}

#[test]
fn linear_interpolate() {
    let values: Vec<f64> = (0..10).step_by(2).map(|x| (x * x) as f64).collect();
    assert!(LinearInterpolation::interpolate(&values, 0.0) == 0.0);
    assert!(LinearInterpolation::interpolate(&values, 1.0) == 4.0);
    assert!(LinearInterpolation::interpolate(&values, 2.0) == 16.0);
    assert!(LinearInterpolation::interpolate(&values, 3.0) == 36.0);

    assert!(LinearInterpolation::interpolate(&values, 0.5) == 2.0);
    assert!(LinearInterpolation::interpolate(&values, 0.8) >= 2.0);
    assert!(LinearInterpolation::interpolate(&values, 0.8) <= 4.0);

    assert!(LinearInterpolation::interpolate(&values, 1.5) == 10.0);
    assert!(LinearInterpolation::interpolate(&values, 1.3) >= 4.0);
    assert!(LinearInterpolation::interpolate(&values, 1.3) <= 16.0);
}


#[test]
fn cubic_interpolate() {
    let values: Vec<f64> = (0..10).step_by(2).map(|x| (x * x) as f64).collect();
    assert!(CubicInterpolation::interpolate(&values, 0.0) == 0.0);
    assert!(CubicInterpolation::interpolate(&values, 1.0) == 4.0);
    assert!(CubicInterpolation::interpolate(&values, 2.0) == 16.0);
    assert!(CubicInterpolation::interpolate(&values, 3.0) == 36.0);

    assert!(CubicInterpolation::interpolate(&values, 0.8) >= 2.0);
    assert!(CubicInterpolation::interpolate(&values, 0.8) <= 4.0);

    assert!(CubicInterpolation::interpolate(&values, 1.3) >= 4.0);
    assert!(CubicInterpolation::interpolate(&values, 1.3) <= 16.0);
}
