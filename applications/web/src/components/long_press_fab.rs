use gloo_console::log;
use gloo_timers::callback::Timeout;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LongPressFabProps {
    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub color: Option<AttrValue>,

    #[prop_or(2_000)]
    pub duration: u32,

    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    #[prop_or_default]
    pub size: Option<AttrValue>,
}

#[function_component]
pub fn LongPressFab(props: &LongPressFabProps) -> Html {
    let is_pressed = use_state(|| false);

    {
        let duration = props.duration.clone();
        let onclick = props.onclick.clone();
        use_effect_with(is_pressed.clone(), move |is_pressed| {
            let duration = duration.clone();
            let is_pressed = is_pressed.clone();
            let onclick = onclick.clone();

            let timeout = Timeout::new(duration, move || {
                if (*is_pressed).clone() == true {
                    onclick.emit(MouseEvent::new("click").unwrap());
                }
            });
            || {
                timeout.cancel();
            }
        });
    }

    let press = {
        let is_pressed = is_pressed.clone();
        move || {
            is_pressed.set(true);
        }
    };

    let release = {
        let is_pressed = is_pressed.clone();
        move || {
            is_pressed.set(false);
        }
    };

    // Mouse Events
    let onclickstart = {
        let press = press.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            press();
        })
    };
    let onclickend = {
        let release = release.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            release();
        })
    };

    // Touch Events
    let ontouchstart = {
        let press = press.clone();
        Callback::from(move |event: TouchEvent| {
            event.prevent_default();
            press();
        })
    };
    let ontouchend = {
        let release = release.clone();
        Callback::from(move |event: TouchEvent| {
            event.prevent_default();
            release();
        })
    };

    html! {
        <ion-fab-button
            longpress={(*is_pressed).to_string()}
            color={&props.color}
            onmousedown={onclickstart.clone()}
            onmouseleave={onclickend.clone()}
            onmouseup={onclickend.clone()}
            ontouchend={ontouchend.clone()}
            ontouchstart={ontouchstart.clone()}
            size={&props.size}
        >
            <div class="loader" />
            {props.children.clone()}
        </ion-fab-button>
    }
}
