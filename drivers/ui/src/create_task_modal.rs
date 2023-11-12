use crate::app::AppContext;
use crate::ionic::*;
use interactions::presenters::CreateTaskForm;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[function_component]
pub fn CreateTaskModal() -> Html {
    // let modal_ref = use_node_ref();

    let runtime = use_context::<AppContext>().expect("no ctx found").runtime;
    let presenter = CreateTaskForm { runtime };

    // let close_modal = {
    //     let modal = modal_ref.cast::<HTMLIonModalElement>().unwrap();

    //     Callback::from(move |_| {
    //         modal.dismiss(JsValue::undefined(), None);
    //     })
    // };

    html! {
        <>
            <ion-button>{&presenter.title()}</ion-button>
            <ion-modal trigger="open-modal">
                <ion-header>
                <ion-toolbar>
                    <ion-buttons slot="start">
                    <ion-button>{"Cancel"}</ion-button>
                    </ion-buttons>
                    <ion-title>{"Welcome"}</ion-title>
                    <ion-buttons slot="end">
                    <ion-button strong="true">{"Confirm"}</ion-button>
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
