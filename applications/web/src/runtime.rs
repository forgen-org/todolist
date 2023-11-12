use async_trait::async_trait;
use interactions::services::TodoListStore;
use interactions::todolist::{Event, TodoList};
use local_storage::TodoListLocalStorage;

pub struct Runtime {
    local_storage: TodoListLocalStorage,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            local_storage: TodoListLocalStorage {},
        }
    }
}

#[async_trait]
impl TodoListStore for Runtime {
    async fn get_current(&self) -> TodoList {
        self.local_storage.get_current().await
    }

    async fn push_events(&self, new_events: Vec<Event>) -> () {
        self.local_storage.push_events(new_events).await
    }
}
