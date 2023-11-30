use crate::hooks::{use_runtime, use_task_state};
use crate::ionic::*;
use interactions::presenters::UseTask;
use wasm_bindgen::JsValue;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component]
pub fn TaskCreateButton() -> Html {
    let modal_ref = use_node_ref();
    let textarea_ref = use_node_ref();

    let runtime = use_runtime();
    let task_state = use_task_state();

    let open_modal = {
        let modal_ref = modal_ref.clone();
        let textarea_ref = textarea_ref.clone();

        Callback::from(move |_| {
            let modal_ref = modal_ref.clone();
            let textarea_ref = textarea_ref.clone();
            wasm_bindgen_futures::spawn_local(async move {
                modal_ref
                    .cast::<HTMLIonModalElement>()
                    .expect("no modal found")
                    .present()
                    .await;
                textarea_ref
                    .cast::<HTMLIonTextareaElement>()
                    .expect("no modal found")
                    .set_focus()
                    .await;
            })
        })
    };

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

            let textarea_ref = textarea_ref.clone();
            wasm_bindgen_futures::spawn_local(async move {
                task_state.set(UseTask::add(&runtime, &task_state, description).await);
                let textarea: HtmlTextAreaElement = textarea_ref
                    .cast::<HTMLIonTextareaElement>()
                    .expect("no modal found")
                    .get_input_element()
                    .await
                    .into();
                textarea.set_value("");
                close_modal.emit(e)
            });
        })
    };

    html! {
        <>
            <ion-fab-button color="dark" onclick={open_modal}>
                <ion-icon name="add"></ion-icon>
            </ion-fab-button>
            <ion-modal initial-breakpoint="0.25" ref={modal_ref}>
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
                      <ion-textarea
                          color="medium"
                          label="Description"
                          label-placement="floating"
                          ref={textarea_ref}
                          rows="5"
                      >
                    </ion-textarea>
                </ion-content>
            </ion-modal>
        </>
    }
}
