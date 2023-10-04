use super::todolist::{Event, TodoList};
use async_trait::async_trait;

#[async_trait]
pub trait TodoListStore {
    async fn get(&self) -> TodoList;
    async fn save(&self, events: Vec<Event>) -> ();
}
