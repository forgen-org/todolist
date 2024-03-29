use todolist::CurrentTask;

use crate::{commands::TaskCommand, queries::TaskQuery};

#[derive(Clone, Default, PartialEq)]
pub struct TaskState {
    pub current_task: Option<CurrentTask>,
    pub error: Option<String>,
}

#[async_trait::async_trait]
pub trait UseTask: TaskCommand + TaskQuery {
    async fn add(&self, state: &TaskState, description: String) -> TaskState {
        if let Err(error) =
            TaskCommand::send(self, todolist::Message::AddTask { description }).await
        {
            TaskState {
                error: Some(error.to_string()),
                ..state.clone()
            }
        } else {
            self.get_current(state).await
        }
    }

    async fn complete(&self, state: &TaskState) -> TaskState {
        if let Err(error) = TaskCommand::send(self, todolist::Message::CompleteTask).await {
            TaskState {
                error: Some(error.to_string()),
                ..state.clone()
            }
        } else {
            self.get_current(state).await
        }
    }

    async fn delete(&self, state: &TaskState) -> TaskState {
        if let Err(error) = TaskCommand::send(self, todolist::Message::DeleteTask).await {
            TaskState {
                error: Some(error.to_string()),
                ..state.clone()
            }
        } else {
            self.get_current(state).await
        }
    }

    fn dismiss_error(&self, state: &TaskState) -> TaskState {
        TaskState {
            error: None,
            ..state.clone()
        }
    }

    async fn get_current(&self, state: &TaskState) -> TaskState {
        let current_task = Some(TaskQuery::get_current_task(self).await);
        TaskState {
            current_task,
            ..state.clone()
        }
    }

    async fn skip(&self, state: &TaskState) -> TaskState {
        if let Err(error) = TaskCommand::send(self, todolist::Message::SkipTask).await {
            TaskState {
                error: Some(error.to_string()),
                ..state.clone()
            }
        } else {
            self.get_current(state).await
        }
    }

    async fn start(&self, state: &TaskState) -> TaskState {
        if let Err(error) = TaskCommand::send(self, todolist::Message::StartTask).await {
            TaskState {
                error: Some(error.to_string()),
                ..state.clone()
            }
        } else {
            self.get_current(state).await
        }
    }

    async fn stop(&self, state: &TaskState) -> TaskState {
        if let Err(error) = TaskCommand::send(self, todolist::Message::PauseTask).await {
            TaskState {
                error: Some(error.to_string()),
                ..state.clone()
            }
        } else {
            self.get_current(state).await
        }
    }
}
