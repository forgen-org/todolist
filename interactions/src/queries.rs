use crate::services::Repository;
use todolist::Task;

pub async fn get_next_task<R>(runtime: R) -> Option<Task>
where
    R: Repository<todolist::TodoList>,
{
    let todolist = Repository::<todolist::TodoList>::get(&runtime).await?;
    todolist.get_current_task()
}
