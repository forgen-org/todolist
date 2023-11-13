use std::sync::Arc;

use async_trait::async_trait;
use interactions::services::Store;
use local_storage::LocalStore;
use yew::Properties;

#[derive(Clone, Properties)]
pub struct Runtime {
    event_store: Arc<LocalStore<todolist::Event>>,
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
