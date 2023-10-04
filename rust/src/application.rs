use crate::domain::{AddTask, TodoListStore};
use crate::infrastructure::in_memory_store::InMemoryStore;
use dto::*;
use wasm_bindgen::prelude::*;

mod adapters;
mod dto;

#[wasm_bindgen]
pub struct Application {
    #[wasm_bindgen(skip)]
    pub store: InMemoryStore,
}

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // console_error_panic_hook::set_once();

        Self {
            store: InMemoryStore::new(),
        }
    }

    pub async fn get_todolist(&self) -> TodoListDto {
        let todolist = TodoListStore::get(&self.store).await;

        todolist.into()
    }

    pub async fn add_task(&self, description: String) -> () {
        AddTask {
            todolist: &self.store,
        }
        .handle(description)
        .await;
    }
}
