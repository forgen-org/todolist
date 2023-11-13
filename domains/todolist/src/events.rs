use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Event {
    TaskAdded { description: String },
    TaskCompleted { index: usize },
}
