use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Empty backlog")]
    EmptyBacklog,

    #[error("Task not in progress")]
    NotInProgress,
}
