use crate::{components::home::Home, runtime::Runtime};
use yew::prelude::*;

#[function_component]
pub fn App(runtime: &Runtime) -> Html {
    html! {
        <ContextProvider<Runtime> context={(*runtime).clone()}>
            <Home />
        </ContextProvider<Runtime>>
    }
}
