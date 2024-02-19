

use std::f64::consts::PI;

use crate::Pos;

pub struct Car {
    pub id: u32,
    t: u64,
    pub pos: Pos,
    vel: Pos,
    pub orientation: f64,
}

impl Car {

    const ACCELERATION: f32 = 2.5; // acceleration speed
    const MAX_SPEED: f32 = 4.0; // speed cap

    pub fn new (id: u32, start_pos: Pos, start_orientation: f64) -> Self {
        Car {
            id,
            t: 0,
            pos: start_pos,
            vel: Pos::zero(),
            orientation: start_orientation,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // accelerating
        self.orientation -= 0.003;
        self.orientation = (self.orientation + PI) % (2.0 * PI) - PI; // wrapping orientation from -pi/2 to pi/2

        let sign = if self.orientation >= PI / 2.0 { -1.0 } else { 1.0 };

        self.vel.x += Car::ACCELERATION * delta_time * sign;
        self.vel.y -= (self.orientation.tan() as f32) * Car::ACCELERATION * delta_time * sign;
        // capping car speed
        if self.vel.len() > Car::MAX_SPEED {
            self.vel = self.vel.normalize().mul(Car::MAX_SPEED)
        }
        if self.t == 2 {
            println!("car pos: {}", self.pos);
        }

        //self.pos = self.pos.add(self.vel);
        self.t += 1;
    }
}
