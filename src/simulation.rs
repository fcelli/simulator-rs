use crate::body::Body;
use crate::integrator::Integrator;
use crate::physics;
use ggez::{
    self,
    event::EventHandler,
    glam::Vec2,
    graphics::{self, Color, DrawParam, Mesh},
    Context, GameResult,
};

pub struct Simulation {
    bodies: Vec<Body>,
    integrator: Box<dyn Integrator>,
    dt: f64,
    cycle_counter: i32,
}

impl Simulation {
    pub fn new(bodies: Vec<Body>, integrator: Box<dyn Integrator>, dt: f64) -> Self {
        Self {
            bodies,
            integrator,
            dt,
            cycle_counter: 0,
        }
    }
}

impl EventHandler for Simulation {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.integrator.step(&mut self.bodies, self.dt);
        let energy: f64 = physics::mechanical_energy(&self.bodies);
        self.cycle_counter += 1;
        if self.cycle_counter % 100 == 0 {
            println!("Mechanical energy: {}", energy);
            self.cycle_counter = 0
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for body in &self.bodies {
            let x: f32 = body.position.x as f32;
            let y: f32 = body.position.y as f32;
            let circle = Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(x, y),
                5.0,
                0.1,
                Color::WHITE,
            )?;
            canvas.draw(&circle, DrawParam::default());
        }
        canvas.finish(ctx)?;
        Ok(())
    }
}
