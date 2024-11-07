#![allow(dead_code)]

use crate::body::Body;
use crate::physics;
use crate::utils::Vector2D;

pub trait Integrator {
    fn step(&mut self, bodies: &mut Vec<Body>, dt: f64);

    fn compute_forces(&self, bodies: &[Body]) -> Vec<Vector2D> {
        let mut forces = vec![Vector2D::zero(); bodies.len()];
        for i in 0..bodies.len() {
            for j in i + 1..bodies.len() {
                let force: Vector2D = physics::gravitational_force(&bodies[i], &bodies[j]);
                let neg_force: Vector2D = -&force;
                forces[i] += force;
                forces[j] += neg_force;
            }
        }
        forces
    }
}

pub struct EulerIntegrator {}

impl EulerIntegrator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Integrator for EulerIntegrator {
    fn step(&mut self, bodies: &mut Vec<Body>, dt: f64) {
        let forces = self.compute_forces(bodies);
        for (body, force) in bodies.iter_mut().zip(forces.iter()) {
            let acceleration: Vector2D = force * (1.0 / body.mass);
            body.position += &body.velocity * dt;
            body.velocity += acceleration * dt;
        }
    }
}

pub struct EulerCromerIntegrator {}

impl EulerCromerIntegrator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Integrator for EulerCromerIntegrator {
    fn step(&mut self, bodies: &mut Vec<Body>, dt: f64) {
        let forces = self.compute_forces(bodies);
        for (body, force) in bodies.iter_mut().zip(forces.iter()) {
            let acceleration: Vector2D = force * (1.0 / body.mass);
            body.velocity += acceleration * dt;
            body.position += &body.velocity * dt;
        }
    }
}
