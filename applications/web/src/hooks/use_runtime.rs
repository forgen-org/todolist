use crate::runtime::Runtime;
use yew::prelude::*;

impl PartialEq for Runtime {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct RuntimeProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn RuntimeProvider(props: &RuntimeProviderProps) -> Html {
    let runtime = Runtime {};

    html! {
        <ContextProvider<Runtime> context={runtime}>
            {props.children.clone()}
        </ContextProvider<Runtime>>
    }
}

#[hook]
pub fn use_runtime() -> Runtime {
    use_context::<Runtime>().expect("no ctx found")
}
