extern crate image;

use std::{fs, path::Path};
use std::f32::consts::PI;
use image::{ImageBuffer, Rgb, save_buffer, Luma};
use nalgebra::{Matrix4, Vector4};
use rand::{thread_rng, Rng}; // provides rng

use crate::{
    render::{
        GRAY,
        GREEN,
        as_rgb
    },
    Dim,
    Pos,
};

pub struct Track {
    map: ImageBuffer<Luma<u8>, Vec<u8>>,
    pub start_pos: Pos,
    pub start_orientation: f64,
}

pub fn generate_track (size: &Dim) -> Track {
    print!("\rGenerating track... ");

    const PRINT_DEBUG: bool = false;
    const ROAD_WIDTH: f32 = 40.0;
    const N_POINTS_RANGE: core::ops::Range<u32> = 20..30;
    const POS_NOISE_REL_DELTA: f32 = 0.15;
    const POINT_MIN_MERGE_DIST: f32 = 70.0;
    const POINT_MIN_MERGE_ANGLE: f32 = PI / 6.0;
    let pos_noise_delta = POS_NOISE_REL_DELTA * (size.w + size.h) / 2.0;

    let mut rng = thread_rng();
    let n_points: u32 = rng.gen_range(N_POINTS_RANGE);
    if PRINT_DEBUG {
        println!("n_points: {}", n_points);
    }

    // initial point generation
    let mut points: Vec<BezierPoint> = Vec::new();
    for i in 0..n_points {

        // generating points in a loop
        let t = i as f32 / n_points as f32;
        let mut pos =
            if t < 0.25 {
                let t = t * 4.0;
                Pos::new(size.w / 6.0 + (t * size.w * 2.0 / 3.0), size.h / 6.0)
            } else if t < 0.5 {
                let t = (t - 0.25) * 4.0;
                Pos::new(size.w * 5.0 / 6.0, size.h / 6.0 + (t * size.w * 2.0 / 3.0))
            } else if t < 0.75 {
                let t = (t - 0.5) * 4.0;
                Pos::new(size.w * 5.0 / 6.0 - (t * size.w * 2.0 / 3.0), size.h * 5.0 / 6.0)
            } else {
                let t = (t - 0.75) * 4.0;
                Pos::new(size.w / 6.0, size.h * 5.0 / 6.0 - (t * size.w * 2.0 / 3.0))
            };

        // adding noise
        let r1: f32 = rng.gen();
        let r2: f32 = rng.gen();
        pos = pos.add(Pos::new((r1 * 2.0 - 1.0) * pos_noise_delta, (r2 * 2.0 - 1.0) * pos_noise_delta));
        points.push(BezierPoint::new(pos, Pos::zero()));
        if PRINT_DEBUG {
            println!("New point {} {}", points.last().unwrap().pos, points.last().unwrap().tan);
        }
    }

    // merging points that are too close to each other or border
    let mut i = 0;
    let mut j = 1;
    while i < points.len() {
        // checking if points are too close to border
        if points[i].pos.x < ROAD_WIDTH || points[i].pos.x > size.w - ROAD_WIDTH || points[i].pos.y < ROAD_WIDTH || points[i].pos.y > size.h - ROAD_WIDTH {
            if PRINT_DEBUG {
                println!("Removing point {}, too close to border", i)
            }
            points.remove(i); // popping
            i = 0;
            j = 1;
            continue;
        }

        // calculating distance
        let dist = points[j].pos.sub(points[i].pos).len();
        if dist <= POINT_MIN_MERGE_DIST {
            if PRINT_DEBUG {
                println!("Removing point {}, collided with {} (dist: {:.2})", i, j, dist)
            }
            points.remove(i); // popping
            i = 0;
            j = 1;
            continue;
        }
        j += 1;
        if i == j {
            j += 1;
        }
        if j >= points.len() {
            i += 1;
            j = 0;
        }
    }

    // removing points with sharp angle
    let mut i = 0;
    while i < points.len() {
        let this = points[(i + 1) % points.len()].pos.sub(points[i].pos).normalize();
        let next = points[(i + 2) % points.len()].pos.sub(points[(i + 1) % points.len()].pos).normalize();
        let delta_theta = this.dot(next).acos();
        if PRINT_DEBUG {
            println!("{delta_theta}");
        }
        if delta_theta <= POINT_MIN_MERGE_ANGLE || delta_theta >= PI - POINT_MIN_MERGE_ANGLE {
            // popping
            if PRINT_DEBUG {
                println!("Removing point {}, creased with {} (delta theta: {:.2})", i, (i + 1) % points.len(), delta_theta)
            }
            points.remove(i);
            i = 0;
            continue;
        }
        i += 1;
    }

    // calculating tangents
    for i in 0..points.len() {
        // settings tangent parallel to vector from prev point to next
        points[i].tan = points[(i + 1) % points.len()].pos.sub(points[(i + points.len() - 1) % points.len()].pos).normalize();
    }

    if points.len() < 5 {
        // regenerate track if it sucks
        generate_track(size)
    } else {

        // preparing to create track map and pretty image
        let mut map = ImageBuffer::new(size.w as u32, size.h as u32);
        for p in map.pixels_mut() {
            *p = Luma([0]);
        }
        let mut img = ImageBuffer::new(size.w as u32, size.h as u32);
        for p in img.pixels_mut() {
            *p = Rgb(as_rgb(GREEN));
        }
        let render_matrix_decomp = Matrix4::new(
            0.0, 0.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            0.0, 0.0, 1.0, 0.0,
            3.0, 2.0, 1.0, 0.0,
        ).svd(true, true);

        // drawing the road
        for i in 0.. points.len() {
            // catmull-rom spline drawing
            let p1 = &points[i];
            let p2 = &points[(i + 1) % points.len()];
            let x = 200.0;
            let x_vec = Vector4::new(
                p1.pos.x, p2.pos.x, p1.tan.x * x, p2.tan.x * x,
            );
            let x_coeffs = render_matrix_decomp.solve(&x_vec, 0.0).expect("failed to solve matrix (x)");            let y_vec = Vector4::new(
                p1.pos.y, p2.pos.y, p1.tan.y * x, p2.tan.y * x,
            );
            let y_coeffs = render_matrix_decomp.solve(&y_vec, 0.0).expect("failed to solve matrix (y)");
            let mut t: f32 = 0.01;
            while t < 1.0 {
                let p = Pos::new(
                    x_coeffs[0] * t.powf(3.0) + x_coeffs[1] * t.powf(2.0) + x_coeffs[2] * t + x_coeffs[3],
                    y_coeffs[0] * t.powf(3.0) + y_coeffs[1] * t.powf(2.0) + y_coeffs[2] * t + y_coeffs[3],
                );
                t += 0.01;
                // drawing circles
                let d = ROAD_WIDTH as i32;
                let r = d >> 1;
                for y in 0..d {
                    for x in 0..d {
                        let rel_x = x - r;
                        let rel_y = y - r;
                        if rel_x.pow(2) + rel_y.pow(2) > r.pow(2) {
                            continue;
                        }
                        let pix_x = p.x as i32 + rel_x;
                        let pix_y = p.y as i32 + rel_y;
                        if pix_x < 0 || pix_x >= size.w as i32 || pix_y < 0 || pix_y >= size.h as i32 {
                            continue;
                        }
                        map.put_pixel(pix_x as u32, pix_y as u32, Luma([255]));
                        img.put_pixel(pix_x as u32, pix_y as u32, Rgb(as_rgb(GRAY)));
                    }
                }
                // putting pixel at start pos
                img.put_pixel(points[0].pos.x as u32, points[0].pos.y as u32, Rgb(as_rgb(GRAY)));
            }
        }

        // downloading track data
        if !Path::new("track_data/").exists() {
            fs::create_dir("track_data/").unwrap();
        }
        save_buffer("track_data/map.png", &map, map.width(), map.height(), image::ColorType::L8).expect("Failed to download track map");
        save_buffer("track_data/img.png", &img, img.width(), img.height(), image::ColorType::Rgb8).expect("Failed to download track map");

        let start_pos = points[0].pos.clone();
        let start_orientation = (-points[0].tan.y / points[0].tan.x).atan() as f64;
        println!("done");
        Track {
            map,
            start_pos,
            start_orientation,
        }
    }
}

struct BezierPoint {
    pos: Pos, // position
    tan: Pos, // tangent vector
}

impl BezierPoint {
    fn new(pos: Pos, tan: Pos) -> Self {
        BezierPoint { pos, tan }
    }
}

