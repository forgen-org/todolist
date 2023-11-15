use std::sync::{Arc, Mutex};

use gloo_storage::{LocalStorage, Storage};
use interactions::commands::TaskCommand;
use interactions::ports::{CurrentTaskRepository, EventStore, Observable, SnapshotRepository};
use interactions::presenters::{TaskState, UseTask};
use interactions::queries::TaskQuery;
use interactions::todolist::{CurrentTask, Event, Snapshot};
use yew::Properties;

#[derive(Clone, Properties)]
pub struct Runtime {
    task_state: Arc<SimpleObservable<TaskState>>,
}

impl PartialEq for Runtime {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            task_state: Arc::new(SimpleObservable::new(TaskState { current_task: None })),
        }
    }
}

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
        LocalStorage::set("tasks", &events).unwrap();
    }
}

#[async_trait::async_trait]
impl SnapshotRepository for Runtime {
    async fn get(&self) -> Option<Snapshot> {
        LocalStorage::get::<Snapshot>("tasks").ok()
    }
    async fn save(&self, value: Snapshot) -> () {
        LocalStorage::set("tasks", &value).unwrap();
    }
}

impl TaskCommand for Runtime {}
impl TaskQuery for Runtime {}
impl UseTask for Runtime {}

struct SimpleObservable<T> {
    state: Mutex<T>,
}

impl<T> SimpleObservable<T> {
    fn new(state: T) -> Self {
        Self {
            state: Mutex::new(state),
        }
    }
}

impl<T> Observable<T> for SimpleObservable<T> {
    fn mutate(&self, new_state: T) -> () {
        *self.state.lock().unwrap() = new_state;
    }
    fn observe<F>(&self, subscriber: F) -> ()
    where
        F: Fn(&T) -> (),
    {
        subscriber(&*self.state.lock().unwrap());
    }
}

#[async_trait::async_trait]
impl Observable<TaskState> for Runtime {
    fn mutate(&self, new_state: TaskState) -> () {
        self.task_state.mutate(new_state);
    }
    fn observe<F>(&self, subscriber: F) -> ()
    where
        F: Fn(&TaskState) -> (),
    {
        self.task_state.observe(subscriber);
    }
}
