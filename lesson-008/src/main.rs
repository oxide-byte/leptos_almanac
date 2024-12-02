use leptos::prelude::*;

#[component]
pub fn ShowList() -> impl IntoView {
    let count = [1,2,3,4,5,6];
    view! {
        Hello
        <br/>
        <ul>
            {
                count.into_iter()
                .map(|n| view! { <li> - {n} - </li>})
                .collect_view()
            }
        </ul>
    }
}

fn main() {
    mount_to_body(ShowList);
}