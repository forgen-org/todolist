use crate::task::{Task, TaskState};

pub struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_next_task(&mut self) -> Option<&Task> {
        // First, try to find a task that has already been started or is expired
        let started = self.tasks.iter().find(|task| match task.get_state() {
            TaskState::Started => true,
            TaskState::Expired => true,
            _ => false,
        });

        match started {
            Some(started) => return Some(started),
            None => {
                // If no task has been started, sort tasks and find the best one
                self.tasks
                    .iter()
                    .filter(|&task| matches!(task.get_state(), TaskState::Idle))
                    .max()
            }
        }
    }
}
