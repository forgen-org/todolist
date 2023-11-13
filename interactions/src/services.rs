use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[async_trait]
pub trait Store<E>
where
    E: Serialize + DeserializeOwned + Sync + Send,
{
    async fn pull(&self) -> Vec<E>;
    async fn push(&self, events: Vec<E>) -> ();
}

#[async_trait]
pub trait Repository<P>
where
    P: Serialize + DeserializeOwned + Sync + Send,
{
    async fn get(&self) -> Option<P>;
    async fn save(&self, data: P) -> ();
}
