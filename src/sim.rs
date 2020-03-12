extern crate js_sys;

pub struct Body {
    x: f64,
    y: f64,
    mass: f64
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
}
