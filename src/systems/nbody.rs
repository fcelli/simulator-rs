use super::{Coordinates, MechanicalSystem};
use crate::{
    physics::{gravitational_force, gravitational_potential_energy, kinetic_energy},
    vectors::Vector2,
};

#[derive(Default)]
pub struct NBodySystem {
    coordinates: Vec<Coordinates>,
    masses: Vec<f64>,
}

impl NBodySystem {
    pub fn add_body(&mut self, x: f64, y: f64, vx: f64, vy: f64, m: f64) {
        let position = Vector2::new(x, y);
        let velocity = Vector2::new(vx, vy);
        let coord = Coordinates::new(position, velocity);
        self.coordinates.push(coord);
        self.masses.push(m);
    }

    pub fn mechanical_energy(&self) -> f64 {
        let mut mechanical_energy = 0.0;
        for i in 0..self.coordinates.len() {
            let m_i = &self.masses[i];
            let pos_i = &self.coordinates[i].position;
            let vel_i = &self.coordinates[i].velocity;
            mechanical_energy += kinetic_energy(m_i, vel_i);
            for j in i + 1..self.coordinates.len() {
                let m_j = &self.masses[j];
                let pos_j = &self.coordinates[j].position;
                mechanical_energy += gravitational_potential_energy(pos_i, m_i, pos_j, m_j);
            }
        }
        mechanical_energy
    }
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
