use crate::components::LongPressButton;
use crate::hooks::{use_runtime, use_task_state};
use interactions::{presenters::UseTask, todolist::CurrentTask};
use yew::prelude::*;

#[function_component]
pub fn TaskStickyNote() -> Html {
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
                task_state.set(UseTask::start(&runtime, &task_state).await);
            });
        })
    };

    match &task_state.current_task {
        Some(task) => match task {
            CurrentTask::None => html! {
                <div class="sticky-note bg-peach">
                    <p style="text-align:center">
                        {"Nothing here yet..."}
                        <br />
                        {"Please add a task with the button below."}
                    </p>
                </div>
            },
            CurrentTask::Ready => html! {
                <div class="sticky-note bg-yellow">
                    <LongPressButton color="dark" onclick={start}>
                        {"Start"}
                        <ion-icon name="play" slot="end"></ion-icon>
                    </LongPressButton>
                </div>
            },
            CurrentTask::InProgress { description, .. } => {
                html! {
                    <div class="sticky-note bg-green">
                        <p style="text-align:center">{description}</p>
                    </div>
                }
            }
        },
        None => html! {
            <ion-spinner />
        },
    }
}
