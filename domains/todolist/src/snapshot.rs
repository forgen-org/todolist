use crate::{errors::Error, Event, Message};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    pub backlog: VecDeque<String>,
    pub state: State,
}

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum State {
    #[default]
    Idle,
    InProgress {
        expires_at: chrono::DateTime<chrono::Utc>,
    },
}

impl Snapshot {
    pub fn apply(&mut self, events: Vec<Event>) -> () {
        for event in events.iter() {
            match event {
                Event::TaskAdded { description } => {
                    self.backlog.push_back(description.clone());
                }
                Event::TaskCompleted => {
                    self.backlog.pop_front();
                    self.state = State::Idle;
                }
                Event::TaskSkipped => {
                    self.backlog.rotate_left(1);
                    self.state = State::Idle;
                }
                Event::TaskStarted => {
                    self.state = State::InProgress {
                        expires_at: chrono::Utc::now() + chrono::Duration::minutes(60),
                    };
                }
            }
        }
    }

    pub fn send(&self, message: Message) -> Result<Vec<Event>, Error> {
        match message {
            Message::AddTask { description } => Ok(vec![Event::TaskAdded { description }]),
            Message::CompleteTask => {
                if let State::InProgress { .. } = self.state {
                    Ok(vec![Event::TaskCompleted])
                } else {
                    Err(Error::NotInProgress)
                }
            }
            Message::SkipTask => {
                if let State::InProgress { .. } = self.state {
                    Ok(vec![Event::TaskSkipped])
                } else {
                    Err(Error::NotInProgress)
                }
            }
            Message::StartTask => {
                if self.backlog.len() > 0 {
                    Ok(vec![Event::TaskStarted])
                } else {
                    Err(Error::EmptyBacklog)
                }
            }
        }
    }
}
