extern crate image;

use std::{fs, path};
use std::f32::consts::PI;
use image::{ImageBuffer, Rgb, save_buffer, Rgba, Luma};
use nalgebra::{Matrix4, Vector4, SVD, Const};
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use rand::{thread_rng, Rng};

use crate::{colors::*, Pos, Dim, RenderContext};

pub struct Track {
    points: Vec<BezierPoint>,
    pub map: ImageBuffer<Luma<u8>, Vec<u8>>,
    pub img: ImageBuffer<Rgb<u8>, Vec<u8>>,
    pub start_pos: Pos,
    pub start_orientation: f32,
    render_matrix_decomp: SVD<f32, Const<4>, Const<4>>,
}

impl Track {
    /*pub fn new(curve: Pos, start_pos: Pos, start_orientation: f32) -> Self {
        Self { vec![1; 0], start_pos, start_orientation }
    }*/

    pub fn render(&self, gl: &mut GlGraphics, render_args: &RenderArgs, render_context: &RenderContext) {
        // drawing road
        for i in 0.. self.points.len() {
            // catmull-rom spline drawing
            let p1 = &self.points[i];
            let p2 = &self.points[(i + 1) % self.points.len()];
            let x = 200.0;
            let x_vec = Vector4::new(
                p1.pos.x, p2.pos.x, p1.tan.x * x, p2.tan.x * x,
            );
            let x_eq = self.render_matrix_decomp.solve(&x_vec, 0.0).expect("failed to solve matrix (x)");
            let y_vec = Vector4::new(
                p1.pos.y, p2.pos.y, p1.tan.y * x, p2.tan.y * x,
            );
            let y_eq = self.render_matrix_decomp.solve(&y_vec, 0.0).expect("failed to solve matrix (y)");
            let mut t: f32 = 0.01;
            while t < 1.0 {
                let p = Pos::new(
                    x_eq[0] * t.powf(3.0) + x_eq[1] * t.powf(2.0) + x_eq[2] * t + x_eq[3],
                    y_eq[0] * t.powf(3.0) + y_eq[1] * t.powf(2.0) + y_eq[2] * t + y_eq[3],
                );
                gl.draw(render_args.viewport(), |c,gl| {
                    let point: graphics::types::Rectangle = [(p.x - 20.0) as f64, (p.y - 20.0) as f64, 40.0, 40.0];
                    let transform = render_context.apply_transformation(c.transform);

                    graphics::ellipse(GRAY, point, transform, gl)
                });
                t += 0.01
            }
        }
        /*for i in 0..self.points.len() {
            let p = &self.points[i];

            // drawing points
            gl.draw(render_args.viewport(), |c,gl| {
                let point: graphics::types::Rectangle = [(p.pos.x - 5.0) as f64, (p.pos.y - 5.0) as f64, 10.0, 10.0];
                let transform = render_context.apply_transformation(c.transform);

                graphics::ellipse(BLACK, point, transform, gl)
            });

            //drawing connecting lines
            let next = &self.points[(i + 1) % self.points.len()];
            gl.draw(render_args.viewport(), |c,gl| {
                let point: graphics::types::Line = [p.pos.x as f64, p.pos.y as f64, next.pos.x as f64, next.pos.y as f64];
                let transform = render_context.apply_transformation(c.transform);

                graphics::line(RED, 0.5, point, transform, gl)
            });

            // drawing tangents
            gl.draw(render_args.viewport(), |c,gl| {
                let point: graphics::types::Rectangle = [p.pos.x as f64, p.pos.y as f64, (p.pos.x + p.tan.mul(25.0).x) as f64, (p.pos.y + p.tan.mul(25.0).y) as f64];
                let transform = render_context.apply_transformation(c.transform);

                graphics::line(BLACK, 1.0, point, transform, gl)
            });
        }*/
    }
}

pub fn generate_track (size: &Dim) -> Track {
    println!("Generating track");

    // TODO remove generating track output
    const PRINT_DEBUG: bool = true;
    const N_POINTS_RANGE: core::ops::Range<u32> = 20..30;
    const POS_NOISE_REL_DELTA: f32 = 0.15;
    const POINT_MIN_MERGE_DIST: f32 = 70.0;
    const POINT_MIN_MERGE_ANGLE: f32 = PI / 6.0;
    let POS_NOISE_DELTA = POS_NOISE_REL_DELTA * (size.w + size.h) / 2.0;

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
        pos = pos.add(Pos::new((r1 * 2.0 - 1.0) * POS_NOISE_DELTA, (r2 * 2.0 - 1.0) * POS_NOISE_DELTA));
        points.push(BezierPoint::new(pos, Pos::zero()));
        if PRINT_DEBUG {
            println!("New point {} {}", points.last().unwrap().pos, points.last().unwrap().tan);
        }
    }

    // merging points that are too close
    let mut i = 0;
    let mut j = 1;
    while i < points.len() {

        // calculating distance
        let dist = points[j].pos.sub(points[i].pos).len();
        if dist <= POINT_MIN_MERGE_DIST {
            // popping
            if PRINT_DEBUG {
                println!("Removing point {}, collided with {} (dist: {:.2})", i, j, dist)
            }
            points.remove(i);
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
        println!("{delta_theta}");
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

    // creating track map
    let mut map = ImageBuffer::new(size.w as u32, size.h as u32);
    for p in map.pixels_mut() {
        *p = Luma([0]);
    }

    // saving track map
    if !path::Path::new("track_data/").exists() {
        fs::create_dir("track_data/").unwrap();
    }
    save_buffer("track_data/map.png", &map, map.width(), map.height(), image::ColorType::L8).expect("Failed to download track map");

    // creating pretty track image
    let mut img = ImageBuffer::new(size.w as u32, size.h as u32);
    for p in img.pixels_mut() {
        *p = Rgb(as_rgb(GREEN));
    }
    save_buffer("track_data/img.png", &img, img.width(), img.height(), image::ColorType::Rgb8).expect("Failed to download track map");


    if points.len() > 4 {
        Track {
            points,
            map,
            img,
            start_pos: Pos::new(100.0, 100.0),
            start_orientation: 0.0,
            render_matrix_decomp: Matrix4::new(
                    0.0, 0.0, 0.0, 1.0,
                    1.0, 1.0, 1.0, 1.0,
                    0.0, 0.0, 1.0, 0.0,
                    3.0, 2.0, 1.0, 0.0,
                ).svd(true, true),

        }
    } else {
        generate_track(size)
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