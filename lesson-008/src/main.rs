use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::html::*;
use leptos::prelude::*;

#[component]
pub fn HelloUser() -> impl IntoView {
    let button_class = "bg-blue-700 hover:bg-blue-800 px-20 py-3 text-white rounded-lg";

    let (name, set_name) = signal("Unknown".to_string());
    let input_element: NodeRef<html::Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element
            .get()
            .expect("<input> should be mounted")
            .value();
        set_name.set(value);
    };

    view! {
        <div>
            <h1 class="text-4xl"> Uncontrolled </h1>
            <h2 class="text-2xl">
                Hello {name}
            </h2>
            <form on:submit=on_submit>
                <p>
                Enter your name:
                <input type="text"
                    value=name
                    node_ref=input_element
                />
                </p>
                <button type="Submit" class=button_class>
                    Submit
                </button>
            </form>
        </div>
    }
}

fn main() {
    mount_to_body(HelloUser);
}