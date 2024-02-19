mod car;
mod track;

use piston::input::UpdateArgs;

use crate::Pos;
use car::Car;
pub use track::Track;

pub struct Race {
    pub t: u64,
    pub n_cars: u32,
    pub cars: Vec<Car>,
}

impl Race {

    pub fn new (n_cars: u32, start_pos: Pos, start_orientation: f64) -> Race {
        println!("Initializing new race (start orientation: {start_orientation})");
        let mut cars = Vec::new();
        for i in 0..n_cars {
            cars.push(Car::new(i, start_pos, start_orientation));
        }
        Race {
            t: 0,
            n_cars,
            cars,
        }
    }

    pub fn update(&mut self, update_args: &UpdateArgs) {
        let delta_time = update_args.dt as f32;
        for car in self.cars.iter_mut() {
            car.update(delta_time);
        }
        self.t += 1;
    }

}

pub use track::generate_track;
