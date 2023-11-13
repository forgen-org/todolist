use crate::services::{Repository, Store};

pub async fn create_task<R>(runtime: &R, description: String) -> Result<(), todolist::Error>
where
    R: Store<todolist::Event> + Repository<todolist::TodoList>,
{
    let events = todolist::Message::AddTask { description }.send()?;

    Store::<todolist::Event>::push(runtime, events.clone()).await;

    let mut todolist = Repository::<todolist::TodoList>::get(runtime)
        .await
        .unwrap_or_default();
    todolist.apply(events);
    Repository::<todolist::TodoList>::save(runtime, todolist).await;

    Ok(())
}
