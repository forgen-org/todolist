#[derive(Default)]
pub struct TodoList {
    pub tasks: Vec<Task>,
}

pub struct Task {
    pub description: String,
    pub done: bool,
}

pub enum Command {
    AddTask { description: String },
    CompleteTask { index: usize },
}

#[derive(Debug)]
pub enum Error {
    TaskNotFound,
}

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
