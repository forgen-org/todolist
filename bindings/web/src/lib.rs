mod home;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

extern crate console_error_panic_hook;
use std::panic;

#[wasm_bindgen(js_name = setPanicHook)]
pub fn panic_hook() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}
