use gloo_console::log;
use gloo_timers::callback::Timeout;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use yew::prelude::*;

#[hook]
pub fn use_long_press(onclick: Callback<MouseEvent>, duration_millis: u32) -> NodeRef {
    // Refs
    let node_ref = use_node_ref();

    // States
    let timeout = use_state(|| None);

    // Callbacks
    let onpress = {
        let node_ref = node_ref.clone();
        let timeout = timeout.clone();
        Callback::from(move |_: TouchEvent| {
            let element = node_ref
                .cast::<HtmlElement>()
                .expect("Unable to cast to HtmlElement");

            element.class_list().add_1("longpress").unwrap();

            let onclick = onclick.clone();
            timeout.set(Some(Timeout::new(duration_millis, move || {
                onclick.emit(MouseEvent::new("click").unwrap());
            })));
        })
    };
    let onrelease = {
        // let loader = loader.clone();
        let node_ref = node_ref.clone();
        let timeout = timeout.clone();
        Callback::from(move |_: TouchEvent| {
            let element = node_ref
                .cast::<HtmlElement>()
                .expect("Unable to cast to HtmlElement");
            element.class_list().remove_1("longpress").unwrap();

            timeout.set(None);
        })
    };

    // Closures
    let press_closure = use_memo((), move |_| {
        Closure::<dyn Fn()>::new(move || {
            onpress.emit(TouchEvent::new("touchstart").unwrap());
        })
    });
    let release_closure = use_memo((), move |_| {
        Closure::<dyn Fn()>::new(move || {
            onrelease.emit(TouchEvent::new("touchend").unwrap());
        })
    });

    // Effects
    use_effect_with(node_ref.clone(), move |node_ref| {
        let element = node_ref
            .cast::<HtmlElement>()
            .expect("Unable to cast to HtmlElement");

        element.set_onclick(None);
        element.set_onmousedown(Some((*press_closure.clone()).as_ref().unchecked_ref()));
        element.set_onmouseleave(Some((*release_closure.clone()).as_ref().unchecked_ref()));
        element.set_onmouseup(Some((*release_closure.clone()).as_ref().unchecked_ref()));
        element.set_ontouchend(Some((*release_closure.clone()).as_ref().unchecked_ref()));
        element.set_ontouchstart(Some((*press_closure.clone()).as_ref().unchecked_ref()));

        || ()
    });

    node_ref
}
