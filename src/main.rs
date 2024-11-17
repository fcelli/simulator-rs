use ggez::{
    self,
    conf::{WindowMode, WindowSetup},
    ContextBuilder, GameResult,
};
use simulator_rs::{
    integrators::{Integrator, LeapfrogIntegrator},
    simulation::Simulation,
    systems::{MechanicalSystem, NBodySystem},
};

const DEFAULT_WINDOW_HEIGHT: f32 = 960f32;
const DEFAULT_WINDOW_WIDTH: f32 = 1440f32;

fn main() -> GameResult {
    // Initialise the window
    let (ctx, event_loop) = ContextBuilder::new("simulator-rs", "fcelli")
        .window_setup(WindowSetup::default().title("simulator-rs"))
        .window_mode(WindowMode::default().dimensions(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT))
        .build()
        .unwrap();

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
    let state = Simulation::new(system, integrator, dt);

    // Run the simulation
    ggez::event::run(ctx, event_loop, state);
}
