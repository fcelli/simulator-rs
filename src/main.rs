mod body;
mod integrator;
mod physics;
mod simulation;
mod utils;

use body::Body;
use ggez::{
    self,
    conf::{WindowMode, WindowSetup},
    ContextBuilder, GameResult,
};
use integrator::{EulerIntegrator, Integrator};
use simulation::Simulation;
use utils::Vector2D;

const DEFAULT_WINDOW_HEIGHT: f32 = 960f32;
const DEFAULT_WINDOW_WIDTH: f32 = 1440f32;

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("N-Body Simulation", "fcelli")
        .window_setup(WindowSetup::default().title("N-Body Simulation"))
        .window_mode(WindowMode::default().dimensions(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT))
        .build()
        .unwrap();

    // Create some bodies for the simulation
    let mut bodies = vec![
        Body::new(Vector2D::new(0.0, -50.0), Vector2D::new(2.0, 0.0), 1000.0),
        Body::new(Vector2D::new(0.0, 50.0), Vector2D::new(-2.0, 0.0), 1000.0),
        Body::new(Vector2D::new(0.0, 150.0), Vector2D::new(4.0, 0.0), 0.001),
    ];

    // Redefine positions to be relative to the centre of the screen
    for body in bodies.iter_mut() {
        body.position.x += DEFAULT_WINDOW_WIDTH as f64 / 2.0;
        body.position.y += DEFAULT_WINDOW_HEIGHT as f64 / 2.0;
    }

    // Initialize an integrator
    let integrator: Box<dyn Integrator> = Box::new(EulerIntegrator::new());

    // Create the main simulation state
    let dt: f64 = 0.5;
    let state = Simulation::new(bodies, integrator, dt);

    // Run the simulation
    ggez::event::run(ctx, event_loop, state);
}
