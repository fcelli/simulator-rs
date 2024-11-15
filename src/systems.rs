use crate::{
    physics::gravitational_force,
    traits::{MechanicalSystem, State},
    vectors::Vector2,
};

pub struct Coordinates {
    pub position: Vector2,
    pub velocity: Vector2,
}

impl Coordinates {
    pub fn new(position: Vector2, velocity: Vector2) -> Self {
        Coordinates { position, velocity }
    }
}

pub struct NBodyState {
    pub coordinates: Vec<Coordinates>,
    pub masses: Vec<f64>,
}

impl State for NBodyState {
    fn get_coordinates(&self) -> &Vec<Coordinates> {
        &self.coordinates
    }

    fn get_coordinates_mut(&mut self) -> &mut Vec<Coordinates> {
        &mut self.coordinates
    }
}

pub struct NBodySystem {
    pub state: NBodyState,
}

impl MechanicalSystem for NBodySystem {
    type State = NBodyState;

    fn get_state(&self) -> &Self::State {
        &self.state
    }

    fn get_state_mut(&mut self) -> &mut Self::State {
        &mut self.state
    }

    fn accelerations(&self, state: &Self::State) -> Vec<Vector2> {
        let mut accelerations = vec![Vector2::zero(); state.coordinates.len()];
        for i in 0..state.coordinates.len() {
            for j in i + 1..state.coordinates.len() {
                let pos_i = &state.coordinates[i].position;
                let mass_i = &state.masses[i];
                let pos_j = &state.coordinates[j].position;
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
