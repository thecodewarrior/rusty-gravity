use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;
use crate::ui_glue::*;
use web_sys::{MouseEvent, KeyboardEvent};
use crate::draw::Renderer;
use crate::sim::Universe;

#[wasm_bindgen]
pub struct App {
    canvas: web_sys::HtmlCanvasElement,
    universe: Universe,
    renderer: Renderer,
    sim_running: bool,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen]
    pub fn new() -> App {
        let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();

        let canvas = document.get_element_by_id("universe").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let universe = Universe::new();
        let renderer = Renderer::new(canvas.clone());

        App {
            canvas,
            universe,
            renderer,
            sim_running: false
        }
    }

    #[wasm_bindgen]
    pub fn init(&self) {
        hook_event("#universe", "mousedown", "universe_mousedown");
        hook_event("body", "keydown", "body_keydown");
        request_animation_frame("frame");
    }

    #[wasm_bindgen]
    pub fn universe_mousedown(&mut self, event: MouseEvent) {
        log("down")
    }

    #[wasm_bindgen]
    pub fn body_keydown(&mut self, event: KeyboardEvent) {
        if event.key() == String::from(" ") {
            self.sim_running = !self.sim_running;
        }
    }

    #[wasm_bindgen]
    pub fn frame(&mut self) {
        if self.sim_running {
            self.universe.tick();
        }
        let data = self.universe.render_data();
        self.renderer.draw(&data);
        request_animation_frame("frame");
    }
}
