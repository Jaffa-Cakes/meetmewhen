#[cfg(target_arch = "wasm32")]
use super::api;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local as wait;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="min-h-screen min-w-screen bg-zinc-900 text-zinc-100 flex flex-col justify-center">
            <div class="flex justify-around">
                <div class="bg-zinc-800 p-5 pt-4 rounded-2xl">
                    <span class="text-xl mr-5">{ "Type:" }</span>
                    <select class="rounded bg-zinc-900 text-zinc-100 p-2 pt-1 cursor-pointer">
                        <option>{ "Dates" }</option>
                        <option>{ "Days" }</option>
                    </select>

                    <div class="flex gap-2 mt-4">
                        <div>
                            <span class="block text-center">{ "M" }</span>
                            <div class="w-6 border bg-zinc-300 h-36 cursor-pointer hover:bg-zinc-500" />
                        </div>
                        <div>
                            <span class="block text-center">{ "T" }</span>
                            <div class="w-6 border bg-zinc-300 h-36 cursor-pointer hover:bg-zinc-500" />
                        </div>
                        <div>
                            <span class="block text-center">{ "W" }</span>
                            <div class="w-6 border bg-zinc-300 h-36 cursor-pointer hover:bg-zinc-500" />
                        </div>
                        <div>
                            <span class="block text-center">{ "T" }</span>
                            <div class="w-6 border bg-zinc-300 h-36 cursor-pointer hover:bg-zinc-500" />
                        </div>
                        <div>
                            <span class="block text-center">{ "F" }</span>
                            <div class="w-6 border bg-zinc-300 h-36 cursor-pointer hover:bg-zinc-500" />
                        </div>
                        <div>
                            <span class="block text-center">{ "S" }</span>
                            <div class="w-6 border bg-zinc-300 h-36 cursor-pointer hover:bg-zinc-500" />
                        </div>
                        <div>
                            <span class="block text-center">{ "S" }</span>
                            <div class="w-6 border bg-zinc-300 h-36 cursor-pointer hover:bg-zinc-500" />
                        </div>
                    </div>
                </div>

                <div class="flex flex-col justify-center">
                    <input class="bg-zinc-800 rounded text-zinc-100 text-center" type="text" placeholder="Event Name"/>

                    <button class="bg-zinc-800 rounded text-zinc-100 p-2 pt-1 mt-4 cursor-pointer hover:bg-zinc-700" type="submit">{ "Create Event" }</button>
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
