use core::{integrators::LeapfrogIntegrator, System};
use graphics::renderers::WindowRenderer;
use simulations::{systems::NBodySystem, Simulation};
use std::time::{Duration, Instant};

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

    // Initialise an integrator
    let integrator = LeapfrogIntegrator;

    // Initialise a renderer
    let mut renderer = WindowRenderer::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT);

    // Create the main simulation state
    let dt: f64 = 0.5;
    let mut simulation = Simulation::new(system, integrator, dt);

    // Run the simulation loop
    while renderer.is_window_open() {
        let start_time = Instant::now();

        // Update simulation
        simulation.update();

        // Render simulation
        simulation.render(&mut renderer);

        // Control simulation speed
        let frame_time = Instant::now().duration_since(start_time);
        if frame_time < Duration::from_millis(16) {
            std::thread::sleep(Duration::from_millis(16) - frame_time);
        }
    }
}