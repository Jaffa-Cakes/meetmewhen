use super::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn Input(props: &Props) -> Html {
    let Props { class } = props;

    html! {
        <input class={classes!("bg-zinc-800", "rounded", "text-zinc-100", "text-center", class.clone())} type="text" />
    }
}
