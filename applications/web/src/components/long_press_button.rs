use yew::prelude::*;

use crate::hooks::use_long_press;

#[derive(Properties, PartialEq)]
pub struct LongPressButtonProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or("primary".into())]
    pub color: AttrValue,

    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    #[prop_or("round".into())]
    pub shape: AttrValue,

    #[prop_or("large".into())]
    pub size: AttrValue,
}

#[function_component]
pub fn LongPressButton(props: &LongPressButtonProps) -> Html {
    let button_ref = use_long_press(props.onclick.clone(), 2_000);

    html! {
        <ion-button
            class="longpress-button"
            color={&props.color}
            ref={button_ref}
            shape={&props.shape}
            size={&props.size}
        >
            <div class="longpress-success">
                <ion-icon color="success" name="checkmark-outline"></ion-icon>
            </div>
            {props.children.clone()}
        </ion-button>
    }
}
