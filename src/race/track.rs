

use crate::{Pos, Dim};

pub struct Track {
    curve: Pos,
    pub start_pos: Pos,
    pub start_orientation: f32,
}

impl Track {
    pub fn new(curve: Pos, start_pos: Pos, start_orientation: f32) -> Self {
        Self { curve, start_pos, start_orientation }
    }
}

pub fn generate_track (size: &Dim) -> Track {
    println!("Generating track");
    Track::new(Pos::zero(), Pos::new(100.0, 102.0), 0.0)
}