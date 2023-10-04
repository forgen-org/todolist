use wasm_bindgen::prelude::*;

use crate::domain::TodoList;

#[wasm_bindgen]
pub struct TodoListDto {
    tasks: Vec<TaskDto>,
}

#[wasm_bindgen]
impl TodoListDto {
    #[wasm_bindgen(getter)]
    pub fn tasks(&self) -> Vec<TaskDto> {
        self.tasks.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct TaskDto {
    description: String,
    pub done: bool,
}

#[wasm_bindgen]
impl TaskDto {
    #[wasm_bindgen(getter)]
    pub fn description(&self) -> String {
        self.description.clone()
    }
}

impl From<TodoList> for TodoListDto {
    fn from(todolist: TodoList) -> Self {
        Self {
            tasks: todolist
                .tasks
                .into_iter()
                .map(|task| TaskDto {
                    description: task.description,
                    done: task.done,
                })
                .collect(),
        }
    }
}
