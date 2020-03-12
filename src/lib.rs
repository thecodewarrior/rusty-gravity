mod utils;
extern crate js_sys;

use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str, nop: i32) {
    alert(format!("Hello, {}!", name).as_str());
}

#[wasm_bindgen]
pub struct Body {
    x: f64,
    y: f64,
    mass: f64
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    bodies: Vec<Body>,
}

impl Universe {
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 800;
        let height = 800;

        let bodies = (0..10)
            .map(|i| {
                Body {
                    x: js_sys::Math::random() * (width as f64),
                    y: js_sys::Math::random() * (height as f64),
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
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn bodies_count(&self) -> usize {
        self.bodies.len()
    }

    pub fn bodies(&self) -> *const Body {
        self.bodies.as_ptr()
    }
}
