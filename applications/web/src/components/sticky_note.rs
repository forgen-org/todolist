use crate::hooks::{use_runtime, use_task_state};

use interactions::{presenters::UseTask, todolist::CurrentTask};
use yew::prelude::*;

#[function_component]
pub fn StickyNote() -> Html {
    let runtime = use_runtime();
    let task_state = use_task_state();

    {
        let runtime = runtime.clone();
        let task_state = task_state.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                task_state.set(UseTask::get_current(&runtime, &task_state).await);
            });
            || ()
        });
    }

    let start = {
        let runtime = runtime.clone();
        let task_state = task_state.clone();
        Callback::from(move |_| {
            let runtime = runtime.clone();
            let task_state = task_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                task_state.set(UseTask::start(&runtime, &task_state).await.unwrap());
            });
        })
    };

    match &task_state.current_task {
        Some(task) => match task {
            CurrentTask::None => html! {
                {"Nothing here yet... Please add a task with the button below."}
            },
            CurrentTask::Ready => html! {
                <ion-button color="dark" onclick={start} shape="round" size="large">
                    {"Start"}
                    <ion-icon name="play" slot="end"></ion-icon>
                </ion-button>
            },
            CurrentTask::InProgress { description, .. } => html! {
                {description}
            },
        },
        None => html! {
            <ion-spinner />
        },
    }
}
