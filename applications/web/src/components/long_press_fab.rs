use yew::prelude::*;

use crate::hooks::use_long_press;

#[derive(Properties, PartialEq)]
pub struct LongPressFabProps {
    #[prop_or("primary".into())]
    pub color: AttrValue,

    #[prop_or(2_000)]
    pub duration: u32,

    pub icon: AttrValue,

    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    #[prop_or_default]
    pub size: Option<AttrValue>,
}

#[function_component]
pub fn LongPressFab(props: &LongPressFabProps) -> Html {
    let button_ref = use_long_press(props.onclick.clone(), 2_000);

    html! {
        <ion-fab-button
            color={&props.color}
            ref={button_ref}
            size={props.size.clone()}
        >
            <ion-icon name={&props.icon}></ion-icon>
            <div class="longpress-background">
                <ion-icon color={&props.color} name={&props.icon}></ion-icon>
            </div>
        </ion-fab-button>
    }
}
