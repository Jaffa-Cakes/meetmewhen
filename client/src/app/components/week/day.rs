use super::*;

#[derive(PartialEq)]
pub enum Name {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Name::Monday => write!(f, "M"),
            Name::Tuesday => write!(f, "T"),
            Name::Wednesday => write!(f, "W"),
            Name::Thursday => write!(f, "T"),
            Name::Friday => write!(f, "F"),
            Name::Saturday => write!(f, "S"),
            Name::Sunday => write!(f, "S"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: Name,
}

#[function_component]
pub fn Day(props: &Props) -> Html {
    let Props { name } = props;

    html! {
        <div>
            <span class="block text-center">{ name.to_string() }</span>
            <div class="w-6 border bg-zinc-300 h-36 cursor-pointer hover:bg-zinc-500" />
        </div>
    }
}
