use application::{commands::CreateTask, queries::ListTasks};
use local_storage::TodoListLocalStorage;

use wasm_bindgen::prelude::*;

use crate::log;

#[wasm_bindgen]
pub struct Home {
    todolist_store: TodoListLocalStorage,
}

#[wasm_bindgen]
impl Home {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let todolist_store = TodoListLocalStorage {};

        Self { todolist_store }
    }

    #[wasm_bindgen(js_name = listTasks)]
    pub async fn list_tasks(&self) -> TaskList {
        let tasks = ListTasks(&self.todolist_store).handle().await;

        TaskList {
            tasks: tasks
                .into_iter()
                .map(|task| Task {
                    description: task.description,
                    done: task.done,
                })
                .collect(),
        }
    }

    #[wasm_bindgen(js_name = addTask)]
    pub async fn add_task(&self, description: String) -> () {
        log(&format!("Adding task: {}", description));
        CreateTask(&self.todolist_store).handle(description).await;
        log("Task added");
    }
}

#[wasm_bindgen]
pub struct TaskList {
    tasks: Vec<Task>,
}

#[wasm_bindgen]
impl TaskList {
    #[wasm_bindgen(getter)]
    pub fn tasks(&self) -> Vec<Task> {
        self.tasks.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Task {
    description: String,
    pub done: bool,
}

#[wasm_bindgen]
impl Task {
    #[wasm_bindgen(getter)]
    pub fn description(&self) -> String {
        self.description.clone()
    }
}
