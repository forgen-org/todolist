use todolist::CurrentTask;

use crate::ports::{CurrentTaskRepository, Observable};

#[derive(Clone, Default)]
pub struct TaskState {
    pub current_task: Option<CurrentTask>,
}

#[async_trait::async_trait]
pub trait UseTask: CurrentTaskRepository + Observable<TaskState> {
    async fn fetch_current_task(&self) {
        let current_task = Some(CurrentTaskRepository::get(self).await.unwrap_or_default());
        self.mutate(TaskState { current_task });
    }
}
