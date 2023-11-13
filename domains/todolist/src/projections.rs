use serde::{Deserialize, Serialize};

use crate::events::Event;

#[derive(Default, Serialize, Deserialize)]
pub struct TodoList {
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub done: bool,
}

impl TodoList {
    pub fn apply(&mut self, events: Vec<Event>) -> () {
        for event in events.iter() {
            match event {
                Event::TaskAdded { description } => {
                    self.tasks.push(Task {
                        description: description.clone(),
                        done: false,
                    });
                }
                Event::TaskCompleted { index } => {
                    self.tasks[*index].done = true;
                }
            }
        }
    }
}

impl From<Vec<Event>> for TodoList {
    fn from(events: Vec<Event>) -> Self {
        let mut todo_list = TodoList::default();
        todo_list.apply(events);
        todo_list
    }
}
