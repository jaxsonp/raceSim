
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};

const CAR_COLOR: [f32; 4] = [0.95, 0.1, 0.2, 1.0]; // red


pub struct Car {
    x_pos: f32,
    y_pos: f32,
    rotation: f32,
}

impl Car {
    const CAR_WIDTH: f32 = 10.0;
    const CAR_LENGTH: f32 = Car::CAR_WIDTH * 2.0;

    fn new () {

    }

    fn render(&self, gl: &mut GlGraphics, render_args: &RenderArgs) {
        let body_rect = graphics::rectangle::centered([self.x_pos as f64, self.y_pos as f64, Car::CAR_WIDTH as f64, Car::CAR_LENGTH as f64]);

        gl.draw(render_args.viewport(), |c,gl| {
            let transform = c.transform;

            graphics::rectangle(CAR_COLOR, body_rect, transform, gl)
        });
    }
}