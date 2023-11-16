use crate::components::{CreateTaskModal, StickyNote};
use crate::hooks::{RuntimeProvider, TaskStateProvider};
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <RuntimeProvider>
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
                    </ion-content>
                    <ion-footer>
                        <ion-toolbar />
                    </ion-footer>
                </ion-app>
            </TaskStateProvider>
        </RuntimeProvider>
    }
}

impl App {
    pub fn render() {
        yew::Renderer::<App>::new().render();
    }
}
