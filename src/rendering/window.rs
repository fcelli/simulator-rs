use minifb::{Key, Window as MinifbWindow, WindowOptions};

pub struct Window {
    window: MinifbWindow,
    buffer: Vec<u32>,
}

impl Window {
    pub fn new(width: usize, height: usize) -> Self {
        let window = MinifbWindow::new("simulator_rs", width, height, WindowOptions::default())
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });
        let buffer = vec![0; width * height];
        Self { window, buffer }
    }

    pub fn update(&mut self) -> bool {
        if !self.window.is_open() || self.window.is_key_down(Key::Escape) {
            return false;
        }
        let (width, height) = self.window.get_size();
        self.window
            .update_with_buffer(&self.buffer, width, height)
            .unwrap();
        true
    }

    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.window.get_size()
    }

    pub fn get_buffer_mut(&mut self) -> &mut Vec<u32> {
        &mut self.buffer
    }
}
