use application::commands::CreateTask;
use local_storage::TodoListLocalStorage;

use wasm_bindgen::prelude::*;

use crate::log;

#[wasm_bindgen(js_name = createTask)]
pub async fn add_task(description: String) {
    log(&format!("Adding task: {}", description));

    let create_task = CreateTask {
        todolist_store: Box::new(TodoListLocalStorage {}),
    };

    create_task.handle(description).await.unwrap();
    log("Task added");
}
