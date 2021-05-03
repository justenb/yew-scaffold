#![recursion_limit = "10000"]
mod app;
mod containers;
mod pages;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::utils::document;

#[wasm_bindgen(start)]
pub fn start_app() {
    let div = document().query_selector("#app").unwrap().unwrap();
    App::<app::App>::new().mount(div);
}
