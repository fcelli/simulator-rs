mod window_renderer;

use crate::core::systems::System;
pub use window_renderer::WindowRenderer;

pub trait Renderer<S: System> {
    fn render(&mut self, system: &S);
}
