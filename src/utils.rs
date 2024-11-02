#[derive(Clone)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
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

use std::ops::{Add, AddAssign, Mul, Neg, Sub};

impl Add for Vector2D {
    type Output = Vector2D;
    fn add(self, rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for &Vector2D {
    type Output = Vector2D;
    fn add(self, rhs: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, rhs: Vector2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;
    fn sub(self, rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
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

impl Mul<f64> for Vector2D {
    type Output = Vector2D;
    fn mul(self, scalar: f64) -> Vector2D {
        Vector2D {
            x: self.x * scalar,
            y: self.y * scalar,
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

impl Neg for Vector2D {
    type Output = Vector2D;
    fn neg(self) -> Self::Output {
        Vector2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Neg for &Vector2D {
    type Output = Vector2D;
    fn neg(self) -> Self::Output {
        Vector2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    fn setup_test_data() -> (Vector2D, Vector2D, Vector2D) {
        (
            Vector2D::new(0.0, 0.0),
            Vector2D::new(3.0, 4.0),
            Vector2D::new(1.0, 2.0),
        )
    }

    #[test]
    fn test_magnitude() {
        let (v1, v2, v3) = setup_test_data();
        assert_eq!(v1.magnitude(), 0.0);
        assert_eq!(v2.magnitude(), 5.0);
        assert_abs_diff_eq!(v3.magnitude(), 2.2360679, epsilon = 1e-7);
    }

    #[test]
    fn test_normalize() {
        let (mut v1, mut v2, mut v3) = setup_test_data();
        v1 = v1.normalize();
        v2 = v2.normalize();
        v3 = v3.normalize();
        assert_abs_diff_eq!(v1.magnitude(), 0.0, epsilon = 1e-7);
        assert_abs_diff_eq!(v2.magnitude(), 1.0, epsilon = 1e-7);
        assert_abs_diff_eq!(v3.magnitude(), 1.0, epsilon = 1e-7);
    }

    #[test]
    fn test_add() {
        let (v1, v2, v3) = setup_test_data();
        let r = v1 + v2 + v3;
        assert_eq!(r.x, 4.0);
        assert_eq!(r.y, 6.0);
    }

    #[test]
    fn test_sub() {
        let (v1, v2, v3) = setup_test_data();
        let r = v1 - v2 - v3;
        assert_eq!(r.x, -4.0);
        assert_eq!(r.y, -6.0);
    }

    #[test]
    fn test_mul() {
        let (v1, v2, _) = setup_test_data();
        let r1 = v1 * 2.0;
        let r2 = v2 * 2.0;
        assert_eq!(r1.x, 0.0);
        assert_eq!(r1.y, 0.0);
        assert_eq!(r2.x, 6.0);
        assert_eq!(r2.y, 8.0);
    }
}
