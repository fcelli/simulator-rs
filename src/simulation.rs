use crate::{
    systems::NBodySystem,
    traits::{Integrator, MechanicalSystem},
};
use ggez::{
    self,
    event::EventHandler,
    glam::Vec2,
    graphics::{self, Color, DrawParam, Mesh},
    Context, GameResult,
};

pub struct Simulation<System: MechanicalSystem> {
    system: System,
    integrator: Box<dyn Integrator<System>>,
    dt: f64,
}

impl<System: MechanicalSystem> Simulation<System> {
    pub fn new(system: System, integrator: Box<dyn Integrator<System>>, dt: f64) -> Self {
        Self {
            system,
            integrator,
            dt,
        }
    }
}

impl EventHandler for Simulation<NBodySystem> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.integrator.step(&mut self.system, self.dt);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for coord in &self.system.coordinates {
            let x: f32 = coord.position.x as f32;
            let y: f32 = coord.position.y as f32;
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
