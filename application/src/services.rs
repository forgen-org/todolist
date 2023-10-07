use async_trait::async_trait;
use todolist::{Event, TodoList};

#[async_trait]
pub trait TodoListStore {
    async fn get_current(&self) -> TodoList;
    async fn push_events(&self, events: Vec<Event>) -> ();
}
