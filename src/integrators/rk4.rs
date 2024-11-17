use crate::systems::{Coordinates, MechanicalSystem};

use super::Integrator;

pub struct RK4Integrator;

impl RK4Integrator {
    fn derive<System: MechanicalSystem>(&self, system: &System) -> Vec<Coordinates> {
        let accelerations = system.calculate_accelerations();
        let coordinates = system.get_coordinates();
        coordinates
            .iter()
            .zip(accelerations.iter())
            .map(|(coord, a)| Coordinates {
                position: coord.velocity.clone(),
                velocity: a.clone(),
            })
            .collect()
    }

    fn update<System: MechanicalSystem>(
        &self,
        system: &mut System,
        initial: &[Coordinates],
        k: &[Coordinates],
        dt: f64,
    ) {
        let coordinates = system.get_coordinates_mut();
        coordinates
            .iter_mut()
            .zip(initial.iter())
            .zip(k.iter())
            .for_each(|((coord, initial), k)| {
                coord.position = initial.position.clone() + k.position.clone() * dt;
                coord.velocity = initial.velocity.clone() + k.velocity.clone() * dt;
            });
    }
}

impl<System: MechanicalSystem> Integrator<System> for RK4Integrator {
    fn step(&self, system: &mut System, dt: f64) {
        let initial = system.get_coordinates().clone();
        let dt_div2 = dt / 2.0;
        let dt_div6 = dt / 6.0;

        let k1 = self.derive(&*system);
        self.update(system, &initial, &k1, dt_div2);
        let k2 = self.derive(&*system);
        self.update(system, &initial, &k1, dt_div2);
        let k3 = self.derive(&*system);
        self.update(system, &initial, &k1, dt);
        let k4 = self.derive(&*system);

        let coordinates = system.get_coordinates_mut();
        coordinates
            .iter_mut()
            .zip(initial.iter())
            .zip(k1.iter().zip(k2.iter()).zip(k3.iter().zip(k4.iter())))
            .for_each(|((coord, initial), ((k1, k2), (k3, k4)))| {
                coord.position = initial.position.clone()
                    + (k1.position.clone()
                        + k2.position.clone() * 2.0
                        + k3.position.clone() * 2.0
                        + k4.position.clone())
                        * dt_div6;

                coord.velocity = initial.velocity.clone()
                    + (k1.velocity.clone()
                        + k2.velocity.clone() * 2.0
                        + k3.velocity.clone() * 2.0
                        + k4.velocity.clone())
                        * dt_div6;
            });
    }
}
