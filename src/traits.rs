use crate::vectors::Vector2;

pub trait State {
    fn get_coordinates(&self) -> &Vec<Vector2>;
    fn get_velocities(&self) -> &Vec<Vector2>;
    fn set_coordinates(&mut self, coordinates: Vec<Vector2>);
    fn set_velocities(&mut self, velocities: Vec<Vector2>);
}

pub trait MechanicalSystem {
    type State: State;
    fn get_state(&self) -> &Self::State;
    fn get_state_mut(&mut self) -> &mut Self::State;
    fn accelerations(&self, state: &Self::State) -> Vec<Vector2>;
}

pub trait Integrator<System: MechanicalSystem> {
    fn step(&self, system: &mut System, dt: f64);
}
