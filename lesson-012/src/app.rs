use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use reactive_stores::{Patch, Store};

#[derive(Clone, Debug, Default, Store, Patch)]
pub struct GlobalState {
    pub counter: i32,
}

#[component]
pub fn AddComponent() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let counter = state.counter();

    let on_click = move |_| {
        let x = counter.get();
        counter.patch(x + 1);
        console_log(format!("Counter: {}", counter.get()).as_str());
    };

    view! {
        <button on:click=on_click> Click </button>
    }
}

#[component]
pub fn ShowComponent() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();

    view! {
        <p> { move || state.counter().get() } </p>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(Store::new(GlobalState::default()));
    view! {
        <div>
            <div> <AddComponent/> </div>
            <div> <ShowComponent/> </div>
        </div>
    }
}