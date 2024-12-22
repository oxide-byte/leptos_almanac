// use wasm_bindgen::JsCast;
// use web_sys::HtmlElement;
use wasm_bindgen_test::*;
use leptos::prelude::*;
use leptos::task::tick;
use wasm_bindgen::JsCast;
use crate::CounterButton;

wasm_bindgen_test_configure!(run_in_browser);

// https://rustwasm.github.io/book/reference/debugging.html
// https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html
// https://book.leptos.dev/testing.html

#[wasm_bindgen_test]
async fn button_click()  {

    mount_to_body(CounterButton);
    let document = document();
    let div = document.query_selector("div").unwrap().unwrap();

    assert_eq!(div.inner_html(), "<button class=\"bg-blue-700 hover:bg-blue-800 px-20 py-3 text-white rounded-lg\">0</button>");

    let button = div
        .query_selector("button")
        .unwrap()
        .unwrap()
        .unchecked_into::<web_sys::HtmlElement>();

    button.click();
    tick().await;

    let div = document.query_selector("div").unwrap().unwrap();
    assert_eq!(div.inner_html(), "<button class=\"bg-blue-700 hover:bg-blue-800 px-20 py-3 text-white rounded-lg\">1</button>");


}