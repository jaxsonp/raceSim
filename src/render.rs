

use opengl_graphics::{GlGraphics, Texture};
use graphics::{
    rectangle::{
        Border, Rectangle, Shape
    }, DrawState, Image, ImageSize, Transformed};
use piston::RenderArgs;

use crate::{race::Race, Pos};

// colors to make it look pretty :D
pub const GREEN: [f32; 4] = [0.5, 0.72, 0.56, 1.0];
pub const RED: [f32; 4] = [0.95, 0.1, 0.2, 1.0];
pub const GRAY: [f32; 4] = [0.44, 0.4, 0.46, 1.0];
pub const BLACK: [f32; 4] = [0.0, 0.02, 0.02, 1.0];
pub const WHITE: [f32; 4] = [0.99, 1.0, 0.98, 1.0];

// other rendering args
const CAR_LENGTH: f64 = 30.0;
const CAR_WIDTH: f64 = CAR_LENGTH / 2.0;

/*
 * Helper function to convert float/RGBA colors into int/RGB colors
 */
pub fn as_rgb(c: [f32; 4]) -> [u8; 3] {
    [
        (c[0] * 255.0) as u8,
        (c[1] * 255.0) as u8,
        (c[2] * 255.0) as u8,
    ]
}

pub struct Renderer {
    gl: GlGraphics,
    pos: Pos,
    scale: f32,
    render_args: Option<RenderArgs>,
    draw_state: DrawState,
}

impl Renderer {
    pub fn new(gl: GlGraphics) -> Self {
        let draw_state = DrawState {
            scissor: None,
            stencil: None,
            blend: None,
        };
        Self {
            gl,
            pos: Pos::zero(),
            scale: 1.0,
            render_args: None,
            draw_state,
        }
    }

    pub fn draw_track(&mut self, img: &Texture) -> () {
        if !self.render_args.is_some() {
            return;
        }
        let viewport = self.render_args.unwrap().viewport();

        // drawing the background
        self.gl.draw(viewport, |c, gl| {
            Rectangle {
                color: GREEN,
                shape: Shape::Square,
                border: None,
            }.draw([0.0, 0.0, viewport.draw_size[0] as f64, viewport.draw_size[1] as f64], &self.draw_state, c.transform, gl)
        });
        // drawing track image
        self.gl.draw(viewport, |c, gl| {
            let transform = c.transform
                .trans(self.pos.x as f64, self.pos.y as f64).scale(self.scale as f64, self.scale as f64); // applying render context transform

            Image::new().rect([ 0.0, 0.0, img.get_size().0 as f64, img.get_size().1 as f64 ])
                .draw(img, &self.draw_state, transform, gl)
        });
    }

    pub fn draw_cars(&mut self, race: &Race) {
        for car in race.cars.iter() {
            self.draw_car(car.pos, car.orientation);
        }
    }

    pub fn draw_car(&mut self, pos: Pos, orientation: f32) -> () {
        if !self.render_args.is_some() {
            return;
        }
        self.gl.draw(self.render_args.unwrap().viewport(), |c, gl| {
            let transform = c.transform
                .trans(self.pos.x as f64, self.pos.y as f64).scale(self.scale as f64, self.scale as f64) // applying render context transform
                .trans(pos.x as f64, pos.y as f64)
                .rot_rad(orientation as f64);
            Rectangle {
                color: RED,
                shape: Shape::Round(CAR_WIDTH / 6.0, 3),
                border: Some(Border {
                    color: BLACK,
                    radius: 1.0,
                }),
            }.draw([
                    (CAR_WIDTH / 2.0),
                    (-CAR_LENGTH / 2.0),
                    CAR_WIDTH,
                    CAR_LENGTH
                ],
                &self.draw_state, transform, gl)
        });
    }

    pub fn pan_view(&mut self, x_off: f64, y_off: f64) -> () {
        self.pos.x += x_off as f32;
        self.pos.y += y_off as f32;
    }

    pub fn zoom_view(&mut self, zoom_amt: f32, mouse_pos: Pos) -> () {
        self.pos.x = self.pos.x * zoom_amt + mouse_pos.x * (1.0 - zoom_amt);
        self.pos.y = self.pos.y * zoom_amt + mouse_pos.y * (1.0 - zoom_amt);
        self.scale *= zoom_amt;
    }

    pub fn set_render_args(&mut self, new_args: RenderArgs) -> () {
        self.render_args = Some(new_args);
    }
}