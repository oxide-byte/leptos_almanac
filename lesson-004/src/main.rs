use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::logging::log;

#[component]
pub fn CounterButton(counter: ReadSignal<i32>, on_click: impl FnMut(MouseEvent)+ 'static) -> impl IntoView {
    let button_class = "bg-blue-700 hover:bg-blue-800 px-20 py-3 text-white rounded-lg";

    view! {
        <button on:click=on_click class=button_class>
            {counter}
        </button>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (counter, set_counter) = signal(0);
    let on_click = move |_| {
        log!("Click {:?}", counter.get());
        set_counter.update(|counter| *counter += 1);
    };
    view! {
        <CounterButton
            counter=counter
            on_click=on_click
        />
    }
}


fn main() {
    mount_to_body(App)
}