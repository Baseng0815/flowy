pub trait Interpolation {
    fn interpolate(points: &[f64], index: f64) -> f64;
}

pub struct LinearInterpolation {
}

impl Interpolation for LinearInterpolation {
    fn interpolate(points: &[f64], index: f64) -> f64 {
        let c0 = index.floor() as usize;
        let c1 = (c0 + 1).min(points.len() - 1);

        let s = index - c0 as f64;

        (1.0 - s) * points[c0] + s * points[c1]
    }
}

pub struct CubicInterpolation {
}

impl Interpolation for CubicInterpolation {
    fn interpolate(points: &[f64], index: f64) -> f64 {
        let c1 = index.floor() as usize;
        // let c0 = (c1 - 1).max(0);
         let c0 = if c1 == 0 { 0 } else { c1 - 1 };
        let c2 = (c1 + 1).min(points.len() - 1);

        let s = index - c0 as f64;

        let w0 = -s/3.0 + s.powi(2)/2.0 - s.powi(3)/6.0;
        let w1 = 1.0 - s.powi(2) + (s.powi(3) - s)/2.0;
        let w2 = s + (s.powi(2) - s.powi(3))/2.0 + (s.powi(3) - s)/6.0;

        w0 * points[c0] + w1 * points[c1] + w2 * points[c2]
    }
}
