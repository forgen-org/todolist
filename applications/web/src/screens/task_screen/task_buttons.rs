use super::task_create_button::TaskCreateButton;
use crate::hooks::{use_runtime, use_task_state};
use interactions::{presenters::UseTask, todolist::CurrentTask};
use yew::prelude::*;

#[function_component]
pub fn TaskButtons() -> Html {
    let runtime = use_runtime();
    let task_state = use_task_state();

    let complete = {
        let task_state = task_state.clone();
        let runtime = runtime.clone();

        Callback::from(move |_| {
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

        Callback::from(move |_| {
            let task_state = task_state.clone();
            let runtime = runtime.clone();

            wasm_bindgen_futures::spawn_local(async move {
                task_state.set(UseTask::skip(&runtime, &task_state).await);
            });
        })
    };

    if let Some(CurrentTask::InProgress { .. }) = &task_state.current_task {
        html! {
            <>
                <ion-fab edge="true" horizontal="start" vertical="bottom" slot="fixed">
                    <ion-fab-button color="danger" size="small">
                        <ion-icon name="trash"></ion-icon>
                    </ion-fab-button>
                </ion-fab>
                <ion-fab edge="true" horizontal="center" vertical="bottom" slot="fixed">
                    <ion-fab-button color="success" onclick={complete}>
                        <ion-icon name="checkmark"></ion-icon>
                    </ion-fab-button>
                </ion-fab>
                <ion-fab edge="true" horizontal="end" vertical="bottom" slot="fixed">
                    <ion-fab-button color="warning" onclick={skip} size="small">
                        <ion-icon name="play-skip-forward"></ion-icon>
                    </ion-fab-button>
                </ion-fab>
            </>
        }
    } else {
        html! {
            <TaskCreateButton />
        }
    }
}
