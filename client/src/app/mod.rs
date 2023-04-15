#[cfg(target_arch = "wasm32")]
use super::api;

use api_types::prelude::*;

use std::collections::HashMap;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

mod atoms;
mod components;
mod pages;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local as wait;
#[cfg(not(target_arch = "wasm32"))]
fn wait<F>(future: F)
where
    F: core::future::Future<Output = ()> + 'static,
{
    futures::executor::block_on(future);
}

////////////////////////

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Index,
    #[not_found]
    #[at("/not-found")]
    NotFound,
}

////////////////////////

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

////////////////////////

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

////////////////////////

fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => {
            html! { <pages::Index /> }
        }
        Route::NotFound => {
            html! { <pages::NotFound /> }
        }
    }
}
