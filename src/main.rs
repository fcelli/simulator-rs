use std::time::{Duration, Instant};

use minifb::{Key, Window, WindowOptions};
use simulator_rs::{
    integrators::{Integrator, LeapfrogIntegrator},
    simulation::Simulation,
    systems::{MechanicalSystem, NBodySystem},
};

const DEFAULT_WINDOW_WIDTH: usize = 800;
const DEFAULT_WINDOW_HEIGHT: usize = 600;

fn main() {
    // Create N-body system
    let mut system = NBodySystem::default();
    system.add_body(0.0, -50.0, 2.0, 0.0, 1000.0);
    system.add_body(0.0, 50.0, -2.0, 0.0, 1000.0);
    system.add_body(0.0, 150.0, 4.0, 0.0, 0.001);
    system.add_body(0.0, -200.0, -3.0, 0.0, 0.001);
    system.add_body(0.0, 400.0, 1.5, 0.0, 0.001);

    // Redefine positions to be relative to the centre of the window
    system.get_coordinates_mut().iter_mut().for_each(|coord| {
        coord.position.x += DEFAULT_WINDOW_WIDTH as f64 / 2.0;
        coord.position.y += DEFAULT_WINDOW_HEIGHT as f64 / 2.0;
    });

    // Initialize an integrator
    let integrator: Box<dyn Integrator<NBodySystem>> = Box::new(LeapfrogIntegrator);

    // Create the main simulation state
    let dt: f64 = 0.5;
    let mut simulation = Simulation::new(system, integrator, dt);

    // Initialize the window
    let mut window = Window::new(
        "simulator_rs",
        DEFAULT_WINDOW_WIDTH,
        DEFAULT_WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Create the pixel buffer
    let mut buffer = vec![0; DEFAULT_WINDOW_WIDTH * DEFAULT_WINDOW_HEIGHT];

    // Run the simulation loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let start_time = Instant::now();

        // Update simulation state
        simulation.update();

        // Draw simulation state to window
        simulation.draw(&mut buffer, DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT);

        // Update the window with the new frame
        window
            .update_with_buffer(&buffer, DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT)
            .unwrap();

        // Control simulation speed
        let frame_time = Instant::now().duration_since(start_time);
        if frame_time < Duration::from_millis(16) {
            std::thread::sleep(Duration::from_millis(16) - frame_time);
        }
    }
}
