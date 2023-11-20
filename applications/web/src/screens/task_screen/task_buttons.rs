use super::task_create_button::TaskCreateButton;
use crate::components::{LongPressCountdown, LongPressFab};
use crate::hooks::{use_runtime, use_task_state};
use gloo_console::log;
use interactions::{presenters::UseTask, todolist::CurrentTask};
use yew::prelude::*;

#[function_component]
pub fn TaskButtons() -> Html {
    let runtime = use_runtime();
    let task_state = use_task_state();

    let delete = {
        let task_state = task_state.clone();
        let runtime = runtime.clone();

        Callback::from(move |_: MouseEvent| {
            let task_state = task_state.clone();
            let runtime = runtime.clone();

            wasm_bindgen_futures::spawn_local(async move {
                task_state.set(UseTask::delete(&runtime, &task_state).await);
            });
        })
    };

    let complete = {
        let task_state = task_state.clone();
        let runtime = runtime.clone();

        Callback::from(move |_: MouseEvent| {
            let task_state = task_state.clone();
            let runtime = runtime.clone();

            wasm_bindgen_futures::spawn_local(async move {
                task_state.set(UseTask::complete(&runtime, &task_state).await);
            });
        })
    };

    let skip = {
        let task_state = task_state.clone();
        let runtime = runtime.clone();

        Callback::from(move |_: MouseEvent| {
            let task_state = task_state.clone();
            let runtime = runtime.clone();

            wasm_bindgen_futures::spawn_local(async move {
                task_state.set(UseTask::skip(&runtime, &task_state).await);
            });
        })
    };

    if let Some(CurrentTask::InProgress { .. }) = &task_state.current_task {
        html! {
            <div style="display:flex; padding: 24px;">
                <LongPressFab color="danger" icon="trash" onclick={delete} />
                <div style="flex-grow:1;"></div>
                <LongPressCountdown onclick={complete} seconds={120} />
                <div style="flex-grow:1;"></div>
                <LongPressFab color="warning" icon="play-skip-forward" onclick={skip} />
            </div>
        }
    } else {
        html! {
            <TaskCreateButton />
        }
    }
}
