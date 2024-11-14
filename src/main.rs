use ggez::{
    self,
    conf::{WindowMode, WindowSetup},
    ContextBuilder, GameResult,
};
use nbody::{
    integrators::EulerCromerIntegrator,
    simulation::Simulation,
    systems::{NBodyState, NBodySystem},
    traits::Integrator,
    vectors::Vector2,
};

const DEFAULT_WINDOW_HEIGHT: f32 = 960f32;
const DEFAULT_WINDOW_WIDTH: f32 = 1440f32;

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("N-Body Simulation", "fcelli")
        .window_setup(WindowSetup::default().title("N-Body Simulation"))
        .window_mode(WindowMode::default().dimensions(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT))
        .build()
        .unwrap();

    // Create some bodies for the simulation
    let mut state = NBodyState {
        coordinates: vec![
            Vector2::new(0.0, -50.0),
            Vector2::new(0.0, 50.0),
            Vector2::new(0.0, 150.0),
        ],
        velocities: vec![
            Vector2::new(2.0, 0.0),
            Vector2::new(-2.0, 0.0),
            Vector2::new(4.0, 0.0),
        ],
        masses: vec![1000.0, 1000.0, 0.001],
    };

    // Redefine positions to be relative to the centre of the screen
    for coord in state.coordinates.iter_mut() {
        coord.x += DEFAULT_WINDOW_WIDTH as f64 / 2.0;
        coord.y += DEFAULT_WINDOW_HEIGHT as f64 / 2.0;
    }

    let system = NBodySystem { state };

    // Initialize an integrator
    let integrator: Box<dyn Integrator<NBodySystem>> = Box::new(EulerCromerIntegrator);

    // Create the main simulation state
    let dt: f64 = 0.5;
    let state = Simulation::new(system, integrator, dt);

    // Run the simulation
    ggez::event::run(ctx, event_loop, state);
}
