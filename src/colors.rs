
// colors to make it look pretty :D
pub const GREEN: [f32; 4] = [0.5, 0.72, 0.56, 1.0];
pub const RED: [f32; 4] = [0.95, 0.1, 0.2, 1.0];
pub const GRAY: [f32; 4] = [0.44, 0.4, 0.46, 1.0];
pub const BLACK: [f32; 4] = [0.0, 0.02, 0.02, 1.0];
pub const WHITE: [f32; 4] = [0.99, 1.0, 0.98, 1.0];

pub fn as_rgb(c: [f32; 4]) -> [u8; 3] {
    [
        (c[0] * 255.0) as u8,
        (c[1] * 255.0) as u8,
        (c[2] * 255.0) as u8,
    ]
}