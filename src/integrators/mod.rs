mod euler;
mod euler_cromer;
mod leapfrog;
mod rk4;

use crate::systems::MechanicalSystem;
pub use euler::EulerIntegrator;
pub use euler_cromer::EulerCromerIntegrator;
pub use leapfrog::LeapfrogIntegrator;
pub use rk4::RK4Integrator;

pub trait Integrator<System: MechanicalSystem> {
    fn step(&self, system: &mut System, dt: f64);
}
