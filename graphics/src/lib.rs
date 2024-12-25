pub mod primitives;
pub mod window;

use primitives::Drawable;

pub trait Renderable {
    fn get_drawables(&self) -> Vec<Box<dyn Drawable>>;
}
