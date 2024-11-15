mod euler;
mod euler_cromer;

pub use euler::EulerIntegrator;
pub use euler_cromer::EulerCromerIntegrator;

use crate::systems::MechanicalSystem;

pub trait Integrator<System: MechanicalSystem> {
    fn step(&self, system: &mut System, dt: f64);
}
