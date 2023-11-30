use crate::hooks::{RuntimeProvider, TaskStateProvider};
use crate::screens::TaskScreen;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <RuntimeProvider>
            <TaskStateProvider>
                <TaskScreen />
            </TaskStateProvider>
        </RuntimeProvider>
    }
}

impl App {
    pub fn render() {
        yew::Renderer::<App>::new().render();
    }
}
