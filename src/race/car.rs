
use piston::input::UpdateArgs;

use crate::Pos;

pub struct Car {
    pub id: u32,
    t: u64,
    pub pos: Pos,
    pub orientation: f32,
}

impl Car {

    pub fn new (id: u32, start_pos: Pos, start_orientation: f32) -> Self {
        Car {
            id,
            t: 0,
            pos: start_pos,
            orientation: start_orientation
        }
    }

    pub fn update(&mut self, update_args: &UpdateArgs) {
        self.t += 1;
    }
}