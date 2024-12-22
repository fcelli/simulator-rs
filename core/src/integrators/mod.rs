mod euler;
mod euler_cromer;
mod leapfrog;
mod rk4;

use crate::System;
pub use euler::EulerIntegrator;
pub use euler_cromer::EulerCromerIntegrator;
pub use leapfrog::LeapfrogIntegrator;
pub use rk4::RK4Integrator;

pub trait Integrator<S: System> {
    fn step(&self, system: &mut S, dt: f64);
}
