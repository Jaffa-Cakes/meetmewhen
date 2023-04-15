use super::*;

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
