use super::task_buttons::TaskButtons;
use super::task_error::TaskError;
use super::task_sticky_note::TaskStickyNote;
use crate::hooks::TaskStateProvider;
use yew::prelude::*;

#[function_component]
pub fn TaskScreen() -> Html {
    html! {
        <TaskStateProvider>
            <ion-app>
                <ion-header>
                    <ion-toolbar>
                        <ion-title>{"TodoList"}</ion-title>
                    </ion-toolbar>
                </ion-header>
                <ion-content class="ion-padding" scroll-y="false">
                    <main>
                        <TaskStickyNote />
                    </main>
                    <TaskButtons />
                    <TaskError />
                </ion-content>
                <ion-footer>
                    <ion-toolbar />
                </ion-footer>
            </ion-app>
        </TaskStateProvider>
    }
}
