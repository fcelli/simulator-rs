use minifb::{Window, WindowOptions};

pub struct WindowRenderer {
    pub window: Window,
    pub buffer: Vec<u32>,
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
