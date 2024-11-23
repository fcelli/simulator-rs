use super::{
    systems::{NBodySystem, System},
    Simulation,
};
use crate::rendering::Window;

pub trait Render {
    fn render(&self, window: &mut Window);
}

impl Render for Simulation<NBodySystem> {
    fn render(&self, window: &mut Window) {
        // Clear the buffer
        window.clear(0);

        // Get window size
        let (width, height) = window.get_size();

        // Get the buffer
        let buffer = window.get_buffer_mut();

        // Draw each body as a white pixel
        for coord in self.system.get_coordinates() {
            let x = coord.position.x as usize;
            let y = coord.position.y as usize;
            if x < width && y < height {
                buffer[y * width + x] = 0xFFFFFF;
            }
        }
    }
}
