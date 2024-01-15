mod car;
mod track;

use graphics::color::BLACK;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

use crate::{Pos, Dim};
use crate::RenderContext;
use car::Car;
use track::Track;

const GRASS_COLOR: [f32; 4] = [0.5, 0.72, 0.56, 1.0];
const GRAY: [f32; 4] = [0.44, 0.4, 0.46, 1.0];

pub struct Race {
    t: u64,
    n_cars: u32,
    cars: Vec<Car>,
    size: Dim,
}

impl Race {


    pub fn new (n_cars: u32, size: &Dim, start_pos: Pos, start_orientation: f32) -> Race {
        println!("Generating new race");
        let mut cars = Vec::new();
        for i in 0..n_cars {
            cars.push(Car::new(i, start_pos, start_orientation));
        }

        let new_race = Race { t: 0, n_cars, cars, size: size.clone() };


        new_race
    }

    pub fn render(&self, gl: &mut GlGraphics, render_args: &RenderArgs, render_context: &RenderContext) {
        // drawing grass
        gl.draw(render_args.viewport(), |c, gl| {
            graphics::clear(GRASS_COLOR, gl);
        });

        // drawing border
        gl.draw(render_args.viewport(), |c,gl| {
            let border_rect: graphics::types::Rectangle = [0.0, 0.0, self.size.w, self.size.h];
            let transform = render_context.apply_transformation(c.transform);

            graphics::rectangle(BLACK, border_rect, transform, gl)
        });
        gl.draw(render_args.viewport(), |c,gl| {
            let border_rect: graphics::types::Rectangle = [1.0, 1.0, self.size.w - 2.0, self.size.h - 2.0];
            let transform = render_context.apply_transformation(c.transform);

            graphics::rectangle(GRASS_COLOR, border_rect, transform, gl)
        });

        // drawing cars
        for car in self.cars.iter() {
            car.render(gl, render_args, render_context);
        }
    }

    pub fn update(&mut self, gl: &mut GlGraphics, update_args: &UpdateArgs) {
        for car in self.cars.iter_mut() {
            car.update(gl, update_args);
        }
        self.t += 1;
        if self.t % 60 == 0 {
            //println!("t: {}", self.t);
        }
    }

}

pub use track::generate_track;