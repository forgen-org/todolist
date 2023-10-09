use super::services::TodoListStore;
use todolist::{Command as TodoListCommand, Error as TodoListError};

pub struct CreateTask {
    pub todolist_store: Box<dyn TodoListStore>,
}

impl CreateTask {
    pub async fn handle(&self, description: String) -> Result<(), TodoListError> {
        let todolist = self.todolist_store.get_current().await;

        let events = todolist.handle(TodoListCommand::AddTask { description })?;

        self.todolist_store.push_events(events).await;

        Ok(())
    }
}
