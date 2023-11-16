use todolist::CurrentTask;

use crate::{commands::TaskCommand, queries::TaskQuery};

#[derive(Clone, Default, PartialEq)]
pub struct TaskState {
    pub current_task: Option<CurrentTask>,
}

#[async_trait::async_trait]
pub trait UseTask: TaskCommand + TaskQuery {
    async fn add(
        &self,
        state: &TaskState,
        description: String,
    ) -> Result<TaskState, todolist::Error> {
        TaskCommand::send(self, todolist::Message::AddTask { description }).await?;
        Ok(self.get_current(state).await)
    }

    async fn get_current(&self, state: &TaskState) -> TaskState {
        let current_task = Some(TaskQuery::get_current_task(self).await);
        TaskState {
            current_task,
            ..state.clone()
        }
    }

    async fn start(&self, state: &TaskState) -> Result<TaskState, todolist::Error> {
        TaskCommand::send(self, todolist::Message::StartTask).await?;
        Ok(self.get_current(state).await)
    }
}
