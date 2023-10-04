use crate::domain::{AddTask, TodoList, TodoListEvent, TodoListStore};
use crate::infrastructure::in_memory_store::InMemoryStore;
use async_trait::async_trait;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Application {
    #[wasm_bindgen(skip)]
    pub store: InMemoryStore,
}

#[wasm_bindgen]
impl Application {
    pub fn new() -> Self {
        Self {
            store: InMemoryStore::new(),
        }
    }

    pub async fn get_todolist(&self) -> JsValue {
        let todolist = TodoListStore::get(&self.store).await;

        serde_wasm_bindgen::to_value(&todolist).unwrap()
    }

    pub async fn add_task(&self) -> () {
        AddTask {
            todolist: &self.store,
        }
        .handle()
        .await
        .unwrap();
    }
}

#[async_trait]
impl TodoListStore for InMemoryStore {
    async fn get(&self) -> TodoList {
        let mut todolist = TodoList::default();

        match self.get() {
            Some(value) => {
                let events: Vec<TodoListEvent> = serde_json::from_value(value).unwrap_or_default();
                todolist.apply(events);
            }
            None => (),
        }

        todolist
    }

    async fn save(&self, events: Vec<TodoListEvent>) -> () {
        let value = serde_json::to_value(events).unwrap_or_default();
        self.save(value);
    }
}
