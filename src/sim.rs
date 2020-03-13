extern crate js_sys;
use wasm_bindgen::prelude::*;
use crate::math::{Vec2, vec2};
use crate::draw;
use std::f64::consts::PI;
use core::iter;
use crate::utils::IndicesHelper;

const G: f64 = 1.;
const DENSITY: f64 = 1.;

#[derive(PartialEq, Copy, Clone)]
pub struct Body {
    position: Vec2,
    velocity: Vec2,
    mass: f64,
}

impl Body {
    pub fn new() -> Body {
        Body {
            position: vec2(0, 0),
            velocity: vec2(0, 0),
            mass: 0.0
        }
    }
}

pub struct Universe {
    size: Vec2,
    bodies: Vec<Option<Body>>,
}

impl Universe {
    pub fn new() -> Universe {
        let size = vec2(800, 800);
        let center = size / 2;

        let central_mass = 5000.;
        let mut bodies: Vec<Body> = iter::once(Body {
            position: center,
            velocity: vec2(0, 0),
            mass: central_mass
        }).chain(
            (0..)
                .map(|i| {
                    Body {
                        position: vec2(
                            js_sys::Math::random(),
                            js_sys::Math::random(),
                        ) * size,
                        velocity: vec2(0, 0),
                        mass: 20. + js_sys::Math::random() * 10.0
                    }
                })
                .filter(|body| {
                    ((body.position - center) / (size/2)).length_sq() <= 1.
                })
                .take(10)
        )
            .collect();

        for i in bodies.indices() {
            let delta = bodies[i].position - center;
            if delta == vec2(0, 0) {
                continue;
            }
            let distance = delta.length();
            let distance_sq = delta.length_sq();

            let mut lower_mass = central_mass;
            // for j in bodies.indices() {
            //     if j == i {
            //         continue;
            //     }
            //     if (bodies[j].position - center).length_sq() <= distance_sq {
            //         lower_mass += bodies[j].mass;
            //     }
            // }

            let speed = orbital_velocity(lower_mass, distance);
            let velocity = delta / distance * speed;
            bodies[i].velocity = vec2(velocity.y, -velocity.x);
        }


        Universe {
            size,
            bodies: bodies.into_iter().map(|it| Some(it)).collect(),
        }
    }

    pub fn tick(&mut self) {
        // The instructions to merge bodies. If a body's index doesn't contain its own index, it is
        // going to merge _into_ body pointed to. These mergers can be chained, so you can have
        // a body merge with a body that's merging with another body, in which case the pointer
        // chain is followed. Due to directionality of the algorithm, bodies should always merge
        // into bodies with lower indices. To prevent loops, an upward merger is an error.
        let mut mergers: Vec<usize> = (0 .. self.bodies.len())
            .map(|i| i)
            .collect();

        for i in 0 .. self.bodies.len(){
            let body = self.bodies[i];
            if body.is_none() {
                continue;
            }
            let mut body = body.unwrap();
            for j in 0 .. self.bodies.len() {
                if i == j { continue; }
                let other = self.bodies[j];
                if other.is_none() {
                    continue;
                }
                let other = other.unwrap();

                let delta = other.position - body.position;
                let distance_sq = delta.length_sq();

                if distance_sq < (radius_for(body.mass) + radius_for(other.mass)).powi(2) {
                    if j > i {
                        mergers[j] = mergers[i];
                    }
                } else {
                    let f = G * (body.mass * other.mass) / distance_sq;
                    body.velocity += delta.normalized() * (f / body.mass);
                }
            }
            self.bodies[i] = Some(body);
        }

        // Go in reverse order, since body pointers always point to lower indices, any chained
        // pointers will accumulate on their way to the uppermost body. This system will likely need
        // to be reworked.
        for i in (0 .. self.bodies.len()).rev() {
            let dest = mergers[i];
            if dest != i {
                let source = self.bodies[i]
                    .expect("Tried to merge None into another body");
                let target = self.bodies[dest]
                    .expect("Tried to merge a body into None");
                let mut body = Body::new();
                body.mass = target.mass + source.mass;
                body.position = (target.position * target.mass + source.position * source.mass) / body.mass;
                body.velocity = (target.velocity * target.mass + source.velocity * source.mass) / body.mass;

                self.bodies[i] = None;
                self.bodies[dest] = Some(body);
            }
        }

        for i in 0 .. self.bodies.len() {
            let body = self.bodies[i];
            if body.is_none() {
                continue;
            }
            let mut body = body.unwrap();

            body.position += body.velocity;
            if body.position.x < 0. || body.position.x > self.size.x as f64 {
                body.velocity.x *= -0.5;
                body.position.x = body.position.x.clamp(0., self.size.x as f64)
            }
            if body.position.y < 0. || body.position.y > self.size.y as f64 {
                body.velocity.y *= -0.5;
                body.position.y = body.position.y.clamp(0., self.size.y as f64)
            }

            self.bodies[i] = Some(body);
        }

    }

    pub fn render_data(&self) -> draw::RenderData {
        draw::RenderData {
            bodies: self.bodies.iter().filter_map(|body| {
                body.map(|body| {
                    draw::BodyRenderData {
                        x: body.position.x,
                        y: body.position.y,
                        radius: radius_for(body.mass)
                    }
                })
            }).collect()
        }
    }
}

fn radius_for(mass: f64) -> f64 {
    (mass / DENSITY / PI).sqrt()
}

fn orbital_velocity(mass: f64, distance: f64) -> f64 {
    (G * mass / distance).sqrt()
}
