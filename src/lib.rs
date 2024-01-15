mod race;

use graphics::Transformed;
use piston::{RenderArgs, UpdateArgs};
use std::fmt;
use opengl_graphics::GlGraphics;

use race::Race;

pub struct Simulation {
    gl: GlGraphics, // OpenGL drawing backend.
    race: Race,
    size: Dim,
}

impl Simulation {
    pub fn new(gl: GlGraphics, n_cars: u32, size: &Dim) -> Self {
        println!("Initializing simulation");
        let size = size.clone();

        use race::generate_track;
        let track = generate_track(&size);

        let race = Race::new(n_cars, &size, track.start_pos, track.start_orientation);
        Simulation { gl, race, size, }
    }

    pub fn render(&mut self, render_args: &RenderArgs, render_context: &RenderContext) {
        // drawing background
        /*self.gl.draw(render_args.viewport(), |c, gl| {
            graphics::clear(WHITE, gl);
        });*/
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
    pub scale: f64,
}

impl RenderContext {
    pub fn new () -> Self {
        Self {
            pos: Pos::zero(),
            scale: 1.0,
        }
    }

    pub fn apply_transformation (&self, transform: [[f64; 3]; 2]) -> [[f64; 3]; 2] {
        transform.trans(self.pos.x, self.pos.y).scale(self.scale, self.scale)
    }
}

/*
 * Data struct to elegantly store position data
 */
#[derive(Copy, Clone)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

impl Pos {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn update(&mut self, x: f64, y: f64) -> () {
        self.x = x;
        self.y = y;
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
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
    pub w: f64,
    pub h: f64,
}

impl Dim {
    pub fn new(w: f64, h: f64) -> Self {
        Self { w, h }
    }

    pub fn update(&mut self, w: f64, h: f64) -> () {
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