use crate::systems::MechanicalSystem;

use super::Integrator;

pub struct EulerCromerIntegrator;

impl<System: MechanicalSystem> Integrator<System> for EulerCromerIntegrator {
    fn step(&self, system: &mut System, dt: f64) {
        let accelerations = system.calculate_accelerations();
        let coordinates = system.get_coordinates_mut();

        // update velocities
        coordinates
            .iter_mut()
            .zip(accelerations.iter())
            .for_each(|(coord, a)| coord.velocity += a * dt);

        // update positions
        coordinates
            .iter_mut()
            .for_each(|coord| coord.position += &coord.velocity * dt);
    }
}