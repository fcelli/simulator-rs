pub struct Body {
    pub position: [f64; 2],
    pub velocity: [f64; 2],
    pub mass: f64,
}

impl Body {
    pub fn new(x: f64, y: f64, vx: f64, vy: f64, m: f64) -> Self {
        Self {
            position: [x, y],
            velocity: [vx, vy],
            mass: m,
        }
    }

    pub fn distance(&self, b: &Body) -> f64 {
        let dx = self.position[0] - b.position[0];
        let dy = self.position[1] - b.position[1];
        (dx * dx + dy * dy).sqrt()
    }

    pub fn print(&self) {
        println!("Position: [{}, {}]", self.position[0], self.position[1]);
        println!("Velocity: [{}, {}]", self.velocity[0], self.velocity[1]);
        println!("Mass: {}", self.mass)
    }
}
