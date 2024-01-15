

use std::f32::consts::PI;
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use rand::{thread_rng, Rng, rngs::ThreadRng};

use crate::{Pos, Dim, RenderContext, GRAY, BLACK, RED};


pub struct Track {
    points: Vec<BezierPoint>,
    pub start_pos: Pos,
    pub start_orientation: f32,
}

impl Track {
    /*pub fn new(curve: Pos, start_pos: Pos, start_orientation: f32) -> Self {
        Self { vec![1; 0], start_pos, start_orientation }
    }*/

    pub fn render(&self, gl: &mut GlGraphics, render_args: &RenderArgs, render_context: &RenderContext) {
        // drawing curve
        for i in 0..self.points.len() {
            let mut t: f32 = 0.0;

            let this = &self.points[i];
            let next = &self.points[(i + 1) % self.points.len()];

            // bezier stuff
            let p1 = this.pos;
            let p2 = this.pos.add(this.handle);
            let p3 = next.pos.sub(next.handle);
            let p4 = next.pos;

            while t < 1.0 {
                let p = p1.mul((1.0 - t).powf(3.0)).add(p2.mul(3.0 * (1.0-t).powf(2.0) * t)).add(p3.mul(3.0 * (1.0-t) * t.powf(2.0))).add(p4.mul(t.powf(3.0)));
                gl.draw(render_args.viewport(), |c,gl| {
                    let point: graphics::types::Rectangle = [(p.x - 10.0) as f64, (p.y - 10.0) as f64, 20.0, 20.0];
                    let transform = render_context.apply_transformation(c.transform);

                    graphics::ellipse(*GRAY, point, transform, gl)
                });
                t += 0.01;
            }
            // drawing control lines
            gl.draw(render_args.viewport(), |c,gl| {
                let point: graphics::types::Line = [p1.x as f64, p1.y as f64, p2.x as f64, p2.y as f64];
                let transform = render_context.apply_transformation(c.transform);

                graphics::line(*BLACK, 0.5, point, transform, gl)
            });
            gl.draw(render_args.viewport(), |c,gl| {
                let point: graphics::types::Line = [p2.x as f64, p2.y as f64, p3.x as f64, p3.y as f64];
                let transform = render_context.apply_transformation(c.transform);

                graphics::line(*BLACK, 0.5, point, transform, gl)
            });
            gl.draw(render_args.viewport(), |c,gl| {
                let point: graphics::types::Line = [p4.x as f64, p4.y as f64, p3.x as f64, p3.y as f64];
                let transform = render_context.apply_transformation(c.transform);

                graphics::line(*BLACK, 0.5, point, transform, gl)
            });
        }

        for i in 0..self.points.len() {
            let p = &self.points[i];

            // drawing points
            /*gl.draw(render_args.viewport(), |c,gl| {
                let point: graphics::types::Rectangle = [(p.pos.x - 5.0) as f64, (p.pos.y - 5.0) as f64, 10.0, 10.0];
                let transform = render_context.apply_transformation(c.transform);

                graphics::ellipse(*BLACK, point, transform, gl)
            });

            //drawing connecting lines
            let next = &self.points[(i + 1) % self.points.len()];
            gl.draw(render_args.viewport(), |c,gl| {
                let point: graphics::types::Line = [p.pos.x as f64, p.pos.y as f64, next.pos.x as f64, next.pos.y as f64];
                let transform = render_context.apply_transformation(c.transform);

                graphics::line(*RED, 1.0, point, transform, gl)
            });

            // drawing handle points
            gl.draw(render_args.viewport(), |c,gl| {
                let point: graphics::types::Rectangle = [(p.pos.x + p.handle.x - 2.0) as f64, (p.pos.y + p.handle.y - 2.0) as f64, 4.0, 4.0];
                let transform = render_context.apply_transformation(c.transform);

                graphics::ellipse(*BLACK, point, transform, gl)
            });
            gl.draw(render_args.viewport(), |c,gl| {
                let point: graphics::types::Rectangle = [(p.pos.x - p.handle.x - 2.0) as f64, (p.pos.y - p.handle.y - 2.0) as f64, 4.0, 4.0];
                let transform = render_context.apply_transformation(c.transform);

                graphics::ellipse(*BLACK, point, transform, gl)
            });*/
        }
    }
}

