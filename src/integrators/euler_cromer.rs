use super::Integrator;
use crate::{systems::MechanicalSystem, vectors::Vector2};

/// Euler-Cromer integrator.
///
/// v_{i + 1} = v_i + a_i * dt
///
/// x_{i + 1} = x_i + v_{i + 1} * dt
pub struct EulerCromerIntegrator;

impl EulerCromerIntegrator {
    fn update_positions<System: MechanicalSystem>(&self, system: &mut System, dt: f64) {
        system
            .get_coordinates_mut()
            .iter_mut()
            .for_each(|coord| coord.position += &coord.velocity * dt);
    }

    fn update_velocities<System: MechanicalSystem>(
        &self,
        system: &mut System,
        accelerations: &[Vector2],
        dt: f64,
    ) {
        system
            .get_coordinates_mut()
            .iter_mut()
            .zip(accelerations.iter())
            .for_each(|(coord, a)| coord.velocity += a * dt);
    }
}

impl<System: MechanicalSystem> Integrator<System> for EulerCromerIntegrator {
    fn step(&self, system: &mut System, dt: f64) {
        let accelerations = system.calculate_accelerations();
        self.update_velocities(system, &accelerations, dt);
        self.update_positions(system, dt);
    }
}
