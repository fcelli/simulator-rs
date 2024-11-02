mod body;
mod integrator;
mod utils;

use body::Body;
use integrator::{EulerIntegrator, Integrator};
use utils::Vector2D;

fn main() {
    let body1 = Body::new(Vector2D::new(0.0, 0.0), Vector2D::new(0.5, 0.0), 1.0);
    let body2 = Body::new(Vector2D::new(0.0, 1.0), Vector2D::new(-0.5, 0.0), 1.0);
    let body3 = Body::new(Vector2D::new(0.0, -1.0), Vector2D::new(0.8, 0.0), 0.001);
    let bodies = vec![body1, body2, body3];
    let mut integrator = EulerIntegrator::new(bodies);
    for _ in 0..200 {
        integrator.step(0.01);
    }
}
