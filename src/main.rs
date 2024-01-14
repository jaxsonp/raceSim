extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 800;
const MAX_FPS: u32 = 60;

// colors
const GREEN: [f32; 4] = [0.5, 0.72, 0.56, 1.0];
const GRAY: [f32; 4] = [0.44, 0.4, 0.46, 1.0];
const RED: [f32; 4] = [0.95, 0.1, 0.2, 1.0];


pub struct Simulation {
    gl: GlGraphics, // OpenGL drawing backend.
    t: u64,  // ticks since start
    car: Car,
}

impl Simulation {
    fn render(&mut self, render_args: &RenderArgs) {



        self.gl.draw(render_args.viewport(), |c, gl| {
            // draw background
            graphics::clear(GREEN, gl);
        });

        self.car.render(&mut self.gl, render_args);
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.t += 1;
    }
}

struct Car {
    x: f32,
    y: f32,

}

impl Car {
    const CAR_WIDTH: f32 = 10.0;
    const CAR_LENGTH: f32 = Car::CAR_WIDTH * 2.0;

    fn render(&self, gl: &mut GlGraphics, render_args: &RenderArgs) {
        let body_rect = graphics::rectangle::centered([self.x as f64, self.y as f64, Car::CAR_WIDTH as f64, Car::CAR_LENGTH as f64]);

        gl.draw(render_args.viewport(), |c,gl| {
            let transform = c.transform;

            graphics::rectangle(RED, body_rect, transform, gl)
        });
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Racetrack Simulator", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new simulation and run it.
    let mut sim = Simulation {
        gl: GlGraphics::new(opengl),
        t: 0,
        car: Car {x: 60.0, y: 100.0},
    };

    let mut event_settings = EventSettings::new();
    event_settings.max_fps = MAX_FPS as u64;
    let mut events = Events::new(event_settings);
    // event loop
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            sim.render(&args);
        }

        if let Some(args) = e.update_args() {
            sim.update(&args);
        }
    }
}
