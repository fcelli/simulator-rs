use crate::{systems::Coordinates, vectors::Vector2};

pub trait MechanicalSystem {
    fn get_coordinates(&self) -> &Vec<Coordinates>;
    fn get_coordinates_mut(&mut self) -> &mut Vec<Coordinates>;
    fn calculate_accelerations(&self) -> Vec<Vector2>;
}

pub trait Integrator<System: MechanicalSystem> {
    fn step(&self, system: &mut System, dt: f64);
}
