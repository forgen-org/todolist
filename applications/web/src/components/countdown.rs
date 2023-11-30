use gloo_timers::callback::Interval;
use std::cmp::max;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CountdownProps {
    pub seconds: i64,
}

#[function_component]
pub fn Countdown(props: &CountdownProps) -> Html {
    let seconds = use_state(|| props.seconds);

    {
        let seconds = seconds.clone();
        use_effect_with((), move |_| {
            let mut i = 0;
            let interval = Interval::new(1000, move || {
                i += 1;
                let new_seconds = *seconds.clone() - i;
                seconds.set(new_seconds);
            });
            || {
                interval.forget();
            }
        });
    }

    html! {
        format_seconds(&seconds)
    }
}

fn format_seconds(seconds: &i64) -> String {
    let seconds = max(seconds, &0);
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    format!("{:02}m {:02}s", minutes, seconds)
}
