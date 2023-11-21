use crate::{Seconds, Snapshot, State};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum CurrentTask {
    #[default]
    None,
    Ready,
    InProgress {
        description: String,
        expires_in: Seconds,
    },
}

impl From<Snapshot> for CurrentTask {
    fn from(snapshot: Snapshot) -> Self {
        if let Some(current) = snapshot.backlog.front() {
            match snapshot.state {
                State::Idle => CurrentTask::Ready,
                State::Paused { .. } => CurrentTask::Ready,
                State::Started { expires_at } => CurrentTask::InProgress {
                    description: current.clone(),
                    expires_in: Seconds((expires_at - chrono::Utc::now()).num_seconds()),
                },
            }
        } else {
            CurrentTask::None
        }
    }
}
