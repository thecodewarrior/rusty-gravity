use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct BodyRenderData {
    pub x: f64,
    pub y: f64,
    pub radius: f64
}

pub struct RenderData {
    pub bodies: Vec<BodyRenderData>
}

pub struct Renderer {
    context: web_sys::CanvasRenderingContext2d
}

impl Renderer {
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Renderer {

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Renderer {
            context
        }
    }

    pub fn draw(&self, data: &RenderData) {
        self.context.clear_rect(0., 0., 800., 800.);

        self.context.begin_path();

        for body in &data.bodies {
            self.context
                .arc(body.x, body.y, body.radius, 0., f64::consts::PI * 2.)
                .unwrap();
            self.context.close_path();
        }

        self.context.fill();
    }
}
