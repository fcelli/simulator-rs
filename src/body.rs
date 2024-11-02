#![allow(dead_code)]
use crate::utils::Vector2D;

const G: f64 = 1.0;

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

    pub fn gravitational_force(&self, b: &Body) -> Vector2D {
        // Computes the gravitational force of body b on self.
        let direction: Vector2D = self.direction(b);
        let r: f64 = direction.magnitude();
        if r == 0.0 {
            // if the bodies overlap, do not apply any force.
            return Vector2D::zero();
        }
        let r2: f64 = r * r;
        let magnitude: f64 = G * self.mass * b.mass / r2;
        direction.normalize() * magnitude
    }

    pub fn print(&self) {
        println!("Position: {}", self.position.to_str());
        println!("Velocity: {}", self.velocity.to_str());
        println!("Mass: {}", self.mass)
    }
}
