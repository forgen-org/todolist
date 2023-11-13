use std::rc::Rc;

use crate::services::Store;

pub async fn get_next_task<R: Store<todolist::Event>>(
    runtime: R,
    description: String,
) -> Result<Option<String>, todolist::Error> {
    unimplemented!("get_next_task")
    // let event_stor Store<todolist::Event>(runtime);

    // Ok(())
}
