mod euler;
mod euler_cromer;
mod rk4;

use crate::systems::MechanicalSystem;
pub use euler::EulerIntegrator;
pub use euler_cromer::EulerCromerIntegrator;
pub use rk4::RK4Integrator;

pub trait Integrator<System: MechanicalSystem> {
    fn step(&self, system: &mut System, dt: f64);
}
