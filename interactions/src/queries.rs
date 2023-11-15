use crate::ports::CurrentTaskRepository;
use todolist::CurrentTask;

#[async_trait::async_trait]
pub trait TaskQuery: CurrentTaskRepository {
    async fn get_current_task(&self) -> CurrentTask {
        CurrentTaskRepository::get(self).await.unwrap_or_default()
    }
}
