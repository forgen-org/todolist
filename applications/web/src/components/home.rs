use crate::runtime::Runtime;

use super::create_task_modal::CreateTaskModal;
use interactions::queries::get_next_task;
use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    let runtime = use_context::<Runtime>().expect("no ctx found");

    let task = use_state(|| None);

    {
        let task = task.clone();
        use_effect_with((), move |_| {
            let runtime = runtime.clone();
            let task = task.clone();

            wasm_bindgen_futures::spawn_local(async move {
                task.set(get_next_task(runtime).await);
            });

            || ()
        });
    }

    html! {
      <>
        <ion-header>
            <ion-toolbar>
                <ion-title>{"Hello!"}</ion-title>
                <ion-buttons slot="end">
                  <CreateTaskModal />
                </ion-buttons>
            </ion-toolbar>
          </ion-header>
          <ion-content>
            {
              if let Some(task) = (*task).clone() {
                html! {
                  &task.description
                }
              } else {
                html! {
                  <h1>{"No task"}</h1>
                }
              }
            }
          </ion-content>
      </>
    }
}
