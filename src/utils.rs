pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn magnitude(&self) -> f64 {
        let x2: f64 = self.x * self.x;
        let y2: f64 = self.y * self.y;
        (x2 + y2).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag: f64 = self.magnitude();
        if mag == 0.0 {
            Self { x: 0.0, y: 0.0 }
        } else {
            Self {
                x: self.x / mag,
                y: self.y / mag,
            }
        }
    }

    pub fn to_str(&self) -> String {
        format!("[{}, {}]", self.x, self.y)
    }
}

use std::ops::{Add, Mul, Sub};

impl Add for &Vector2D {
    type Output = Vector2D;
    fn add(self, rhs: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for &Vector2D {
    type Output = Vector2D;
    fn sub(self, rhs: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f64> for &Vector2D {
    type Output = Vector2D;
    fn mul(self, scalar: f64) -> Vector2D {
        Vector2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}
