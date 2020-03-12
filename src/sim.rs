extern crate js_sys;
use wasm_bindgen::prelude::*;
use crate::draw;

#[derive(PartialEq, Copy, Clone)]
pub struct Body {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    mass: f64,
}

pub struct Universe {
    width: u32,
    height: u32,
    bodies: Vec<Body>,
}

impl Universe {
    pub fn new() -> Universe {
        let width = 800;
        let height = 800;

        let bodies = (0..20)
            .map(|i| {
                Body {
                    x: js_sys::Math::random() * (width as f64),
                    y: js_sys::Math::random() * (height as f64),
                    vx: (js_sys::Math::random() * 2. - 1.) * 0.005,
                    vy: (js_sys::Math::random() * 2. - 1.) * 0.005,
                    mass: js_sys::Math::random() * 10.0
                }
            })
            .collect();

        Universe {
            width,
            height,
            bodies,
        }
    }

    pub fn tick(&mut self) {
        for i in 0 .. self.bodies.len() {
            let mut body = self.bodies[i];
            for j in 0 .. self.bodies.len() {
                if i == j { continue; }
                let other = self.bodies[j];

                let dx = other.x - body.x;
                let dy = other.y - body.y;
                let r_sq = dx.powi(2) + dy.powi(2);
                let r = r_sq.sqrt();
                let f = (body.mass * other.mass)/r_sq;
                body.vx += dx / r * f / body.mass;
                body.vy += dy / r * f / body.mass;
            }
            self.bodies[i] = body;
        }

        for body in &mut self.bodies {
            body.x += body.vx;
            body.y += body.vy;
        }
    }

    pub fn render_data(&self) -> draw::RenderData {
        draw::RenderData {
            bodies: self.bodies.iter().map(|body| {
                draw::BodyRenderData {
                    x: body.x,
                    y: body.y,
                    radius: body.mass
                }
            }).collect()
        }
    }
}
