use super::Integrator;
use crate::simulation::{systems::System, vector2::Vector2};

/// Euler-Cromer integrator.
///
/// v_{i + 1} = v_i + a_i * dt
///
/// x_{i + 1} = x_i + v_{i + 1} * dt
pub struct EulerCromerIntegrator;

impl EulerCromerIntegrator {
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

impl<S: System> Integrator<S> for EulerCromerIntegrator {
    fn step(&self, system: &mut S, dt: f64) {
        let accelerations = system.calculate_accelerations();
        self.update_velocities(system, &accelerations, dt);
        self.update_positions(system, dt);
    }
}
