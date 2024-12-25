pub mod systems;

use core::{integrators::Integrator, System};
use graphics::{window::WindowRenderer, Renderable};
use std::time::{Duration, Instant};

pub struct Simulation<S: System, I: Integrator<S>> {
    system: S,
    integrator: I,
    dt: f64,
}

impl<S: System + Renderable, I: Integrator<S>> Simulation<S, I> {
    pub fn new(system: S, integrator: I, dt: f64) -> Self {
        Self {
            system,
            integrator,
            dt,
        }
    }

    fn update(&mut self) {
        self.integrator.step(&mut self.system, self.dt);
    }

    pub fn run(&mut self, renderer: &mut WindowRenderer) {
        while renderer.is_open() {
            // Get start time
            let start_time = Instant::now();
            // Update simulation
            self.update();
            // Render simulation
            renderer.render(&self.system);
            // Control simulation speed
            let frame_time = Instant::now().duration_since(start_time);
            if frame_time < Duration::from_millis(1) {
                std::thread::sleep(Duration::from_millis(1) - frame_time);
            }
        }
    }
}
