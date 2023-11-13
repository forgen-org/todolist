use crate::services::Store;

pub async fn create_task<R: Store<todolist::Event>>(
    runtime: &R,
    description: String,
) -> Result<(), todolist::Error> {
    let events = todolist::Message::AddTask { description }.send()?;

    Store::<todolist::Event>::push(runtime, events).await;

    Ok(())
}
