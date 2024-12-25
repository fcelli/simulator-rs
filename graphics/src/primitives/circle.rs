use super::Drawable;

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: u32,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64, color: u32) -> Self {
        Circle {
            x,
            y,
            radius,
            color,
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Convert circle center and radius to integers
        let x_c = self.x as isize;
        let y_c = self.y as isize;
        let radius = self.radius as isize;

        // Define the bounding box for the circle
        let x_min = (x_c - radius).max(0) as usize;
        let x_max = (x_c + radius).min(width as isize - 1) as usize;
        let y_min = (y_c - radius).max(0) as usize;
        let y_max = (y_c + radius).min(height as isize - 1) as usize;

        // Iterate over the bounding box and check if each pixel lies within the circle
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let dx = x as isize - x_c;
                let dy = y as isize - y_c;

                // Check if the point is inside the circle
                if dx * dx + dy * dy <= radius * radius {
                    buffer[y * width + x] = self.color;
                }
            }
        }
    }
}
