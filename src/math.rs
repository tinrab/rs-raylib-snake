use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

impl Vector2i {
    pub fn new(x: i32, y: i32) -> Self {
        Vector2i { x, y }
    }
}

impl Add<Vector2i> for Vector2i {
    type Output = Vector2i;

    fn add(self, rhs: Vector2i) -> Self::Output {
        Vector2i {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vector2i> for Vector2i {
    type Output = Vector2i;

    fn sub(self, rhs: Vector2i) -> Self::Output {
        Vector2i {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i32> for Vector2i {
    type Output = Vector2i;

    fn mul(self, rhs: i32) -> Self::Output {
        Vector2i {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<i32> for Vector2i {
    type Output = Vector2i;

    fn div(self, rhs: i32) -> Self::Output {
        Vector2i {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
