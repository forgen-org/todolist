use crate::hooks::{use_runtime, use_task_state};
use crate::ionic::*;
use interactions::presenters::UseTask;
use wasm_bindgen::JsValue;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component]
pub fn CreateTaskModal() -> Html {
    let modal_ref = use_node_ref();
    let textarea_ref = use_node_ref();

    let runtime = use_runtime();
    let task_state = use_task_state();

    let close_modal = {
        let modal_ref = modal_ref.clone();

        Callback::from(move |_e| {
            modal_ref
                .cast::<HTMLIonModalElement>()
                .expect("no modal found")
                .dismiss(JsValue::undefined(), None);
        })
    };

    let save = {
        let task_state = task_state.clone();
        let close_modal = close_modal.clone();
        let runtime = runtime.clone();
        let textarea_ref = textarea_ref.clone();

        Callback::from(move |e| {
            let task_state = task_state.clone();
            let close_modal = close_modal.clone();
            let runtime = runtime.clone();

            let description = textarea_ref
                .cast::<HtmlTextAreaElement>()
                .expect("no modal found")
                .value();

            wasm_bindgen_futures::spawn_local(async move {
                task_state.set(UseTask::add(&runtime, &task_state, description).await);
                close_modal.emit(e)
            });
        })
    };

    html! {
        <>
            <ion-fab edge="true" horizontal="end" vertical="bottom" slot="fixed">
                    <ion-fab-button color="dark" id="open-modal">
                        <ion-icon name="add"></ion-icon>
                    </ion-fab-button>
            </ion-fab>
            <ion-modal initial-breakpoint="0.25" trigger="open-modal" ref={modal_ref}>
                <ion-header>
                    <ion-toolbar>
                        <ion-buttons slot="start">
                            <ion-button color="danger" onclick={close_modal}>{"Cancel"}</ion-button>
                        </ion-buttons>
                        <ion-buttons slot="end">
                            <ion-button fill="solid" onclick={save}>{"Save"}</ion-button>
                        </ion-buttons>
                    </ion-toolbar>
                </ion-header>
                <ion-content class="ion-padding" scroll-y="false">
                    <ion-textarea color="medium" id="description-textarea" placeholder="Write something..." ref={textarea_ref} rows="5" />
                </ion-content>
            </ion-modal>
        </>
    }
}
