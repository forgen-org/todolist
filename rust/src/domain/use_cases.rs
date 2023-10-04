use super::{todolist::Command as TodoListCommand, TodoListStore};

pub struct AddTask<'a, A: TodoListStore> {
    pub todolist: &'a A,
}

impl<'a, A: TodoListStore> AddTask<'a, A> {
    pub async fn handle(&self, description: String) -> () {
        let todolist = self.todolist.get().await;

        let events = todolist
            .handle(TodoListCommand::AddTask { description })
            .unwrap();

        self.todolist.save(events).await;
    }
}
