use crate::body::Body;
use crate::utils::Vector2D;

pub const G: f64 = 1.0;

pub fn gravitational_force(b1: &Body, b2: &Body) -> Vector2D {
    // Computes the gravitational force of body b2 on b1.
    let direction: Vector2D = b1.direction(b2);
    let r: f64 = direction.magnitude();
    if r == 0.0 {
        // If the bodies overlap, do not apply any force.
        return Vector2D::zero();
    }
    let r2: f64 = r * r;
    let magnitude: f64 = G * b1.mass * b2.mass / r2;
    direction.normalize() * magnitude
}

pub fn potential_energy(b1: &Body, b2: &Body) -> f64 {
    -G * b1.mass * b2.mass / b1.distance(b2)
}

pub fn mechanical_energy(bodies: &[Body]) -> f64 {
    let mut total_kinetic_energy = 0.0;
    let mut total_potential_energy = 0.0;
    for i in 0..bodies.len() {
        total_kinetic_energy += bodies[i].kinetic_energy();
        for j in i + 1..bodies.len() {
            total_potential_energy += potential_energy(&bodies[i], &bodies[j]);
        }
    }
    total_kinetic_energy + total_potential_energy
}
