use super::*;

////////////////////////
/// Routing Table

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Routes {
    #[at("/")]
    Index,
    #[not_found]
    #[at("/not-found")]
    NotFound,
    #[at("/:id")]
    Event { id: String },
}

////////////////////////
/// Routing Switch
// Determine which page to render based on the current route

fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Index => html! { <pages::Index /> },
        Routes::NotFound => html! { <pages::NotFound /> },
        Routes::Event { id } => html! { <pages::Event id={id} /> },
    }
}

////////////////////////
/// Client Side Renderer

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Routes> render={switch} />
        </BrowserRouter>
    }
}

////////////////////////
/// Server Side Renderer

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
            <Switch<Routes> render={switch} />
        </Router>
    }
}
