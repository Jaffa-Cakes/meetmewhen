use super::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    pub children: Children,
    #[prop_or_default]
    pub r#ref: NodeRef,
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

#[function_component]
pub fn Select(props: &Props) -> Html {
    let Props {
        class,
        children,
        r#ref,
        onchange,
    } = props;

    html! {
        <select class={classes!("rounded", "bg-zinc-900", "text-zinc-100", "p-2", "pt-1", "cursor-pointer", class.clone())} ref={r#ref} {onchange}>
            { for children.iter() }
        </select>
    }
}
