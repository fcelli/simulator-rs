use crate::traits::MechanicalSystem;
use crate::traits::{Integrator, State};

pub struct EulerIntegrator;

impl<S: State> Integrator<S> for EulerIntegrator {
    fn step(&self, system: &dyn MechanicalSystem<State = S>, state: &mut S, dt: f64) {
        let accelerations = system.accelerations(state);
        let coordinates = state
            .coordinates()
            .iter()
            .zip(state.velocities().iter())
            .map(|(x, v)| x + &(v * dt))
            .collect();
        state.update_coordinates(coordinates);
        let velocities = state
            .velocities()
            .iter()
            .zip(accelerations.iter())
            .map(|(v, a)| v + &(a * dt))
            .collect();
        state.update_velocities(velocities);
    }
}

pub struct EulerCromerIntegrator;

impl<S: State> Integrator<S> for EulerCromerIntegrator {
    fn step(&self, system: &dyn MechanicalSystem<State = S>, state: &mut S, dt: f64) {
        let accelerations = system.accelerations(state);
        let velocities = state
            .velocities()
            .iter()
            .zip(accelerations.iter())
            .map(|(v, a)| v + &(a * dt))
            .collect();
        state.update_velocities(velocities);
        let coordinates = state
            .coordinates()
            .iter()
            .zip(state.velocities().iter())
            .map(|(x, v)| x + &(v * dt))
            .collect();
        state.update_coordinates(coordinates);
    }
}
