use super::Integrator;
use crate::simulation::systems::{Coordinates, System};

/// Runge-Kutta integrator.
///
/// x_{i + 1} = x_i + (1 / 6) * (k1 + 2 * k2 + 2 * k3 + k4)
///
/// v_{i + 1} = v_i + (1 / 6) * (k1 + 2 * k2 + 2 * k3 + k4)
pub struct RK4Integrator;

impl RK4Integrator {
    fn derive<S: System>(&self, system: &S) -> Vec<Coordinates> {
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

    fn update<S: System>(
        &self,
        system: &mut S,
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
                coord.position = initial.position.clone() + &k.position * dt;
                coord.velocity = initial.velocity.clone() + &k.velocity * dt;
            });
    }
}

impl<S: System> Integrator<S> for RK4Integrator {
    fn step(&self, system: &mut S, dt: f64) {
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
