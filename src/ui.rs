use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use crate::ui_glue::*;
use web_sys::{EventTarget, MouseEvent};

#[wasm_bindgen]
pub struct App {
    button: i32
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen]
    pub fn new() -> App {
        App {
            button: 1
        }
    }

    #[wasm_bindgen]
    pub fn init(&self) {
        let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
        hook_event(document.get_element_by_id("test").unwrap(), "mousedown", "button_pressed")
    }

    #[wasm_bindgen]
    pub fn button_pressed(&mut self, event: MouseEvent) {
        log("Pressed!")
    }
}

// fn add_event_listener<S, E, F>(id: &str, target: Rc<RefCell<S>>, callback: F)
//     where E: 'static + wasm_bindgen::convert::FromWasmAbi,
//           F: 'static + FnMut(&mut S, E) {
//     let target = target.clone();
//     let closure = Closure::wrap(Box::new(move |event: E| {
//         callback(&mut target.borrow_mut(), event)
//     }) as Box<dyn FnMut(_)>);
// }
