// use super::create_task_modal::CreateTaskModal;
use super::current_task::CurrentTaskView;
use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    html! {
      <>
        <ion-header>
            <ion-toolbar>
                <ion-title>{"Hello!"}</ion-title>
                <ion-buttons slot="end">
                  // <CreateTaskModal />
                </ion-buttons>
            </ion-toolbar>
          </ion-header>
          <ion-content>
            <CurrentTaskView />
          </ion-content>
      </>
    }
}
