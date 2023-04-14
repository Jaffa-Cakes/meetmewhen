use super::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub r#ref: NodeRef,
}

#[function_component]
pub fn Input(props: &Props) -> Html {
    let Props { class, r#ref } = props;

    html! {
        <input class={classes!("bg-zinc-800", "rounded", "text-zinc-100", "text-center", class.clone())} type="text" ref={r#ref}/>
    }
}
