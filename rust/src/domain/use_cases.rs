use super::{todolist::Command as TodoListCommand, Domain};

impl Domain {
    pub async fn add_task(&self) -> Result<(), ()> {
        let todolist = self.todolist.get().await;

        let events = todolist.handle(TodoListCommand::AddTask {
            name: "task".to_string(),
        })?;

        self.todolist.save(events).await;

        Ok(())
    }
}
