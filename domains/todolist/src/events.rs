use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Event {
    TaskAdded { description: String },
    TaskCompleted,
    TaskDeleted,
    TaskPaused,
    TaskSkipped,
    TaskStarted { at: chrono::DateTime<chrono::Utc> },
}
