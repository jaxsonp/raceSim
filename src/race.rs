
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};

use crate::Pos;

mod car;
mod track;

use track::Track;

pub struct Race {
    track: Track,
}

impl Race {

    pub fn new (n_cars: i32, window_w: f32, window_h: f32) -> Race {
        use track::generate_track;
        //let mut vec![Car, n_cars];
        let track = generate_track(window_w, window_h);
        Race { track }
    }

    pub fn render(&self, gl: &mut GlGraphics, render_args: &RenderArgs) {

    }

}