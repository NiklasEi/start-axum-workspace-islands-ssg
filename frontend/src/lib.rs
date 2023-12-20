// This import is required for `wasm_bindgen` to do it's thing with our app
#[allow(unused_imports)]
use app::*;

use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();

    leptos_dom::HydrationCtx::stop_hydrating();
}
