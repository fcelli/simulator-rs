use crate::{
    integrators::Integrator,
    systems::{Draw, MechanicalSystem},
};

pub struct Simulation<System: MechanicalSystem> {
    system: System,
    integrator: Box<dyn Integrator<System>>,
    dt: f64,
}

impl<System: MechanicalSystem + Draw> Simulation<System> {
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

    pub fn draw(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        self.system.draw(buffer, width, height);
    }
}
