use super::*;

////////////////////////
/// Week Component

#[derive(PartialEq, Properties)]
pub struct WeekProps {
    pub selected: Vec<time::Weekday>,
    pub toggle: Callback<time::Weekday>,
}

#[function_component]
pub fn Week(props: &WeekProps) -> Html {
    let WeekProps { selected, toggle } = props;

    let selected = &*selected;
    let selected = selected.clone();

    html! {
        <div class="flex gap-2 mt-4">
            <Day day={time::Weekday::Monday} selected={selected.clone()} {toggle} />
            <Day day={time::Weekday::Tuesday} selected={selected.clone()} {toggle} />
            <Day day={time::Weekday::Wednesday} selected={selected.clone()} {toggle} />
            <Day day={time::Weekday::Thursday} selected={selected.clone()} {toggle} />
            <Day day={time::Weekday::Friday} selected={selected.clone()} {toggle} />
            <Day day={time::Weekday::Saturday} selected={selected.clone()} {toggle} />
            <Day day={time::Weekday::Sunday} selected={selected} {toggle} />
        </div>
    }
}

////////////////////////
/// Weekday Selector
// The individual selectable bar representing a single day of the week

#[derive(Properties, PartialEq)]
struct DayProps {
    day: time::Weekday,
    selected: Vec<time::Weekday>,
    toggle: Callback<time::Weekday>,
}

#[function_component]
fn Day(props: &DayProps) -> Html {
    let DayProps {
        day,
        selected,
        toggle,
    } = props;

    let selected = &*selected;
    let selected = selected.clone();

    let onclick = {
        let day = day.clone();
        let toggle = toggle.clone();

        Callback::from(move |_| {
            toggle.emit(day.clone());
        })
    };

    let day_name = match day {
        time::Weekday::Monday => "M".to_string(),
        time::Weekday::Tuesday => "T".to_string(),
        time::Weekday::Wednesday => "W".to_string(),
        time::Weekday::Thursday => "T".to_string(),
        time::Weekday::Friday => "F".to_string(),
        time::Weekday::Saturday => "S".to_string(),
        time::Weekday::Sunday => "S".to_string(),
    };

    match selected.contains(day) {
        true => {
            html! {
                <div>
                    <span class="block text-center">{ day_name }</span>
                    <div class="w-6 border bg-blue-800 hover:bg-blue-950 h-36 cursor-pointer border-blue-800" {onclick} />
                </div>
            }
        }
        false => {
            html! {
                <div>
                    <span class="block text-center">{ day_name }</span>
                    <div class="w-6 border bg-red-800 hover:bg-red-950 h-36 cursor-pointer border-blue-800" {onclick} />
                </div>
            }
        }
    }
}
