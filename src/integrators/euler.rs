use super::Integrator;
use crate::systems::MechanicalSystem;

pub struct EulerIntegrator;

impl<System: MechanicalSystem> Integrator<System> for EulerIntegrator {
    fn step(&self, system: &mut System, dt: f64) {
        let accelerations = system.calculate_accelerations();
        let coordinates = system.get_coordinates_mut();

        // Update positions
        coordinates
            .iter_mut()
            .for_each(|coord| coord.position += &coord.velocity * dt);

        // Update velocities
        coordinates
            .iter_mut()
            .zip(accelerations.iter())
            .for_each(|(coord, a)| coord.velocity += a * dt);
    }
}
