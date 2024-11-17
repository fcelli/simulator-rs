use ggez::{
    self,
    conf::{WindowMode, WindowSetup},
    ContextBuilder, GameResult,
};
use simulator_rs::{
    integrators::{Integrator, RK4Integrator},
    simulation::Simulation,
    systems::{Coordinates, NBodySystem},
    vectors::Vector2,
};

const DEFAULT_WINDOW_HEIGHT: f32 = 960f32;
const DEFAULT_WINDOW_WIDTH: f32 = 1440f32;

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("simulator-rs", "fcelli")
        .window_setup(WindowSetup::default().title("simulator-rs"))
        .window_mode(WindowMode::default().dimensions(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT))
        .build()
        .unwrap();

    // Create some bodies for the simulation
    let mut system = NBodySystem {
        coordinates: vec![
            Coordinates::new(Vector2::new(0.0, -50.0), Vector2::new(2.0, 0.0)),
            Coordinates::new(Vector2::new(0.0, 50.0), Vector2::new(-2.0, 0.0)),
            Coordinates::new(Vector2::new(0.0, 150.0), Vector2::new(4.0, 0.0)),
        ],
        masses: vec![1000.0, 1000.0, 0.001],
    };

    // Redefine positions to be relative to the centre of the screen
    for coord in system.coordinates.iter_mut() {
        coord.position.x += DEFAULT_WINDOW_WIDTH as f64 / 2.0;
        coord.position.y += DEFAULT_WINDOW_HEIGHT as f64 / 2.0;
    }

    // Initialize an integrator
    let integrator: Box<dyn Integrator<NBodySystem>> = Box::new(RK4Integrator);

    // Create the main simulation state
    let dt: f64 = 0.5;
    let state = Simulation::new(system, integrator, dt);

    // Run the simulation
    ggez::event::run(ctx, event_loop, state);
}
