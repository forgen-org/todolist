use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[async_trait]
pub trait Store<A>
where
    A: Serialize + DeserializeOwned + Sync + Send,
{
    async fn pull(&self) -> Vec<A>;
    async fn push(&self, events: Vec<A>) -> ();
}
