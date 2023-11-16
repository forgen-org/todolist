pub mod task_screen;

use crate::hooks::TaskStateProvider;
use crate::screens::task_screen::{CreateTaskModal, ErrorToast, StickyNote};
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
                        <div class="sticky-note bg-2">
                            <StickyNote />
                        </div>
                    </main>
                    <CreateTaskModal />
                    <ErrorToast />
                </ion-content>
                <ion-footer>
                    <ion-toolbar />
                </ion-footer>
            </ion-app>
        </TaskStateProvider>
    }
}
