use crate::vectors::Vector2;

pub trait State {
    fn coordinates(&self) -> &Vec<Vector2>;
    fn velocities(&self) -> &Vec<Vector2>;
    fn update_coordinates(&mut self, coordinates: Vec<Vector2>);
    fn update_velocities(&mut self, velocities: Vec<Vector2>);
}

pub trait MechanicalSystem {
    type State: State;

    fn accelerations(&self, state: &Self::State) -> Vec<Vector2>;
}

pub trait Integrator<S: State> {
    fn step(&self, system: &dyn MechanicalSystem<State = S>, state: &mut S, dt: f64);
}
