use super::*;

#[derive(PartialEq)]
pub enum Type {
    Button,
    Submit,
    // Reset,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::Button => write!(f, "button"),
            Type::Submit => write!(f, "submit"),
            // Type::Reset => write!(f, "reset"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    pub children: Children,
    #[prop_or(Type::Button)]
    pub r#type: Type,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn Button(props: &Props) -> Html {
    let Props {
        class,
        children,
        r#type,
        onclick,
    } = props;

    html! {
        <button class={classes!("bg-zinc-800", "rounded", "text-zinc-100", "p-2", "pt-1", "mt-4", "cursor-pointer", "hover:bg-zinc-700", class.clone())} type={r#type.to_string()} {onclick}>
            { for children.iter() }
        </button>
    }
}
