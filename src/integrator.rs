use crate::body::Body;
use crate::utils::Vector2D;

pub trait Integrator {
    fn step(&mut self, dt: f64);
}

pub struct EulerIntegrator {
    bodies: Vec<Body>,
}

impl EulerIntegrator {
    pub fn new(bodies: Vec<Body>) -> Self {
        Self { bodies }
    }
}

impl Integrator for EulerIntegrator {
    fn step(&mut self, dt: f64) {
        let mut forces = vec![Vector2D::zero(); self.bodies.len()];
        // compute forces
        for i in 0..self.bodies.len() {
            for j in i + 1..self.bodies.len() {
                let force: Vector2D = self.bodies[i].gravitational_force(&self.bodies[j]);
                let neg_force = -&force;
                forces[i] += force;
                forces[j] += neg_force;
            }
        }
        // apply forces
        for (body, force) in self.bodies.iter_mut().zip(forces.iter()) {
            let acceleration: Vector2D = force * (1.0 / body.mass);
            body.velocity += acceleration * dt;
            body.position += &body.velocity * dt;
        }
    }
}
