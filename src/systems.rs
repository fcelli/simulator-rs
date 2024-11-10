use crate::{
    physics::gravitational_force,
    traits::{MechanicalSystem, State},
    vectors::Vector2,
};

pub struct NBodyState {
    pub coordinates: Vec<Vector2>,
    pub velocities: Vec<Vector2>,
    pub masses: Vec<f64>,
}

impl State for NBodyState {
    fn coordinates(&self) -> &Vec<Vector2> {
        &self.coordinates
    }

    fn velocities(&self) -> &Vec<Vector2> {
        &self.velocities
    }

    fn update_coordinates(&mut self, coordinates: Vec<Vector2>) {
        self.coordinates = coordinates;
    }

    fn update_velocities(&mut self, velocities: Vec<Vector2>) {
        self.velocities = velocities;
    }
}

pub struct NBodySystem;

impl MechanicalSystem for NBodySystem {
    type State = NBodyState;

    fn accelerations(&self, state: &Self::State) -> Vec<Vector2> {
        let mut accelerations = vec![Vector2::zero(); state.coordinates.len()];
        for i in 0..state.coordinates.len() {
            for j in i + 1..state.coordinates.len() {
                let pos_i = &state.coordinates[i];
                let mass_i = &state.masses[i];
                let pos_j = &state.coordinates[j];
                let mass_j = &state.masses[j];
                let force = gravitational_force(pos_i, mass_i, pos_j, mass_j);
                let neg_force = -&force;
                accelerations[i] += force * (1.0 / mass_i);
                accelerations[j] += neg_force * (1.0 / mass_j);
            }
        }
        accelerations
    }
}
