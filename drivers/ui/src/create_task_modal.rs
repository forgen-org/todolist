use crate::app::AppContext;
use crate::ionic::*;
use gloo_console::log;
use interactions::presenters::CreateTaskForm;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[function_component]
pub fn CreateTaskModal() -> Html {
    let modal_ref = use_node_ref();

    let presenter = use_context::<AppContext>()
        .expect("no ctx found")
        .create_task_form;

    let close_modal = {
        let modal_ref = modal_ref.clone();

        Callback::from(move |_| {
            modal_ref
                .cast::<HTMLIonModalElement>()
                .expect("no modal found")
                .dismiss(JsValue::undefined(), None);
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
                    <ion-button strong="true" onclick={&close_modal}>{"Confirm"}</ion-button>
                    </ion-buttons>
                </ion-toolbar>
                </ion-header>
                <ion-content class="ion-padding">
                <ion-item>
                    <ion-input label="Enter your name" label-placement="stacked" type="text" placeholder="Your name"></ion-input>
                </ion-item>
                </ion-content>
            </ion-modal>
        </>
    }
}