pub fn generate_track (size: &Dim) -> Track {
    println!("Generating track");

    // TODO remove generating track output
    const PRINT_DEBUG: bool = true;
    const N_POINTS_RANGE: core::ops::Range<u32> = 30..40;
    const POS_NOISE_DELTA: f32 = 100.0;
    const POINT_MIN_MERGE_DIST: f32 = 80.0;
    const POINT_MIN_MERGE_ANGLE: f32 = PI / 3.0; // value in radians * PI
    const HANDLE_SMOOTH_FACTOR: f32 = 0.2;

    let mut rng = thread_rng();
    let n_points: u32 = rng.gen_range(N_POINTS_RANGE);
    println!("n_points: {}", n_points);

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
        points.push(BezierPoint {
            pos,
            handle: Pos::zero(),
        });
        if PRINT_DEBUG {
            println!("New point {} {}", points.last().unwrap().pos, points.last().unwrap().handle);
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
    println!("{POINT_MIN_MERGE_ANGLE}");
    // removing points with sharp angle
    let mut i = 0;
    while i < points.len() {
        let this = points[i].pos;
        let next = this.sub(points[(i + 1) % points.len()].pos).mul(-1.0);
        let delta_theta = (this.dot(next) / (this.len() *  next.len())).acos();
        println!("{i}: {delta_theta}");
        if delta_theta <= POINT_MIN_MERGE_ANGLE {
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

    // generating handle points
    let mut handles: Vec<Pos> = Vec::new();
    let mut sep_force_vec: Vec<Pos> = Vec::new();
    for i in 0..points.len() {
        // setting handle to midpoint
        handles.push(points[i].pos.add(points[(i + 1) % points.len()].pos).mul(0.5));

        sep_force_vec.push(Pos::zero());
    }
    // calculating seperation forces
    let sep_fn = |dist: f32| ((-dist)/6.0 + 50.0).max(0.0);
    for i in 0..points.len() {
        for j in 0..points.len() {
            if i == j { continue; }
            let force = sep_fn(handles[i].sub(handles[j]).len());
            sep_force_vec[i] = sep_force_vec[i].add(handles[i].sub(handles[j]).normalize().mul(force));
        }
    }
    // applying seperation forces
    for i in 0..points.len() {
        // gramm-schmidt
        let edge_vec = points[(i + 1) % points.len()].pos.sub(points[i].pos);
        let force_vec = sep_force_vec[i].sub(edge_vec.mul(edge_vec.dot(sep_force_vec[i]) / edge_vec.dot(edge_vec)));
        if PRINT_DEBUG {
            println!("Handle sep force for i={}: {}", i, force_vec)
        }
        handles[i] = handles[i].add(force_vec);
        points[i].handle = handles[i].sub(points[i].pos).mul(0.5);
    }

    // smoothing out control points
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        assert!(HANDLE_SMOOTH_FACTOR <= 1.0 && HANDLE_SMOOTH_FACTOR >= 0.0, "HANDLE_SMOOTH_FACTOR must be between 0 and 1");
        handles[j] = handles[j].sub(points[j].pos).mul(-1.0).add(points[j].pos).mul(HANDLE_SMOOTH_FACTOR).add(handles[i].mul(1.0 - HANDLE_SMOOTH_FACTOR)).sub(points[j].pos).mul(-1.0).add(points[j].pos);
    }

    for i in 0..points.len() {
        points[i].handle = handles[i].sub(points[i].pos).mul(0.5);
    }

    /*for i in 0..points.len() {
        let this = &points[i];
        let next = &points[(i + 1) % points.len()];

        // bezier stuff
        let p1 = this.pos;
        let p2 = this.pos.add(this.handle);
        let p3 = next.pos.add(next.handle.mul(-1.0));
        let p4 = next.pos;
        println!("----------\np1: {}\np2: {}\np3: {}\np4: {}", p1, p2, p3, p4)
    }*/

    Track {
        points,
        start_pos: Pos::new(100.0, 102.0),
        start_orientation: 0.0,
    }
}

struct BezierPoint {
    pos: Pos,
    handle: Pos, // relative position of handle
}