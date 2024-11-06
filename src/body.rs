#![allow(dead_code)]
use crate::utils::Vector2D;

pub struct Body {
    pub position: Vector2D,
    pub velocity: Vector2D,
    pub mass: f64,
}

impl Body {
    pub fn new(position: Vector2D, velocity: Vector2D, mass: f64) -> Self {
        Self {
            position,
            velocity,
            mass,
        }
    }

    pub fn direction(&self, b: &Body) -> Vector2D {
        &b.position - &self.position
    }

    pub fn distance(&self, b: &Body) -> f64 {
        self.direction(b).magnitude()
    }

    pub fn kinetic_energy(&self) -> f64 {
        let v2: f64 = self.velocity.magnitude() * self.velocity.magnitude();
        0.5 * self.mass * v2
    }

    pub fn print(&self) {
        println!("Position: {}", self.position.to_str());
        println!("Velocity: {}", self.velocity.to_str());
        println!("Mass: {}", self.mass);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    fn setup_test_data() -> (Body, Body) {
        let b1: Body = Body::new(Vector2D::zero(), Vector2D::zero(), 1.0);
        let b2: Body = Body::new(Vector2D::new(1.0, 2.0), Vector2D::zero(), 2.0);
        (b1, b2)
    }

    #[test]
    fn test_direction() {
        let (b1, b2) = setup_test_data();
        let direction = b1.direction(&b2);
        assert_eq!(direction.x, 1.0);
        assert_eq!(direction.y, 2.0);
    }

    #[test]
    fn test_distance() {
        let (b1, b2) = setup_test_data();
        let distance = b1.distance(&b2);
        assert_abs_diff_eq!(distance, 2.2360679, epsilon = 1e-7);
    }
}
