use crate::home::Home;
use interactions::services::TodoListStore;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct AppContext {
    pub runtime: Rc<dyn TodoListStore>,
}

impl PartialEq for AppContext {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[function_component]
pub fn App(context: &AppContext) -> Html {
    html! {
        <ContextProvider<AppContext> context={(*context).clone()}>
            <Home />
        </ContextProvider<AppContext>>
    }
}

pub fn render(runtime: Rc<dyn TodoListStore>) {
    let ctx = AppContext { runtime };
    yew::Renderer::<App>::with_props(ctx).render();
}
