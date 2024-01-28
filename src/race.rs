mod car;
mod track;

use opengl_graphics::GlGraphics;
use piston::input::UpdateArgs;

use crate::{Pos, Dim};
use car::Car;
pub use track::Track;

pub struct Race {
    t: u64,
    n_cars: u32,
    cars: Vec<Car>,
    size: Dim,
}

impl Race {

    pub fn new (n_cars: u32, size: &Dim, start_pos: Pos, start_orientation: f32) -> Race {
        println!("Generating new race");
        let mut cars = Vec::new();
        for i in 0..n_cars {
            cars.push(Car::new(i, start_pos, start_orientation));
        }

        let new_race = Race { t: 0, n_cars, cars, size: size.clone() };


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