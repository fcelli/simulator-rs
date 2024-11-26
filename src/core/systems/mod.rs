mod nbody;

use super::vector2::Vector2;
pub use nbody::NBodySystem;

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
