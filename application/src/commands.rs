use super::services::TodoListStore;
use todolist::{Command as TodoListCommand, Error as TodoListError};

pub struct CreateTask<'a, S: TodoListStore>(pub &'a S);

impl<'a, S: TodoListStore> CreateTask<'a, S> {
    pub async fn handle(&self, description: String) -> Result<(), TodoListError> {
        let todolist = TodoListStore::get_current(self.0).await;

        let events = todolist.handle(TodoListCommand::AddTask { description })?;

        TodoListStore::push_events(self.0, events).await;

        Ok(())
    }
}
