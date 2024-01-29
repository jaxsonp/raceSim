

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::path::Path;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::{ButtonEvent, CloseEvent, FocusEvent, MouseCursorEvent, MouseRelativeEvent, MouseScrollEvent, UpdateArgs};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent, Button, ButtonState, MouseButton};
use piston::window::WindowSettings;

mod render;
use render::Renderer;

mod race;
use race::{Race, Track, generate_track};

use racetrack_simulator::{Pos, Dim};

const N_CARS: u32 = 1;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 800.0;
const MAX_FPS: u32 = 60;
const SCROLL_SPEED: f32 = 0.07;

pub struct Simulation {
    race: Race,
    size: Dim,
    track: Track,
    track_img: Texture,
}

impl Simulation {
    pub fn new(n_cars: u32, size: &Dim) -> Self {
        println!("Initializing simulation");
        let size = size.clone();

        let track = generate_track(&size);
        let track_img = Texture::from_path(Path::new("track_data/img.png"), &TextureSettings::new()).expect("Failed to load map data");
        let race = Race::new(n_cars, track.start_pos, track.start_orientation);
        Simulation {
            race,
            size,
            track,
            track_img,
        }
    }

    // TEMP FUNCTION, TODO delete
    pub fn regenerate_track(&mut self) -> () {
        self.track = generate_track(&self.size);
    }

    pub fn render(&mut self, renderer: &mut Renderer) {

        renderer.draw_track(&self.track_img);
        renderer.draw_cars(&self.race)
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.race.update(&args);
    }
}

fn main() {
    println!("Starting");

    // creating window
    let opengl = OpenGL::V3_2;
    let size = Dim::new(WIDTH, HEIGHT);
    let mut window: GlutinWindow = WindowSettings::new("Racetrack Simulator", [size.w as f64, size.h as f64])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(true)
        .build()
        .expect("Failed to create window");
    let mut mouse_pos = Pos::zero();
    let mut window_focused = true;
    let mut left_mouse_pressed = false;

    // creating the simulation
    let gl: GlGraphics = GlGraphics::new(opengl);
    let mut sim = Simulation::new(N_CARS, &size);

    // initializing a renderer
    let mut renderer = Renderer::new(gl);

    let mut event_settings = EventSettings::new();
    event_settings.max_fps = MAX_FPS as u64;
    let mut events = Events::new(event_settings);
    // event loop
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            // render event
            renderer.set_render_args(args);
            sim.render(&mut renderer);

        } else if let Some(args) = e.update_args() {
            // update event
            sim.update(&args);

        } else if let Some(args) = e.focus_args() {
            // focus event
            window_focused = args;

        } else if let Some(_) = e.close_args() {
            // close event
            println!("Closing");

        } else if let Some(args) = e.mouse_relative_args() {
            // mouse moved event (rel)
            if left_mouse_pressed && window_focused {
                // panning with user input
                renderer.pan_view(args[0], args[1]);
            }

        } else if let Some(args) = e.mouse_cursor_args() {
            // mouse moved event (abs)
            mouse_pos.update(args[0] as f32, args[1] as f32);

        } else if let Some(args) = e.mouse_scroll_args() {
            // mouse scroll event
            if args[1] != 0.0 {
                // zooming with user input
                let scroll = if args[1] == 1.0 {
                    1.0 / (1.0 - SCROLL_SPEED)
                } else {
                    1.0 - SCROLL_SPEED
                };
                println!("scroll");
                renderer.zoom_view(scroll, mouse_pos);
            }

        } else if let Some(args) = e.button_args() {
            // button event
            if args.button == Button::Mouse(MouseButton::Left) {
                // left mouse button event
                left_mouse_pressed = args.state == ButtonState::Press;

                // DEBUG: regenerate track on click
                /*if left_mouse_pressed {
                    sim.regenerate_track();
                }*/
            }
        }
    }
}
