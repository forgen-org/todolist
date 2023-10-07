use crate::domain::todolist::{Event, TodoList};
use async_trait::async_trait;

#[async_trait]
pub trait TodoListStore {
    async fn get_snapshot(&self) -> TodoList;
    async fn push_events(&self, events: Vec<Event>) -> ();
}
