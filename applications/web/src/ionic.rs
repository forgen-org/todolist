#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen]
extern "C" {
    /// HTMLIonModalElement
    #[wasm_bindgen (extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLIonModalElement , typescript_type = "HTMLIonModalElement")]
    pub type HTMLIonModalElement;

    #[wasm_bindgen (structural , method , js_class = "HTMLIonModalElement" , js_name = dismiss)]
    pub fn dismiss(this: &HTMLIonModalElement, value: JsValue, role: Option<&str>) -> bool;

    #[wasm_bindgen (structural , method , js_class = "HTMLIonModalElement" , js_name = present)]
    pub async fn present(this: &HTMLIonModalElement);
}

#[wasm_bindgen]
extern "C" {
    /// HTMLIonTextareaElement
    #[wasm_bindgen (extends = HtmlElement , extends = Element , extends = Node , extends = EventTarget , extends = :: js_sys :: Object , js_name = HTMLIonTextareaElement , typescript_type = "HTMLIonTextareaElement")]
    pub type HTMLIonTextareaElement;

    #[wasm_bindgen (structural , method , js_class = "HTMLIonTextareaElement" , js_name = setFocus)]
    pub async fn set_focus(this: &HTMLIonTextareaElement);

    #[wasm_bindgen (structural , method , js_class = "HTMLIonTextareaElement" , js_name = getInputElement)]
    pub async fn get_input_element(this: &HTMLIonTextareaElement) -> JsValue;
}
