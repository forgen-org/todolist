use crate::ionic::*;
use crate::runtime::Runtime;
use interactions::presenters::CreateTaskForm;
use wasm_bindgen::JsValue;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component]
pub fn CreateTaskModal() -> Html {
    let modal_ref = use_node_ref();
    let textarea_ref = use_node_ref();

    let runtime = use_context::<Runtime>().expect("no ctx found");
    let presenter = CreateTaskForm {
        runtime: runtime.clone(),
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

    let onsubmit = {
        let close_modal = close_modal.clone();
        let presenter = presenter.clone();
        let textarea_ref = textarea_ref.clone();

        Callback::from(move |e| {
            let close_modal = close_modal.clone();
            let presenter = presenter.clone();

            let description = textarea_ref
                .cast::<HtmlTextAreaElement>()
                .expect("no modal found")
                .value();

            wasm_bindgen_futures::spawn_local(async move {
                presenter.onsubmit(description).await;
                close_modal.emit(e)
            });
        })
    };

    html! {
        <>
            <ion-button id="create-task-modal">{&presenter.title()}</ion-button>
            <ion-modal ref={modal_ref} trigger="create-task-modal">
                <ion-header>
                <ion-toolbar>
                    <ion-buttons slot="start">
                    <ion-button onclick={&close_modal}>{"Cancel"}</ion-button>
                    </ion-buttons>
                    <ion-title>{"Welcome"}</ion-title>
                    <ion-buttons slot="end">
                    <ion-button strong="true" onclick={&onsubmit}>{"Confirm"}</ion-button>
                    </ion-buttons>
                </ion-toolbar>
                </ion-header>
                <ion-content class="ion-padding">
                <ion-item>
                    <ion-textarea ref={textarea_ref} label="Enter your name" label-placement="stacked" placeholder="Your name" />
                </ion-item>
                </ion-content>
            </ion-modal>
        </>
    }
}
