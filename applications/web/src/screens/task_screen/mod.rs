mod task_buttons;
mod task_create_button;
mod task_error;
mod task_sticky_note;

use crate::hooks::TaskStateProvider;
use task_buttons::TaskButtons;
use task_error::TaskError;
use task_sticky_note::TaskStickyNote;
use yew::prelude::*;

#[function_component]
pub fn TaskScreen() -> Html {
    html! {
        <TaskStateProvider>
            <ion-app>
                <ion-content class="ion-padding" scroll-y="false">
                    <main>
                        <TaskStickyNote />
                        <TaskButtons />
                        <TaskError />
                    </main>
                </ion-content>
            </ion-app>
        </TaskStateProvider>
    }
}
