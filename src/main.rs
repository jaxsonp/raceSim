extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod race;
pub mod colors;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{MouseCursorEvent, MouseRelativeEvent, FocusEvent, CloseEvent, ButtonEvent, MouseScrollEvent};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent, Button, ButtonState, MouseButton};
use piston::window::WindowSettings;

use racetrack_simulator::{Pos, Dim, Simulation, RenderContext};

const N_CARS: u32 = 100;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 800.0;
const MAX_FPS: u32 = 60;
const SCROLL_SPEED: f32 = 0.07;

use colors::*;

fn main() {
    println!("Starting");

    // creating window
    let opengl = OpenGL::V3_2;
    let size = Dim::new(WIDTH, HEIGHT);
    let mut window: Window = WindowSettings::new("Racetrack Simulator", [size.w as f64, size.h as f64])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(true)
        .build()
        .expect("Failed to create window");
    let mut mouse_pos = Pos::zero();
    let mut window_focused = true;
    let mut left_mouse_pressed = false;

    // initializing a render context
    let mut render_context = RenderContext::new();

    // creating the simulation
    let gl = GlGraphics::new(opengl);
    let mut sim = Simulation::new(gl, N_CARS, &size);

    let mut event_settings = EventSettings::new();
    event_settings.max_fps = MAX_FPS as u64;
    let mut events = Events::new(event_settings);
    // event loop
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            // render event
            sim.render(&args, &render_context);

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
                render_context.pos.x += args[0] as f32;
                render_context.pos.y += args[1] as f32;
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
                render_context.pos.x = render_context.pos.x * scroll + mouse_pos.x * (1.0 - scroll);
                render_context.pos.y = render_context.pos.y * scroll + mouse_pos.y * (1.0 - scroll);
                render_context.scale *= scroll;
            }

        } else if let Some(args) = e.button_args() {
            // button event
            if args.button == Button::Mouse(MouseButton::Left) {
                // left mouse button event
                left_mouse_pressed = args.state == ButtonState::Press;
                if left_mouse_pressed {
                    sim.regenerate_track();
                }
            }
        }
    }
}