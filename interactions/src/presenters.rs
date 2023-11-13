use crate::{commands::create_task, services::Store};

#[derive(Clone)]
pub struct CreateTaskForm<R>
where
    R: Clone,
{
    pub runtime: R,
}

impl<R> CreateTaskForm<R>
where
    R: Store<todolist::Event> + Clone,
{
    pub async fn onsubmit(&self, description: String) {
        create_task(&self.runtime, description).await.unwrap();
    }

    pub fn title(&self) -> String {
        "Create Task".to_string()
    }
}
