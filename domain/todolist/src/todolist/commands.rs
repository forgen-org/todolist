use crate::entities::TaskId;

pub enum Command {
    CreateTask {
        description: String,
    },
    StartTask,
    CompleteTask,
    SplitTask {
        a: String,
        b: String,
    },
    CompareTasks {
        least_important: Option<TaskId>,
        least_urgent: Option<TaskId>,
        most_important: Option<TaskId>,
        most_urgent: Option<TaskId>,
    },
}
