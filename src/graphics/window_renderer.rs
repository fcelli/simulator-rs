use super::Renderer;
use crate::core::systems::{NBodySystem, System};
use minifb::{Window, WindowOptions};

pub struct WindowRenderer {
    window: Window,
    buffer: Vec<u32>,
}

impl WindowRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        let window = Window::new("simulator-rs", width, height, WindowOptions::default())
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });
        let buffer = vec![0; width * height];
        Self { window, buffer }
    }

    pub fn is_window_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn get_buffer_mut(&mut self) -> &mut Vec<u32> {
        &mut self.buffer
    }
}

impl Renderer<NBodySystem> for WindowRenderer {
    fn render(&mut self, system: &NBodySystem) {
        // Clear the buffer
        self.buffer.fill(0);

        // Get window size
        let (width, height) = self.window.get_size();

        // Draw each body as a white pixel
        for coord in system.get_coordinates() {
            let x = coord.position.x as usize;
            let y = coord.position.y as usize;
            if x < width && y < height {
                self.buffer[y * width + x] = 0xFFFFFF;
            }
        }

        // Update window with buffer
        self.window
            .update_with_buffer(&self.buffer, width, height)
            .unwrap();
    }
}
