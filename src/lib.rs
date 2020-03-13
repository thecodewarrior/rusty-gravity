#![feature(clamp)]

pub mod utils;
pub mod math;
pub mod sim;
pub mod draw;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Game {
    universe: sim::Universe,
    renderer: draw::Renderer,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen]
    pub fn new() -> Game {
        Game {
            universe: sim::Universe::new(),
            renderer: draw::Renderer::new()
        }
    }

    #[wasm_bindgen]
    pub fn frame(&mut self) {
        self.universe.tick();
        let data = self.universe.render_data();
        self.renderer.draw(&data);
    }
}
