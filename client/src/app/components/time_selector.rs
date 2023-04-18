use super::*;

////////////////////////
/// Simplified Time Representation

#[derive(PartialEq, Clone)]
enum BasicWhen {
    Date(time::Date),
    Day(time::Weekday),
}

////////////////////////
/// Time Selector Component

#[derive(PartialEq, Properties)]
pub struct TimeSelectorProps {
    pub when: api_types::basic_event::When,
    pub no_earlier: time::Time,
    pub no_later: time::Time,
    pub selected: api_types::basic_event::When,
    pub toggle: Callback<api_types::basic_event::When>,
}

#[function_component]
pub fn TimeSelector(props: &TimeSelectorProps) -> Html {
    let TimeSelectorProps {
        when,
        no_earlier,
        no_later,
        selected,
        toggle,
    } = props;

    let slots_count = {
        let no_earlier = {
            let no_earlier = no_earlier.as_hms();
            no_earlier.0 as u64 * 60 + no_earlier.1 as u64
        };

        let no_later = {
            let no_later = no_later.as_hms();
            no_later.0 as u64 * 60 + no_later.1 as u64
        };

        let slots_count = (no_later - no_earlier) / 30;

        (0..slots_count).collect::<Vec<_>>()
    };

    match when {
        api_types::basic_event::When::Date(dates) => {
            html! {
                <>
                    {
                        for dates.iter().map(|date| {
                            let date = BasicWhen::Date(*date);
                            html! {
                                <div class="flex flex-col p-4">
                                    <div class="border border-b-0 bg-zinc-700 w-12">
                                        {
                                            for slots_count.iter().map(|_| {
                                                let selected = &*selected;
                                                let selected = selected.clone();

                                                html! {
                                                    <Slot when={date.clone()} {toggle} {selected} />
                                                }
                                            })
                                        }
                                    </div>
                                </div>
                            }
                        })
                    }
                </>
            }
        }
        api_types::basic_event::When::Day(days) => {
            html! {
                <>
                    {
                        for days.iter().map(|day| {
                            let day = BasicWhen::Day(*day);
                            html! {
                                <div class="flex flex-col p-4">
                                    <div class="border border-b-0 bg-zinc-700 w-12">
                                        {
                                            for slots_count.iter().map(|i| {
                                                let selected = &*selected;
                                                let selected = selected.clone();

                                                html! {
                                                    <Slot when={day.clone()} {toggle} {selected} />
                                                }
                                            })
                                        }
                                    </div>
                                </div>
                            }
                        })
                    }
                </>
            }
        }
    }
}

////////////////////////
/// Time Slot
// The individual time slots for each individual day/date

#[derive(PartialEq, Properties)]
struct SlotProps {
    when: BasicWhen,
    selected: api_types::basic_event::When,
    toggle: Callback<api_types::basic_event::When>,
}

#[function_component]
fn Slot(props: &SlotProps) -> Html {
    let SlotProps {
        when,
        selected,
        toggle,
    } = props;

    let onclick = {
        // let times_selected = times_selected.clone();
        let when = when.clone();
        let toggle = toggle.clone();

        Callback::from(move |_| match when {
            BasicWhen::Date(date) => {
                toggle.emit(api_types::basic_event::When::Date(vec![date]));
            }
            BasicWhen::Day(day) => {
                toggle.emit(api_types::basic_event::When::Day(vec![day]));
            }
        })
    };

    let active = match selected {
        api_types::basic_event::When::Date(dates) => {
            if let BasicWhen::Date(date) = when {
                dates.contains(&date)
            } else {
                panic!("This shouldn't happen!");
            }
        }
        api_types::basic_event::When::Day(days) => {
            if let BasicWhen::Day(day) = when {
                days.contains(&day)
            } else {
                panic!("This shouldn't happen!");
            }
        }
    };

    match active {
        true => {
            html! {
                <div class="h-6 border bg-blue-800 hover:bg-blue-950 border-b border-blue-800" {onclick} />
            }
        }
        false => {
            html! {
                <div class="h-6 border bg-red-800 hover:bg-red-950 border-b border-blue-800" {onclick} />
            }
        }
    }
}
