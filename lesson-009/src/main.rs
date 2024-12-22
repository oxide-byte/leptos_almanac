use leptos::prelude::*;

#[component]
pub fn ShowList() -> impl IntoView {
    let count = [1,2,3,4,5,6];
    let (count2, set_count2) = signal(vec!(1,2,3,4,5,6));
    view! {
        <br/>
        Without change detection
        <br/>
        <ul>
            {
                count.into_iter()
                .map(|n| view! { <li> - {n} - </li>})
                .collect_view()
            }
        </ul>

        <br/>
        With change detection
        <br/>
        <ul>
            <For
                each=move || count2.get()
                key=|state| state.clone()
                let:child>
                <li> - {child} - </li>
            </For>
        </ul>
    }
}

fn main() {
    mount_to_body(ShowList);
}