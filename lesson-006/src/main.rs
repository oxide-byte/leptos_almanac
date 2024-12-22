use leptos::ev::MouseEvent;
use leptos::logging::log;
use leptos::prelude::*;


#[component]
pub fn ClickModal(on_close_modal: impl FnMut(MouseEvent)+ 'static) -> impl IntoView {
    let button_class = "bg-green-700 hover:bg-green-800 px-20 py-3 text-white rounded-lg";

    view! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-gray-900 bg-opacity-60">
            <div class="block rounded-lg bg-white w-1/5 p-4 shadow-[0_2px_15px_-3px_rgba(0,0,0,0.07),0_10px_20px_-2px_rgba(0,0,0,0.04)] z-70">

                <button on:click=on_close_modal class=button_class>
                    Close Modal
                </button>

            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let button_class = "bg-blue-700 hover:bg-blue-800 px-20 py-3 text-white rounded-lg";
    let show_modal: RwSignal<bool> = RwSignal::new(false);
    let on_click = move |_| {
        show_modal.set(true);
    };

    let close_modal = move |_| {
        log!("Close Modal");
        show_modal.set(false);
    };

    view! {
        <button on:click=on_click class=button_class>
            Show Modal
        </button>

        <Show when = move || show_modal.get()>
            <ClickModal on_close_modal=close_modal>
            </ClickModal>
        </Show>
    }
}

fn main() {
    mount_to_body(App)
}