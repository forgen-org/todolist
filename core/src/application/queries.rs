use super::services::TodoListStore;
use crate::domain::todolist::Task;

pub struct ListTasks<'a, S: TodoListStore>(pub &'a S);

impl<'a, S: TodoListStore> ListTasks<'a, S> {
    pub async fn handle(&self) -> Vec<Task> {
        let todolist = TodoListStore::get_snapshot(self.0).await;

        todolist.tasks
    }
}
