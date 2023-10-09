use crate::entities::TaskId;

pub enum Event {
    TaskCreated { id: TaskId, description: String },
    TaskStarted { id: TaskId },
    TaskCompleted { id: TaskId },
}
