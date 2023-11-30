use crate::{errors::Error, Event, Message, Seconds};
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
    Started {
        expires_at: chrono::DateTime<chrono::Utc>,
    },
    Paused {
        remaining: Seconds,
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
                Event::TaskDeleted => {
                    self.backlog.pop_front();
                    self.state = State::Idle;
                }
                Event::TaskPaused => match self.state {
                    State::Started { expires_at } => {
                        self.state = State::Paused {
                            remaining: Seconds((expires_at - chrono::Utc::now()).num_seconds()),
                        };
                    }
                    _ => (),
                },
                Event::TaskSkipped => {
                    self.backlog.rotate_left(1);
                    self.state = State::Idle;
                }
                Event::TaskStarted { at } => {
                    let duration = match &self.state {
                        State::Paused { remaining } => chrono::Duration::seconds(remaining.0),
                        _ => chrono::Duration::minutes(60),
                    };
                    self.state = State::Started {
                        expires_at: at.clone() + duration,
                    };
                }
            }
        }
    }

    pub fn send(&self, message: Message) -> Result<Vec<Event>, Error> {
        match message {
            Message::AddTask { description } => Ok(vec![Event::TaskAdded { description }]),
            Message::CompleteTask => match self.state {
                State::Started { .. } => Ok(vec![
                    Event::TaskCompleted,
                    Event::TaskStarted {
                        at: chrono::Utc::now(),
                    },
                ]),
                _ => Err(Error::TaskNotStarted),
            },
            Message::DeleteTask => match self.state {
                State::Started { .. } => Ok(vec![Event::TaskDeleted]),
                _ => Err(Error::TaskNotStarted),
            },
            Message::PauseTask => match self.state {
                State::Started { .. } => Ok(vec![Event::TaskPaused]),
                _ => Err(Error::TaskNotStarted),
            },
            Message::SkipTask => match self.state {
                State::Started { .. } => Ok(vec![Event::TaskSkipped]),
                _ => Err(Error::TaskNotStarted),
            },
            Message::StartTask => match self.state {
                State::Idle => {
                    if self.backlog.len() > 0 {
                        Ok(vec![Event::TaskStarted {
                            at: chrono::Utc::now(),
                        }])
                    } else {
                        Err(Error::EmptyBacklog)
                    }
                }
                _ => Err(Error::TaskAlreadyStarted),
            },
        }
    }
}

impl From<Vec<Event>> for Snapshot {
    fn from(events: Vec<Event>) -> Self {
        let mut snapshot = Snapshot::default();
        snapshot.apply(events);
        snapshot
    }
}
