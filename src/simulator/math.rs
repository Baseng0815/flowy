use epaint::{Vec2, vec2};

#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64
}

pub fn vector2(x: f64, y: f64) -> Vector2 {
    Vector2 { x, y }
}

impl Vector2 {
    pub fn len_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn clamp(&self, min: f64, max: f64) -> Self {
        Self { x: self.x.clamp(min, max), y: self.y.clamp(min, max) }
    }
}

impl std::ops::Mul<Vector2> for f64 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        vector2(self * rhs.x, self * rhs.y)
    }
}

impl std::ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        vector2(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        vector2(self.x - rhs.x, self.y - rhs.y)
    }
}

impl From<Vector2> for Vec2 {
    fn from(value: Vector2) -> Self {
        vec2(value.x as f32, value.y as f32)
    }
}
