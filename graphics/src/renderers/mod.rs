mod window;

use core::System;
pub use window::WindowRenderer;

pub trait Renderer<S: System> {
    fn render(&mut self, system: &S);
}
