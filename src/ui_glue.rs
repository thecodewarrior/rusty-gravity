use wasm_bindgen::prelude::*;
use wasm_bindgen::convert::{WasmAbi, IntoWasmAbi};
use web_sys::EventTarget;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(message: &str);

    #[wasm_bindgen(js_namespace = UIGlue, js_name = hook_event)]
    fn hook_event_internal(target: &JsValue, event: &str, _fn: &str);

    #[wasm_bindgen(js_namespace = UIGlue)]
    pub fn set_interval(_fn: &str, interval: u32) -> u32;
    #[wasm_bindgen(js_namespace = UIGlue)]
    pub fn clear_interval(id: u32);

    #[wasm_bindgen(js_namespace = UIGlue)]
    pub fn set_timeout(_fn: &str, delay: u32) -> u32;
    #[wasm_bindgen(js_namespace = UIGlue)]
    pub fn clear_timeout(id: u32);

    #[wasm_bindgen(js_namespace = UIGlue)]
    pub fn request_animation_frame(_fn: &str);
}

pub fn hook_event<T: AsRef<EventTarget>>(target: T, event: &str, _fn: &str) {
    hook_event_internal(target.as_ref().as_ref(), event, _fn);
}

pub fn hook_event_id(id: &str, event: &str, _fn: &str) {
    let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
    hook_event(document.get_element_by_id(id).unwrap(), event, _fn);
}
