extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod race;
use race::Race;
use racetrack_simulator::Pos;

const WIDTH: f32 = 1000.0;
const HEIGHT: f32 = 800.0;
const MAX_FPS: u32 = 60;

const GRASS_COLOR: [f32; 4] = [0.5, 0.72, 0.56, 1.0];
const GRAY: [f32; 4] = [0.44, 0.4, 0.46, 1.0];


pub struct Simulation {
    gl: GlGraphics, // OpenGL drawing backend.
    race: Race,
}

impl Simulation {
    fn render(&mut self, render_args: &RenderArgs) {

        self.gl.draw(render_args.viewport(), |c, gl| {
            // draw background
            graphics::clear(GRASS_COLOR, gl);
        });

        self.race.render(&mut self.gl, render_args);
    }

    fn update(&mut self, args: &UpdateArgs) {

    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Racetrack Simulator", [WIDTH as f64, HEIGHT as f64])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new simulation and run it.
    let mut sim = Simulation {
        gl: GlGraphics::new(opengl),
        race: Race::new(100, WIDTH, HEIGHT),
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
