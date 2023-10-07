use super::services::TodoListStore;
use todolist::Task;

pub struct ListTasks<'a, S: TodoListStore>(pub &'a S);

impl<'a, S: TodoListStore> ListTasks<'a, S> {
    pub async fn handle(&self) -> Vec<Task> {
        let todolist = TodoListStore::get_current(self.0).await;

        todolist.tasks
    }
}
