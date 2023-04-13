use super::*;

#[function_component(Health)]
pub fn health() -> Html {
    html! {
        <div class="min-h-screen min-w-screen bg-zinc-900 text-zinc-100 flex">
            <h1 class="text-xl">{ "Hello World!" }</h1>
        </div>
    }
}