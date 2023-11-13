use std::sync::Arc;

use async_trait::async_trait;
use interactions::services::{Repository, Store};
use local_storage::LocalStore;
use yew::Properties;

#[derive(Clone, Properties)]
pub struct Runtime {
    event_store: Arc<LocalStore<todolist::Event>>,
    todolist_repository: Arc<LocalStore<todolist::TodoList>>,
}

impl PartialEq for Runtime {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            event_store: Arc::new(LocalStore::new("events".to_string())),
            todolist_repository: Arc::new(LocalStore::new("todolist".to_string())),
        }
    }
}

#[async_trait]
impl Store<todolist::Event> for Runtime {
    async fn pull(&self) -> Vec<todolist::Event> {
        self.event_store.pull().await
    }
    async fn push(&self, new_events: Vec<todolist::Event>) -> () {
        self.event_store.push(new_events).await
    }
}

#[async_trait]
impl Repository<todolist::TodoList> for Runtime {
    async fn get(&self) -> Option<todolist::TodoList> {
        self.todolist_repository.get().await
    }
    async fn save(&self, new_value: todolist::TodoList) -> () {
        self.todolist_repository.save(new_value).await
    }
}
