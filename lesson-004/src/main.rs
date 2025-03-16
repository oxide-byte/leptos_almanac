use leptos::ev;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::logging::log;
use leptos::html::{ button };

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
pub fn CounterButtonBuilder(counter: ReadSignal<i32>, on_click: impl FnMut(MouseEvent)+ 'static) -> impl IntoView {
    let button_class = "bg-green-700 hover:bg-green-800 px-20 py-3 text-white rounded-lg";

    let button = button()
        .class(button_class)
        .on(ev::click, on_click)
        .child(counter);

    button.into_view()
}


#[component]
pub fn App() -> impl IntoView {
    let (counter, set_counter) = signal(0);
    let on_click = move |_| {
        log!("Click {:?}", counter.get());
        set_counter.update(|counter| *counter += 1);
    };
    view! {
        <div>
        <CounterButton
            counter=counter
            on_click=on_click
        />
        <br/>
        <CounterButtonBuilder
            counter=counter
            on_click=on_click
        />
        </div>
    }
}


fn main() {
    mount_to_body(App)
}