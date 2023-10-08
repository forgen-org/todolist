use application::services::TodoListStore;
use application::todolist::{Event, TodoList};
use async_trait::async_trait;
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
                let events: Vec<Event> = serde_json::from_str(&json).unwrap();
                events
            })
            .unwrap_or(vec![]);
        let mut todolist = TodoList::default();
        todolist.apply(events);
        todolist
    }

    async fn push_events(&self, new_events: Vec<Event>) -> () {
        let storage = get_local_storage();
        let mut events = storage
            .get_item("todolist")
            .unwrap()
            .map(|json| {
                let events: Vec<Event> = serde_json::from_str(&json).unwrap();
                events
            })
            .unwrap_or(vec![]);
        events.extend(new_events);
        let json = serde_json::to_string(&events).unwrap();
        storage.set_item("todolist", &json).unwrap();
    }
}
