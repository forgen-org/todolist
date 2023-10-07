use super::services::TodoListStore;
use todolist::Command as TodoListCommand;

pub struct CreateTask<'a, S: TodoListStore>(pub &'a S);

impl<'a, S: TodoListStore> CreateTask<'a, S> {
    pub async fn handle(&self, description: String) -> () {
        let todolist = TodoListStore::get_current(self.0).await;

        let events = todolist
            .handle(TodoListCommand::AddTask { description })
            .unwrap();

        TodoListStore::push_events(self.0, events).await;
    }
}
