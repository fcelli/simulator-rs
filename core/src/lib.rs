pub mod integrators;
pub mod physics;
pub mod vector2;

use vector2::Vector2;

#[derive(Clone)]
pub struct Coordinates {
    pub position: Vector2,
    pub velocity: Vector2,
}

impl Coordinates {
    pub fn new(position: Vector2, velocity: Vector2) -> Self {
        Coordinates { position, velocity }
    }
}

pub trait System {
    fn get_coordinates(&self) -> &Vec<Coordinates>;
    fn get_coordinates_mut(&mut self) -> &mut Vec<Coordinates>;
    fn calculate_accelerations(&self) -> Vec<Vector2>;
}
