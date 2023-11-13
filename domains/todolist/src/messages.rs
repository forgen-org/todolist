use crate::events::Event;

pub enum Message {
    AddTask { description: String },
    CompleteTask { index: usize },
}

#[derive(Debug)]
pub enum Error {
    TaskNotFound,
}

impl Message {
    pub fn send(self) -> Result<Vec<Event>, Error> {
        match self {
            Message::AddTask { description } => Ok(vec![Event::TaskAdded { description }]),
            Message::CompleteTask { index } => Ok(vec![Event::TaskCompleted { index }]),
        }
    }
}
