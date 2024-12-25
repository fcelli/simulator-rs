use core::System;

use crate::Renderable;
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

    pub fn get_buffer_mut(&mut self) -> &mut Vec<u32> {
        &mut self.buffer
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn render<S: System + Renderable>(&mut self, system: &S) {
        // Clear the buffer
        self.buffer.fill(0);
        // Draw all drawables
        let (width, height) = self.window.get_size();
        for drawable in system.get_drawables() {
            drawable.draw(&mut self.buffer, width, height);
        }
        // Update window with buffer
        self.window
            .update_with_buffer(&self.buffer, width, height)
            .unwrap();
    }
}
