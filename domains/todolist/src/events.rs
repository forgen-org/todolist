use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Event {
    TaskAdded { description: String },
    TaskCompleted,
    TaskDeleted,
    TaskSkipped,
    TaskStarted,
}
