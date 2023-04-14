#[cfg(target_arch = "wasm32")]
use super::api;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local as wait;
use yew::prelude::*;

mod atoms;
mod components;

#[function_component]
pub fn App() -> Html {
    html! {
        <div class="min-h-screen min-w-screen bg-zinc-900 text-zinc-100 flex flex-col justify-center">
            <div class="flex justify-around">
                <div class="bg-zinc-800 p-5 pt-4 rounded-2xl">
                    <span class="text-xl mr-5">{ "Type:" }</span>
                    <select class="rounded bg-zinc-900 text-zinc-100 p-2 pt-1 cursor-pointer">
                        <option>{ "Dates" }</option>
                        <option>{ "Days" }</option>
                    </select>

                    <components::Week />
                </div>

                <div class="flex flex-col justify-center">
                    <atoms::InputText />

                    <atoms::Button r#type={atoms::ButtonType::Submit}>{ "Create Event" }</atoms::Button>
                </div>

                <div class="bg-zinc-800 p-5 pt-4 rounded-2xl">
                    <span class="text-xl mr-5">{ "No Earlier Than:" }</span>
                    <select class="rounded bg-zinc-900 text-zinc-100 p-2 pt-1 cursor-pointer">
                        <option>{ "9:00 AM" }</option>
                        <option>{ "10:00 AM" }</option>
                    </select>

                    <br />

                    <span class="text-xl mr-5">{ "No Later Than:" }</span>
                    <select class="rounded bg-zinc-900 text-zinc-100 p-2 pt-1 cursor-pointer">
                        <option>{ "5:00 PM" }</option>
                        <option>{ "6:00 PM" }</option>
                    </select>
                </div>
            </div>
        </div>
    }
}
