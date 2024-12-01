mod button_test;

use leptos::prelude::*;

#[component]
pub fn CounterButton() -> impl IntoView {

    let (counter, set_counter) = signal(0);

    view! {
        <div>
        <button
            on:click=move |_| set_counter.update(|counter| *counter += 1)
            class="bg-blue-700 hover:bg-blue-800 px-20 py-3 text-white rounded-lg"
        >
            {counter}
        </button>
        </div>
    }
}

fn main() {
    mount_to_body(CounterButton)
}