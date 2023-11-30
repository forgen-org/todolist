use super::task_create_button::TaskCreateButton;
use crate::components::{Countdown, LongPressButton, LongPressFab};
use crate::hooks::{use_runtime, use_task_state};
use interactions::{presenters::UseTask, todolist::CurrentTask};
use yew::prelude::*;

#[function_component]
pub fn TaskButtons() -> Html {
    let runtime = use_runtime();
    let task_state = use_task_state();

    let stop = {
        let task_state = task_state.clone();
        let runtime = runtime.clone();

        Callback::from(move |_: MouseEvent| {
            let task_state = task_state.clone();
            let runtime = runtime.clone();

            wasm_bindgen_futures::spawn_local(async move {
                task_state.set(UseTask::stop(&runtime, &task_state).await);
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

    if let Some(CurrentTask::InProgress { expires_in, .. }) = &task_state.current_task {
        html! {
            <div style="display:flex;">
                <LongPressFab color="danger" icon="stop" onclick={stop} />
                <div style="flex-grow:1;"></div>
                <LongPressButton color="dark" onclick={complete}>
                    <Countdown seconds={expires_in.0} />
                </LongPressButton>
                <div style="flex-grow:1;"></div>
                <LongPressFab color="warning" icon="play-skip-forward" onclick={skip} />
            </div>
        }
    } else {
        html! {
            <div style="display:flex;">
                <div style="flex-grow:1;"></div>
                <TaskCreateButton />
                <div style="flex-grow:1;"></div>
            </div>
        }
    }
}
