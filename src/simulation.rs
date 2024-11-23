use crate::{
    integrators::Integrator,
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
    fn render(&self, buffer: &mut Vec<u32>, width: usize, height: usize);
}

impl Render for Simulation<NBodySystem> {
    fn render(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Clear the buffer
        buffer.fill(0);

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
