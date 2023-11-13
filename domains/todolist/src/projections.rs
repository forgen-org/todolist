use super::events::Event;
use super::framework::Projection;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct TodoList {
    pub tasks: Vec<Task>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub done: bool,
}

impl Projection<Event> for TodoList {
    fn apply(&mut self, events: Vec<Event>) {
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

impl TodoList {
    pub fn get_current_task(&self) -> Option<Task> {
        self.tasks.last().cloned()
    }
}
