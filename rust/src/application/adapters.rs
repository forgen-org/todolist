use crate::domain::{TodoList, TodoListEvent, TodoListStore};
use crate::infrastructure::in_memory_store::InMemoryStore;
use async_trait::async_trait;

#[async_trait]
impl TodoListStore for InMemoryStore {
    async fn get(&self) -> TodoList {
        let mut todolist = TodoList::default();

        match self.get() {
            Some(value) => {
                let events: Vec<TodoListEvent> = serde_json::from_value(value).unwrap_or_default();
                todolist.apply(events);
            }
            None => (),
        }

        todolist
    }

    async fn save(&self, events: Vec<TodoListEvent>) -> () {
        let value = serde_json::to_value(events).unwrap_or_default();
        self.save(value);
    }
}
