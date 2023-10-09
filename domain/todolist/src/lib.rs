use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub done: bool,
}

impl Default for TodoList {
    fn default() -> Self {
        Self { tasks: vec![] }
    }
}

pub enum Command {
    AddTask { description: String },
    CompleteTask { index: usize },
}

#[derive(Debug)]
pub enum Error {}

#[derive(Serialize, Deserialize)]
pub enum Event {
    TaskAdded { description: String },
    TaskCompleted { index: usize },
}

impl TodoList {
    pub fn handle(&self, command: Command) -> Result<Vec<Event>, Error> {
        match command {
            Command::AddTask { description } => Ok(vec![Event::TaskAdded { description }]),
            Command::CompleteTask { index } => Ok(vec![Event::TaskCompleted { index }]),
        }
    }

    pub fn apply(&mut self, events: Vec<Event>) -> &Self {
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
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut todo_list = TodoList::default();
        let cmd = Command::AddTask {
            description: "Test task".to_string(),
        };
        let events = todo_list.handle(cmd).unwrap();
        todo_list.apply(events);

        assert_eq!(todo_list.tasks.len(), 1);
        assert_eq!(todo_list.tasks[0].description, "Test task");
        assert_eq!(todo_list.tasks[0].done, false);
    }

    #[test]
    fn test_complete_task() {
        let mut todo_list = TodoList::default();
        let cmd_add = Command::AddTask {
            description: "Test task".to_string(),
        };
        let events_add = todo_list.handle(cmd_add).unwrap();
        todo_list.apply(events_add);

        let cmd_complete = Command::CompleteTask { index: 0 };
        let events_complete = todo_list.handle(cmd_complete).unwrap();
        todo_list.apply(events_complete);

        assert_eq!(todo_list.tasks.len(), 1);
        assert_eq!(todo_list.tasks[0].done, true);
    }

    #[test]
    fn test_handle() {
        let todo_list = TodoList::default();

        // Testing AddTask command
        match todo_list.handle(Command::AddTask {
            description: "Test task".to_string(),
        }) {
            Ok(events) => {
                assert_eq!(events.len(), 1);
                if let Event::TaskAdded { description } = &events[0] {
                    assert_eq!(description, "Test task");
                } else {
                    panic!("Expected TaskAdded event");
                }
            }
            Err(_) => panic!("Expected Ok, got Err"),
        }

        // Testing CompleteTask command
        match todo_list.handle(Command::CompleteTask { index: 0 }) {
            Ok(events) => {
                assert_eq!(events.len(), 1);
                if let Event::TaskCompleted { index } = &events[0] {
                    assert_eq!(*index, 0);
                } else {
                    panic!("Expected TaskCompleted event");
                }
            }
            Err(_) => panic!("Expected Ok, got Err"),
        }
    }

    #[test]
    fn test_apply() {
        let mut todo_list = TodoList::default();

        let events = vec![
            Event::TaskAdded {
                description: "Task 1".to_string(),
            },
            Event::TaskAdded {
                description: "Task 2".to_string(),
            },
            Event::TaskCompleted { index: 0 },
        ];
        todo_list.apply(events);

        assert_eq!(todo_list.tasks.len(), 2);
        assert_eq!(todo_list.tasks[0].description, "Task 1");
        assert_eq!(todo_list.tasks[0].done, true);
        assert_eq!(todo_list.tasks[1].description, "Task 2");
        assert_eq!(todo_list.tasks[1].done, false);
    }
}
