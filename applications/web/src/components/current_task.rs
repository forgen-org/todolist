use crate::runtime::Runtime;

use gloo_console::log;
use interactions::{
    ports::Observable,
    presenters::{TaskState, UseTask},
};
use yew::prelude::*;

#[function_component]
pub fn CurrentTaskView() -> Html {
    let runtime = use_context::<Runtime>().expect("no ctx found");
    let state = use_state(|| TaskState::default());

    {
        let runtime = runtime.clone();
        let state = state.clone();
        use_effect_with((), move |_| {
            Observable::<TaskState>::observe(&runtime, move |new_state| {
                log!("something changed");
                state.set(new_state.clone());
            });

            log!("use effect");

            wasm_bindgen_futures::spawn_local(async move {
                log!("fetching current task");
                UseTask::fetch_current_task(&runtime).await;
                log!("done");
            });

            || ()
        });
    }

    let refetch = Callback::from(move |_| {
        let runtime = runtime.clone();
        wasm_bindgen_futures::spawn_local(async move {
            log!("fetching current task");
            UseTask::fetch_current_task(&runtime).await;
            log!("done");
        });
    });

    // let start = {
    //     let runtime = runtime.clone();
    //     Callback::from(move |_| {
    //         let runtime = runtime.clone();
    //         wasm_bindgen_futures::spawn_local(async move {
    //             UseCurrentTask::start(&runtime).await.unwrap();
    //         });
    //     })
    // };

    if let Some(..) = (*state).current_task {
        return html! {
            {"Ready!"}
        };
    } else {
        return html! {
           <button onclick={refetch}>{"Refetch"}</button>
        };
    }

    // match (*task).clone() {
    //     CurrentTask::InProgress { description, .. } => html! {
    //         <>
    //             {&description}
    //         </>
    //     },
    //     CurrentTask::None => html! {
    //         <>
    //             {"No task found"}
    //         </>
    //     },
    //     CurrentTask::Ready => html! {
    //         <>
    //             <ion-button onclick={start} >{"Start"}</ion-button>
    //         </>
    //     },
    // }
}
