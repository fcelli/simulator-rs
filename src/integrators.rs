use crate::traits::MechanicalSystem;
use crate::traits::{Integrator, State};

pub struct EulerIntegrator;

impl<System: MechanicalSystem> Integrator<System> for EulerIntegrator {
    fn step(&self, system: &mut System, dt: f64) {
        let accelerations = {
            let state = system.get_state();
            system.accelerations(state)
        };

        let state = system.get_state_mut();

        let new_coordinates = state
            .get_coordinates()
            .iter()
            .zip(state.get_velocities().iter())
            .map(|(x, v)| x + &(v * dt))
            .collect();
        state.set_coordinates(new_coordinates);

        let new_velocities = state
            .get_velocities()
            .iter()
            .zip(accelerations.iter())
            .map(|(v, a)| v + &(a * dt))
            .collect();
        state.set_velocities(new_velocities);
    }
}

pub struct EulerCromerIntegrator;

impl<System: MechanicalSystem> Integrator<System> for EulerCromerIntegrator {
    fn step(&self, system: &mut System, dt: f64) {
        let accelerations = {
            let state = system.get_state();
            system.accelerations(state)
        };

        let state = system.get_state_mut();

        let velocities = state
            .get_velocities()
            .iter()
            .zip(accelerations.iter())
            .map(|(v, a)| v + &(a * dt))
            .collect();
        state.set_velocities(velocities);

        let coordinates = state
            .get_coordinates()
            .iter()
            .zip(state.get_velocities().iter())
            .map(|(x, v)| x + &(v * dt))
            .collect();
        state.set_coordinates(coordinates);
    }
}
