pub struct Vector2 {
    x: f64,
    y: f64
}

pub fn vector2(x: f64, y: f64) -> Vector2 {
    Vector2 { x, y }
}

impl Vector2 {
    pub fn len_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }
}