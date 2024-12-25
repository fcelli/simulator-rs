use super::Integrator;
use crate::{vector2::Vector2, System};

/// Leapfrog integrator.
///
/// v_{i + 1 / 2} = v_i + a_i * dt / 2
///
/// x_{i + 1} = x_i + v_{i + 1 / 2} * dt
///
/// v_{i + 1} = v_{i + 1 / 2} + a_{i + 1} * dt / 2
pub struct LeapfrogIntegrator;

impl LeapfrogIntegrator {
    fn update_positions<S: System>(&self, system: &mut S, dt: f64) {
        system
            .get_coordinates_mut()
            .iter_mut()
            .for_each(|coord| coord.position += &coord.velocity * dt);
    }

    fn update_velocities<S: System>(&self, system: &mut S, accelerations: &[Vector2], dt: f64) {
        system
            .get_coordinates_mut()
            .iter_mut()
            .zip(accelerations.iter())
            .for_each(|(coord, a)| coord.velocity += a * dt);
    }
}

impl<S: System> Integrator<S> for LeapfrogIntegrator {
    fn step(&self, system: &mut S, dt: f64) {
        let dt_div2 = dt / 2.0;
        let accelerations = system.calculate_accelerations();
        self.update_velocities(system, &accelerations, dt_div2);
        self.update_positions(system, dt);
        let accelerations = system.calculate_accelerations();
        self.update_velocities(system, &accelerations, dt_div2);
    }
}
