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

    pub fn distance(&self, b: &Body) -> f64 {
        (&self.position - &b.position).magnitude()
    }

    pub fn print(&self) {
        println!("Position: {}", self.position.to_str());
        println!("Velocity: {}", self.velocity.to_str());
        println!("Mass: {}", self.mass)
    }
}
