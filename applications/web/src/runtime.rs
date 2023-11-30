use gloo_storage::{LocalStorage, Storage};
use interactions::commands::TaskCommand;
use interactions::ports::{CurrentTaskRepository, EventStore, SnapshotRepository};
use interactions::presenters::UseTask;
use interactions::queries::TaskQuery;
use interactions::todolist::{CurrentTask, Event, Snapshot};

#[derive(Clone, Default)]
pub struct Runtime {}

#[async_trait::async_trait]
impl CurrentTaskRepository for Runtime {
    async fn get(&self) -> Option<CurrentTask> {
        LocalStorage::get::<CurrentTask>("tasks").ok()
    }
    async fn save(&self, value: CurrentTask) -> () {
        LocalStorage::set("tasks", &value).unwrap();
    }
}

#[async_trait::async_trait]
impl EventStore for Runtime {
    async fn pull(&self) -> Option<Vec<Event>> {
        LocalStorage::get::<Vec<Event>>("events").ok()
    }
    async fn push(&self, new_events: Vec<Event>) -> () {
        let mut events = self.pull().await.unwrap_or_default();
        events.extend(new_events);
        LocalStorage::set("events", &events).unwrap();
    }
}

#[async_trait::async_trait]
impl SnapshotRepository for Runtime
where
    Runtime: EventStore,
{
    async fn get(&self) -> Option<Snapshot> {
        // LocalStorage::get::<Snapshot>("snapshot").ok()
        EventStore::pull(self).await.map(|events| events.into())
    }
    async fn save(&self, value: Snapshot) -> () {
        LocalStorage::set("snapshot", &value).unwrap();
    }
}

impl TaskCommand for Runtime {}
impl TaskQuery for Runtime {}
impl UseTask for Runtime {}
