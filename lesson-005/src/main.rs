use leptos::prelude::*;

#[component]
pub fn UpDownBySignal(signal: WriteSignal<i32>) -> impl IntoView {
    let button_class = "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-full text-sm p-2.5 text-center items-center mr-5";

    view! {
        <div>
            <button class=button_class
                    on:click={move |_| {signal.update(|x| *x = *x + 1)}}
            > + </button>

            <button class=button_class
                    on:click={move |_| {signal.update(|x| *x = *x - 1)}}
            > - </button>
        </div>
    }
}

#[component]
pub fn UpDownByCallback<FU, FD>(current_value:ReadSignal<i32>,
                        on_up:FU, on_down:FD) -> impl IntoView
    where
        FU: Fn(i32) + 'static,
        FD: Fn(i32) + 'static, {
    let button_class = "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-full text-sm p-2.5 text-center items-center mr-5";

    let add_one = move |_| {
        let new = *current_value.read() + 1;
        on_up(new);
    };

    let subtract_one = move |_| {
        let new = *current_value.read() - 1;
        on_down(new);
    };

    view! {
        <div>
            <button class=button_class
                    on:click=add_one
            > + </button>

            <button class=button_class
                    on:click=subtract_one
            > - </button>

            <p> {current_value} </p>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (get_counter, set_counter) = signal(0);

    let on_up_event = move|x| {
        set_counter.set(x);
    };

    let on_down_event = move|x| {
        set_counter.set(x);
    };


    view! {
        <div class="m-10">

            <h1 class="text-4xl">Current Value : {get_counter}</h1>

            <h2 class="text-2xl mt-10">Update by Signal</h2>
            <UpDownBySignal signal=set_counter></UpDownBySignal>

            <h2 class="text-2xl mt-10">Update by Callback</h2>
            <UpDownByCallback
                current_value=get_counter
                on_up=on_up_event
                on_down=on_down_event
            ></UpDownByCallback>
        </div>
    }
}

fn main() {
    mount_to_body(App)
}