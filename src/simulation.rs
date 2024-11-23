use crate::{
    integrators::Integrator,
    rendering::Window,
    systems::{MechanicalSystem, NBodySystem},
};

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

pub trait Render {
    fn render(&self, window: &mut Window);
}

impl Render for Simulation<NBodySystem> {
    fn render(&self, window: &mut Window) {
        // Clear the buffer
        window.clear(0);

        // Get window size
        let (width, height) = window.get_size();

        // Get the buffer
        let buffer = window.get_buffer_mut();

        // Draw each body as a white pixel
        for coord in self.system.get_coordinates() {
            let x = coord.position.x as usize;
            let y = coord.position.y as usize;
            if x < width && y < height {
                buffer[y * width + x] = 0xFFFFFF;
            }
        }
    }
}
