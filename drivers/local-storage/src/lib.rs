use async_trait::async_trait;
use interactions::services::TodoListStore;
use interactions::todolist::{Event, TodoList};
use serde::{Deserialize, Serialize};
use web_sys::Storage;

fn get_local_storage() -> Storage {
    let window = web_sys::window().expect("no global `window` exists");
    let local_storage = window.local_storage().expect("no local storage");
    local_storage.unwrap()
}

pub struct TodoListLocalStorage {}

#[async_trait]
impl TodoListStore for TodoListLocalStorage {
    async fn get_current(&self) -> TodoList {
        let storage = get_local_storage();
        let events = storage
            .get_item("todolist")
            .unwrap()
            .map(|json| {
                let events: Vec<EventDTO> = serde_json::from_str(&json).unwrap();
                events
            })
            .unwrap_or_default();
        let mut todolist = TodoList::default();
        todolist.apply(events.into_iter().map(|e| e.into()).collect());
        todolist
    }

    async fn push_events(&self, new_events: Vec<Event>) -> () {
        let storage = get_local_storage();
        let mut events = storage
            .get_item("todolist")
            .unwrap()
            .map(|json| {
                let events: Vec<EventDTO> = serde_json::from_str(&json).unwrap();
                events
            })
            .unwrap_or_default();
        events.extend(new_events.into_iter().map(|e| e.into()));
        let json = serde_json::to_string(&events).unwrap();
        storage.set_item("todolist", &json).unwrap();
    }
}

#[derive(Serialize, Deserialize)]
enum EventDTO {
    TaskAdded { description: String },
    TaskCompleted { index: usize },
}

impl From<Event> for EventDTO {
    fn from(event: Event) -> Self {
        match event {
            Event::TaskAdded { description } => EventDTO::TaskAdded { description },
            Event::TaskCompleted { index } => EventDTO::TaskCompleted { index },
        }
    }
}

impl From<EventDTO> for Event {
    fn from(dto: EventDTO) -> Self {
        match dto {
            EventDTO::TaskAdded { description } => Event::TaskAdded { description },
            EventDTO::TaskCompleted { index } => Event::TaskCompleted { index },
        }
    }
}
