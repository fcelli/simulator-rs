mod window_renderer;

use crate::simulation::systems::System;
pub use window_renderer::WindowRenderer;

pub trait Renderer<S: System> {
    fn render(&mut self, system: &S);
}
