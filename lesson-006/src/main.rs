use leptos::html::*;
use leptos::prelude::*;

#[component]
pub fn HelloUser() -> impl IntoView {

    let (name, set_name) = signal("Unknown".to_string());

    view! {
        <div>
            <h1 class="text-4xl"> Controlled </h1>
            <h2 class="text-2xl">
                Hello {name}
            </h2>
            <p> Enter your name:
            <input type="text"
                on:input:target=move |ev| {
            set_name.set(ev.target().value());
            }
            prop:value=name
            />
            </p>
        </div>
    }
}

fn main() {
    mount_to_body(HelloUser);
}