use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::window;

pub struct LocalStorage {
    key: String,
}

impl LocalStorage {
    async fn get_snapshot(&self) -> Result<TodoList, JsValue> {
        let window = window().ok_or_else(|| JsValue::from_str("No global `window` exists"))?;
        let local_storage = window
            .local_storage()
            .map_err(|_| JsValue::from_str("Cannot access Local Storage"))?
            .ok_or_else(|| JsValue::from_str("Local Storage is not available"))?;
        let item = local_storage
            .get_item("todo_list")
            .map_err(|_| JsValue::from_str("Cannot read from Local Storage"))?
            .unwrap_or_else(|| String::from("{}"));
        serde_json::from_str(&item).map_err(JsValue::from)
    }

    async fn push_events(&self, events: Vec<Event>) -> Result<(), JsValue> {
        let window = window().ok_or_else(|| JsValue::from_str("No global `window` exists"))?;
        let local_storage = window
            .local_storage()
            .map_err(|_| JsValue::from_str("Cannot access Local Storage"))?
            .ok_or_else(|| JsValue::from_str("Local Storage is not available"))?;
        let json = serde_json::to_string(&events).map_err(JsValue::from)?;
        local_storage
            .set_item("events", &json)
            .map_err(|_| JsValue::from_str("Cannot write to Local Storage"))
    }
}
