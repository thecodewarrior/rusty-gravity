use wasm_bindgen::prelude::*;
use wasm_bindgen::convert::{WasmAbi, IntoWasmAbi};
use web_sys::EventTarget;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(message: &str);
    #[wasm_bindgen(js_namespace = UIGlue, js_name = hook_event)]
    fn hook_event_internal(target: &JsValue, event: &str, function_name: &str);
}

pub fn hook_event<T: AsRef<EventTarget>>(target: T, event: &str, function_name: &str) {
    hook_event_internal(target.as_ref().as_ref(), event, function_name);
}
