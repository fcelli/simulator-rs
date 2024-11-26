pub mod integrators;
pub mod physics;
pub mod systems;
pub mod vector2;

use crate::graphics::Renderer;
use integrators::Integrator;
use systems::System;

pub struct Simulation<S: System, I: Integrator<S>> {
    system: S,
    integrator: I,
    dt: f64,
}

impl<S: System, I: Integrator<S>> Simulation<S, I> {
    pub fn new(system: S, integrator: I, dt: f64) -> Self {
        Self {
            system,
            integrator,
            dt,
        }
    }

    pub fn update(&mut self) {
        self.integrator.step(&mut self.system, self.dt);
    }

    pub fn render<R: Renderer<S>>(&self, renderer: &mut R) {
        renderer.render(&self.system);
    }
}
