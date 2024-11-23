pub mod integrators;
pub mod physics;
pub mod render;
pub mod systems;
pub mod vector2;

use integrators::Integrator;
use systems::System;

pub struct Simulation<S: System> {
    system: S,
    integrator: Box<dyn Integrator<S>>,
    dt: f64,
}

impl<S: System> Simulation<S> {
    pub fn new(system: S, integrator: Box<dyn Integrator<S>>, dt: f64) -> Self {
        Self {
            system,
            integrator,
            dt,
        }
    }

    pub fn update(&mut self) {
        self.integrator.step(&mut self.system, self.dt);
    }
}
