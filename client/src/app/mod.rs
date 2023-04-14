#[cfg(target_arch = "wasm32")]
use super::api;

use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local as wait;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

mod atoms;
mod components;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <Switch<Route> render={switch} />
        </Router>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <NotFound /> }
        }
    }
}

#[function_component]
pub fn Home() -> Html {
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
                    <label>{ "Event Name: " }<atoms::InputText /></label>

                    <atoms::Button r#type={atoms::ButtonType::Submit}>{ "Create Event" }</atoms::Button>

                    <Link<Route> to={Route::NotFound}>
                        { "Not Found Page" }
                    </Link<Route>>
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

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div class="min-h-screen min-w-screen bg-zinc-900 text-zinc-100 flex flex-col justify-center">
            <div class="flex justify-center">
                <span class="text-4xl">{ "Not Found" }</span>
            </div>
        </div>
    }
}
