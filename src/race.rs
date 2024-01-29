mod car;
mod track;

use opengl_graphics::GlGraphics;
use piston::input::UpdateArgs;

use crate::{Pos, Dim};
use car::Car;
pub use track::Track;

pub struct Race {
    pub t: u64,
    pub n_cars: u32,
    pub cars: Vec<Car>,
}

impl Race {

    pub fn new (n_cars: u32, start_pos: Pos, start_orientation: f32) -> Race {
        println!("Initializing new race");
        let mut cars = Vec::new();
        for i in 0..n_cars {
            cars.push(Car::new(i, start_pos, start_orientation));
        }

        let new_race = Race { t: 0, n_cars, cars, };


        new_race
    }

    pub fn update(&mut self, update_args: &UpdateArgs) {
        for car in self.cars.iter_mut() {
            car.update(update_args);
        }
        self.t += 1;
        if self.t % 60 == 0 {
            //println!("t: {}", self.t);
        }
    }

}

pub use track::generate_track;