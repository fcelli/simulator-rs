pub mod integrators;
pub mod physics;
pub mod render;
pub mod systems;
pub mod vector2;

use integrators::Integrator;
use systems::MechanicalSystem;

pub struct Simulation<System: MechanicalSystem> {
    system: System,
    integrator: Box<dyn Integrator<System>>,
    dt: f64,
}

impl<System: MechanicalSystem> Simulation<System> {
    pub fn new(system: System, integrator: Box<dyn Integrator<System>>, dt: f64) -> Self {
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
