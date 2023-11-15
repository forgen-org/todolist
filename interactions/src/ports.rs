#[async_trait::async_trait]
pub trait EventStore {
    async fn pull(&self) -> Option<Vec<todolist::Event>>;
    async fn push(&self, events: Vec<todolist::Event>) -> ();
}

#[async_trait::async_trait]
pub trait SnapshotRepository {
    async fn get(&self) -> Option<todolist::Snapshot>;
    async fn save(&self, value: todolist::Snapshot) -> ();
}

#[async_trait::async_trait]
pub trait CurrentTaskRepository {
    async fn get(&self) -> Option<todolist::CurrentTask>;
    async fn save(&self, value: todolist::CurrentTask) -> ();
}

pub trait Observable<T> {
    fn mutate(&self, new_state: T) -> ();
    fn observe<F>(&self, subscriber: F) -> ()
    where
        F: Fn(&T) -> ();
}
