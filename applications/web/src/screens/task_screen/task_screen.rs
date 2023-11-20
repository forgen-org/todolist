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
                <ion-content class="ion-padding" scroll-y="false">
                    <div style="display:flex; flex-direction: column; height: 100%;">
                        <div style="flex-grow:1;">
                            <TaskStickyNote />
                        </div>
                        <TaskButtons />
                    </div>
                    <TaskError />
                </ion-content>
            </ion-app>
        </TaskStateProvider>
    }
}
