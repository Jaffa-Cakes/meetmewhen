#[cfg(target_arch = "wasm32")]
use super::api;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local as wait;
use yew::prelude::*;

use health::Health;

mod health;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="min-h-screen min-w-screen bg-zinc-900 text-zinc-100 flex">
            <Health />
        </div>
    }
}