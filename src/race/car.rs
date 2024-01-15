
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

use crate::{Pos, RenderContext, RED};

pub struct Car {
    pub id: u32,
    t: u64,
    pos: Pos,
    orientation: f32,
}

impl Car {
    const LENGTH: f32 = 30.0;
    const WIDTH: f32 = Car::LENGTH / 2.0;

    pub fn new (id: u32, start_pos: Pos, start_orientation: f32) -> Self {
        Car {
            id,
            t: 0,
            pos: start_pos,
            orientation: start_orientation
        }
    }

    pub fn render(&self, gl: &mut GlGraphics, render_args: &RenderArgs, render_context: &RenderContext) {
        gl.draw(render_args.viewport(), |c,gl| {
            let body_rect: graphics::types::Rectangle = [
                (-Car::WIDTH / 2.0) as f64,
                (-Car::LENGTH / 2.0) as f64,
                Car::WIDTH as f64,
                Car::LENGTH as f64
            ];
            let transform = render_context.apply_transformation(c.transform)
                .trans(self.pos.x as f64, self.pos.y as f64)
                .rot_rad(self.orientation as f64);

            graphics::rectangle(*RED, body_rect, transform, gl)
        });
    }

    pub fn update(&mut self, gl: &mut GlGraphics, update_args: &UpdateArgs) {
        self.t += 1;
    }
}