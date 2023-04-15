use super::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component]
pub fn Event(props: &Props) -> Html {
    let Props { id } = props;

    html! {
        <div class="min-h-screen min-w-screen bg-zinc-900 text-zinc-100 flex flex-col justify-center">
            <div class="flex justify-center">
                <span class="text-4xl">{ "Event: " }{ id }</span>
            </div>
        </div>
    }
}
