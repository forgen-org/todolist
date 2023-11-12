use std::rc::Rc;

use super::services::TodoListStore;
use todolist::{Command as TodoListCommand, Error as TodoListError};

pub async fn create_task(
    runtime: Rc<dyn TodoListStore>,
    description: String,
) -> Result<(), TodoListError> {
    let todolist = TodoListStore::get_current(runtime.as_ref()).await;

    let events = todolist.handle(TodoListCommand::AddTask { description })?;

    TodoListStore::push_events(runtime.as_ref(), events).await;

    Ok(())
}
