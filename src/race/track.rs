

use crate::Pos;

pub struct Track {
    curve: Pos,
    start_pos: Pos,
    start_orientation: f32,
}

impl Track {
    pub fn new(curve: Pos, start_pos: Pos, start_orientation: f32) -> Self {
        Self { curve, start_pos, start_orientation }
    }
}

pub fn generate_track (w: f32, h: f32) -> Track {
    println!("Generating track");
    Track::new(Pos::zero(), Pos::zero(), 90.0)
}