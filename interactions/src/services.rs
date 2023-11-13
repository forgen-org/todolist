use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use todolist::framework::Projection;

#[async_trait]
pub trait Store<A>
where
    A: Serialize + DeserializeOwned + Sync + Send,
{
    async fn pull(&self) -> Vec<A>;
    async fn push(&self, events: Vec<A>) -> ();
}

#[async_trait]
pub trait Repository<E, A>
where
    A: Projection<E> + Serialize + DeserializeOwned + Sync + Send,
{
    async fn get(&self) -> A;
    async fn save(&self, data: A) -> ();
}
