use crate::body::Body;
use crate::utils::Vector2D;

pub trait Integrator {
    fn step(&mut self, bodies: &mut Vec<Body>, dt: f64);
}

pub struct EulerIntegrator {}

impl EulerIntegrator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Integrator for EulerIntegrator {
    fn step(&mut self, bodies: &mut Vec<Body>, dt: f64) {
        let mut forces = vec![Vector2D::zero(); bodies.len()];
        // Compute forces
        for i in 0..bodies.len() {
            for j in i + 1..bodies.len() {
                let force: Vector2D = bodies[i].gravitational_force(&bodies[j]);
                let neg_force: Vector2D = -&force;
                forces[i] += force;
                forces[j] += neg_force;
            }
        }
        // Apply forces
        for (body, force) in bodies.iter_mut().zip(forces.iter()) {
            let acceleration: Vector2D = force * (1.0 / body.mass);
            body.velocity += acceleration * dt;
            body.position += &body.velocity * dt;
        }
    }
}
