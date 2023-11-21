use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Empty backlog")]
    EmptyBacklog,

    #[error("Task already started")]
    TaskAlreadyStarted,

    #[error("Task not started")]
    TaskNotStarted,
}
