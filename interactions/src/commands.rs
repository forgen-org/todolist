use crate::ports::{CurrentTaskRepository, EventStore, SnapshotRepository};

#[async_trait::async_trait]
pub trait TaskCommand: CurrentTaskRepository + EventStore + SnapshotRepository {
    async fn send(&self, message: todolist::Message) -> Result<(), todolist::Error> {
        let mut snapshot = SnapshotRepository::get(self).await.unwrap_or_default();
        let new_events = snapshot.send(message)?;
        snapshot.apply(new_events.clone());

        EventStore::push(self, new_events).await;
        SnapshotRepository::save(self, snapshot.clone()).await;
        CurrentTaskRepository::save(self, snapshot.into()).await;

        Ok(())
    }
}
