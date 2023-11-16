use crate::Snapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum CurrentTask {
    #[default]
    None,
    Ready,
    InProgress {
        description: String,
        expires_at: chrono::DateTime<chrono::Utc>,
    },
}

impl From<Snapshot> for CurrentTask {
    fn from(snapshot: Snapshot) -> Self {
        if snapshot.backlog.is_empty() {
            return CurrentTask::None;
        }
        match snapshot.state {
            crate::State::Idle => CurrentTask::Ready,
            crate::State::InProgress { expires_at } => CurrentTask::InProgress {
                description: snapshot.backlog.front().unwrap().clone(),
                expires_at,
            },
        }
    }
}
