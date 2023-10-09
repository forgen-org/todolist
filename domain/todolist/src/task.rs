use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct Task {
    id: TaskId,
    description: String,
    ended_at: Option<DateTime<Utc>>,
    importancy: Importancy,
    started_at: Option<DateTime<Utc>>,
    urgency: Urgency,
}

#[derive(Clone)]
pub struct TaskId(Uuid);

#[derive(Clone)]
pub struct Importancy(usize);

#[derive(Clone)]
pub struct Urgency(usize);

pub enum TaskState {
    Idle,
    Started,
    Ended,
    Expired,
}

impl Task {
    pub fn get_state(&self) -> TaskState {
        match (&self.ended_at, &self.started_at) {
            (Some(_), _) => TaskState::Ended,
            (None, Some(started_at)) => {
                if Utc::now().signed_duration_since(*started_at) >= Duration::minutes(60) {
                    TaskState::Expired
                } else {
                    TaskState::Started
                }
            }
            (None, None) => TaskState::Idle,
        }
    }
}

// Comparison rules for tasks:
impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.importancy.0 == other.importancy.0 && self.urgency.0 == other.urgency.0
    }
}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // First, compare based on urgency (larger is better)
        match self.urgency.0.cmp(&other.urgency.0) {
            std::cmp::Ordering::Equal => {
                // If urgency is equal, compare based on importancy (larger is better)
                Some(self.importancy.0.cmp(&other.importancy.0))
            }
            ordering => Some(ordering),
        }
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // This unwraps the Option returned by partial_cmp assuming that
        // we will always be able to produce a total ordering for any two tasks.
        self.partial_cmp(other).unwrap()
    }
}
