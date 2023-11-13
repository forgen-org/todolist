use async_trait::async_trait;
use gloo_storage::{LocalStorage, Storage};
use interactions::services::{Repository, Store};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct LocalStore<A> {
    key: String,
    phantom: std::marker::PhantomData<A>,
}

impl<A> LocalStore<A> {
    pub fn new(key: String) -> Self {
        Self {
            key,
            phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<A> Store<A> for LocalStore<A>
where
    A: Serialize + DeserializeOwned + Sync + Send,
{
    async fn pull(&self) -> Vec<A> {
        LocalStorage::get::<Vec<A>>(self.key.clone()).unwrap_or(vec![])
    }

    async fn push(&self, new_events: Vec<A>) -> () {
        let mut events = self.pull().await;
        events.extend(new_events);
        LocalStorage::set(self.key.clone(), &events).unwrap();
    }
}

#[async_trait]
impl<A> Repository<A> for LocalStore<A>
where
    A: Serialize + DeserializeOwned + Sync + Send,
{
    async fn get(&self) -> Option<A> {
        LocalStorage::get::<A>(self.key.clone()).ok()
    }

    async fn save(&self, new_value: A) -> () {
        LocalStorage::set(self.key.clone(), &new_value).unwrap();
    }
}
