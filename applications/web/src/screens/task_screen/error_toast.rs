use crate::hooks::use_task_state;

use yew::prelude::*;

#[function_component]
pub fn ErrorToast() -> Html {
    let is_open = use_state(|| false);
    let task_state = use_task_state();

    {
        let error = task_state.error.clone();
        let is_open = is_open.clone();
        use_effect_with(error.clone(), move |_| {
            if error.clone().is_some() {
                is_open.set(true);
            }
            || ()
        });
    }

    let error_msg = task_state.error.clone().unwrap_or("".to_string());
    let is_open_attr = is_open.clone().to_string();

    html! {
        <ion-toast color="danger" duration="5000" is-open={is_open_attr} message={error_msg} />
    }
}
