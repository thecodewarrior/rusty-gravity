#![feature(clamp)]

pub mod utils;
pub mod math;
pub mod sim;
pub mod draw;
pub mod ui;

use gloo::{events::EventListener, timers::callback::Timeout};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub struct Game {
    universe: sim::Universe,
    renderer: draw::Renderer,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen]
    pub fn new() -> Game {
        let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();

        let canvas = document.get_element_by_id("game-of-life-canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        Game {
            universe: sim::Universe::new(),
            renderer: draw::Renderer::new(canvas)
        }
    }

    #[wasm_bindgen]
    pub fn frame(&mut self) {
        self.universe.tick();
        let data = self.universe.render_data();
        self.renderer.draw(&data);
    }
}
