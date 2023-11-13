#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use web_sys::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLIonModalElement , typescript_type = "HTMLIonModalElement")]
    pub type HTMLIonModalElement;

    #[wasm_bindgen (structural , method , js_class = "HTMLIonModalElement" , js_name = dismiss)]
    pub fn dismiss(this: &HTMLIonModalElement, value: JsValue, role: Option<&str>) -> bool;
}
