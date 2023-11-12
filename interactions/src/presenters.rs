use std::rc::Rc;

use crate::{commands::create_task, services::TodoListStore};

pub struct CreateTaskForm {
    pub runtime: Rc<dyn TodoListStore>,
}

impl CreateTaskForm {
    pub async fn onsubmit(&self, description: String) {
        create_task(self.runtime.clone(), description)
            .await
            .unwrap();
    }

    pub fn title(&self) -> String {
        "Create Task".to_string()
    }
}
