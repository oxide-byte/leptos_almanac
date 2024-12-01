use leptos::prelude::*;

#[component]
pub fn CounterButton() -> impl IntoView {
    let (counter, set_counter) = signal(0);
    let button_class = "bg-blue-700 hover:bg-blue-800 px-20 py-3 text-white rounded-lg";
    let on_click = move |_| {
        set_counter.update(|counter| *counter += 1);
    };

    view! {
        <button on:click=on_click class=button_class>
            {counter}
        </button>
    }
}

fn main() {
    mount_to_body(|| view! {
        <CounterButton/>
    })
}