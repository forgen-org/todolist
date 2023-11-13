use crate::home::Home;
use interactions::presenters::CreateTaskForm;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct AppContext {
    pub create_task_form: Rc<CreateTaskForm>,
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

pub fn render(ctx: AppContext) {
    yew::Renderer::<App>::with_props(ctx).render();
}
