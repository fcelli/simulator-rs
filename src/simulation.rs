use ggez::{
    self,
    event::EventHandler,
    glam::Vec2,
    graphics::{self, Color, DrawParam, Mesh},
    Context, GameResult,
};

use crate::{
    systems::NBodyState,
    traits::{Integrator, MechanicalSystem, State},
};

pub struct Simulation<S> {
    state: S,
    system: Box<dyn MechanicalSystem<State = S>>,
    integrator: Box<dyn Integrator<S>>,
    dt: f64,
}

impl<S: State> Simulation<S> {
    pub fn new(
        initial_state: S,
        system: Box<dyn MechanicalSystem<State = S>>,
        integrator: Box<dyn Integrator<S>>,
        dt: f64,
    ) -> Self {
        Self {
            state: initial_state,
            system,
            integrator,
            dt,
        }
    }
}

impl EventHandler for Simulation<NBodyState> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.integrator
            .step(&*self.system, &mut self.state, self.dt);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for pos in &self.state.coordinates {
            let x: f32 = pos.x as f32;
            let y: f32 = pos.y as f32;
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
