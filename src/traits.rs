use crate::{systems::Coordinates, vectors::Vector2};

pub trait State {
    fn get_coordinates(&self) -> &Vec<Coordinates>;
    fn get_coordinates_mut(&mut self) -> &mut Vec<Coordinates>;
}

pub trait MechanicalSystem {
    type State: State;
    fn get_state(&self) -> &Self::State;
    fn get_state_mut(&mut self) -> &mut Self::State;
    fn accelerations(&self) -> Vec<Vector2>;
}

pub trait Integrator<System: MechanicalSystem> {
    fn step(&self, system: &mut System, dt: f64);
}
