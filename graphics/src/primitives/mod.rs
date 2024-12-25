mod circle;

pub use circle::Circle;

pub trait Drawable {
    fn draw(&self, buffer: &mut Vec<u32>, width: usize, height: usize);
}
