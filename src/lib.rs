mod race;

use graphics::Transformed;
use piston::{RenderArgs, UpdateArgs};
use std::fmt;
use opengl_graphics::GlGraphics;

use race::{Race, Track, generate_track};

pub const GREEN: &'static [f32; 4] = &[0.5, 0.72, 0.56, 1.0];
pub const RED: &'static [f32; 4] = &[0.95, 0.1, 0.2, 1.0];
pub const GRAY: &'static [f32; 4] = &[0.44, 0.4, 0.46, 1.0];
pub const BLACK: &'static [f32; 4] = &[0.0, 0.02, 0.02, 1.0];


pub struct Simulation {
    gl: GlGraphics, // OpenGL drawing backend.
    race: Race,
    size: Dim,
    track: Track,
}

impl Simulation {
    pub fn new(gl: GlGraphics, n_cars: u32, size: &Dim) -> Self {
        println!("Initializing simulation");
        let size = size.clone();

        let track = generate_track(&size);

        let race = Race::new(n_cars, &size, track.start_pos, track.start_orientation);
        Simulation { gl, race, size, track, }
    }

    // TEMP FUNCTION, TODO delete
    pub fn regenerate_track(&mut self) -> () {
        self.track = generate_track(&self.size);
    }

    pub fn render(&mut self, render_args: &RenderArgs, render_context: &RenderContext) {
        // drawing grass
        self.gl.draw(render_args.viewport(), |c, gl| {
            graphics::clear(*GREEN, gl);
        });
        self.track.render(&mut self.gl, render_args, render_context);
        self.race.render(&mut self.gl, render_args, render_context);
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.race.update(&mut self.gl, &args)
    }
}

/*
 * struct to pass rendering info into render functions
*/
pub struct RenderContext {
    pub pos: Pos,
    pub scale: f32,
}

impl RenderContext {
    pub fn new () -> Self {
        Self {
            pos: Pos::zero(),
            scale: 1.0,
        }
    }

    pub fn apply_transformation (&self, transform: [[f64; 3]; 2]) -> [[f64; 3]; 2] {
        transform.trans(self.pos.x as f64, self.pos.y as f64).scale(self.scale as f64, self.scale as f64)
    }
}

/*
 * Data struct to elegantly store position data
 */
#[derive(Copy, Clone)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn update(&mut self, x: f32, y: f32) -> () {
        self.x = x;
        self.y = y;
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn mul(&self, a: f32) -> Self {
        Self { x: self.x * a, y: self.y * a }
    }

    pub fn add(&self, a: Pos) -> Self {
        Self { x: self.x + a.x, y: self.y + a.y }
    }

    pub fn sub(&self, a: Pos) -> Self {
        Self { x: self.x - a.x, y: self.y - a.y }
    }

    pub fn dot(&self, a: Pos) -> f32 {
        self.x * a.x + self.y * a.y
    }

    pub fn len(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt().abs()
    }

    pub fn normalize(&self) -> Self {
        Self { x: self.x / self.len(), y: self.y / self.len() }
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use write! macro to write to the formatter
        write!(f, "{{{} {}}}", self.x, self.y)
    }
}

/*
 * Data struct to elegantly store dimension data
 */
#[derive(Copy, Clone)]
pub struct Dim {
    pub w: f32,
    pub h: f32,
}

impl Dim {
    pub fn new(w: f32, h: f32) -> Self {
        Self { w, h }
    }

    pub fn update(&mut self, w: f32, h: f32) -> () {
        self.w = w;
        self.h = h;
    }

    pub fn zero() -> Self {
        Self { w: 0.0, h: 0.0 }
    }
}

impl fmt::Display for Dim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use write! macro to write to the formatter
        write!(f, "{{{} {}}}", self.w, self.h)
    }
}