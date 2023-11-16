use interactions::presenters::TaskState;
use yew::prelude::*;

pub type TaskStateHandle = UseStateHandle<TaskState>;

#[derive(Debug, PartialEq, Properties)]
pub struct TaskStateProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn TaskStateProvider(props: &TaskStateProviderProps) -> Html {
    let msg = use_state(|| TaskState::default());

    html! {
        <ContextProvider<TaskStateHandle> context={msg}>
            {props.children.clone()}
        </ContextProvider<TaskStateHandle>>
    }
}

#[hook]
pub fn use_task_state() -> TaskStateHandle {
    use_context::<TaskStateHandle>().expect("no ctx found")
}
