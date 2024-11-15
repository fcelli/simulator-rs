use crate::{physics::gravitational_force, traits::MechanicalSystem, vectors::Vector2};

pub struct Coordinates {
    pub position: Vector2,
    pub velocity: Vector2,
}

impl Coordinates {
    pub fn new(position: Vector2, velocity: Vector2) -> Self {
        Coordinates { position, velocity }
    }
}

pub struct NBodySystem {
    pub coordinates: Vec<Coordinates>,
    pub masses: Vec<f64>,
}

impl MechanicalSystem for NBodySystem {
    fn get_coordinates(&self) -> &Vec<Coordinates> {
        &self.coordinates
    }

    fn get_coordinates_mut(&mut self) -> &mut Vec<Coordinates> {
        &mut self.coordinates
    }

    fn calculate_accelerations(&self) -> Vec<Vector2> {
        let mut accelerations = vec![Vector2::zero(); self.coordinates.len()];
        for i in 0..self.coordinates.len() {
            for j in i + 1..self.coordinates.len() {
                let pos_i = &self.coordinates[i].position;
                let mass_i = &self.masses[i];
                let pos_j = &self.coordinates[j].position;
                let mass_j = &self.masses[j];
                let force = gravitational_force(pos_i, mass_i, pos_j, mass_j);
                let neg_force = -&force;
                accelerations[i] += force * (1.0 / mass_i);
                accelerations[j] += neg_force * (1.0 / mass_j);
            }
        }
        accelerations
    }
}
