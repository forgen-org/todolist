use serde::{Deserialize, Serialize};

pub struct TodoList {
    tasks: Vec<Task>,
}

struct Task {
    name: String,
    done: bool,
}

impl Default for TodoList {
    fn default() -> Self {
        Self { tasks: vec![] }
    }
}

pub enum Command {
    AddTask { name: String },
    CompleteTask { index: usize },
}

#[derive(Serialize, Deserialize)]
pub enum Event {
    TaskAdded { name: String },
    TaskCompleted { index: usize },
}

impl TodoList {
    pub fn handle(&self, command: Command) -> Result<Vec<Event>, ()> {
        match command {
            Command::AddTask { name } => Ok(vec![Event::TaskAdded { name }]),
            Command::CompleteTask { index } => Ok(vec![Event::TaskCompleted { index }]),
        }
    }

    pub fn apply(&mut self, events: Vec<Event>) -> &Self {
        for event in events {
            match event {
                Event::TaskAdded { name } => {
                    self.tasks.push(Task { name, done: false });
                }
                Event::TaskCompleted { index } => {
                    self.tasks[index].done = true;
                }
            }
        }
        self
    }
}
