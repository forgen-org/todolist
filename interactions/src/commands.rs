use std::rc::Rc;

use crate::services::Store;

pub async fn create_task(
    runtime: Rc<dyn Store<todolist::Event>>,
    description: String,
) -> Result<(), todolist::Error> {
    let event_store: &dyn Store<todolist::Event> = runtime.as_ref();
    // let todolist: todolist::TodoList = <dyn TodoListStore>::pull(runtime.as_ref()).await.into();

    let events = todolist::Message::AddTask { description }.send()?;

    event_store.push(events).await;

    Ok(())
}
