use yew::prelude::*;

use crate::hooks::use_long_press;

use super::Countdown;

#[derive(Properties, PartialEq)]
pub struct LongPressCountdownProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    pub seconds: i64,
}

#[function_component]
pub fn LongPressCountdown(props: &LongPressCountdownProps) -> Html {
    let button_ref = use_long_press(props.onclick.clone(), 2_000);

    html! {
        <ion-button
            class="longpress-countdown"
            color="dark"
            ref={button_ref}
            shape="round"
            size="large"
        >
            <span>
                <Countdown seconds={props.seconds} />
            </span>
            <div class="longpress-success">
                <ion-icon color="success" name="checkmark-outline"></ion-icon>
            </div>
        </ion-button>
    }
}
