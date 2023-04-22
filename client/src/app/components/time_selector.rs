use std::collections::BTreeMap;

use super::*;

////////////////////////
/// Selected Representation
// (Index, (Date/Day, [Time Slots]))

pub type Selected = BTreeMap<u16, (BasicWhen, Vec<u16>)>;

////////////////////////
/// Generate Selected Representation

pub fn gen_selected(when: &api_types::basic_event::When) -> Selected {
    let mut selected = Selected::new();

    match when {
        api_types::basic_event::When::Date(dates) => {
            for (i, date) in dates.iter().enumerate() {
                let date = BasicWhen::Date(*date);

                selected.insert(i as u16, (date, Vec::new()));
            }
        }
        api_types::basic_event::When::Day(days) => {
            for (i, day) in days.iter().enumerate() {
                let day = BasicWhen::Day(*day);

                selected.insert(i as u16, (day, Vec::new()));
            }
        }
    }

    selected
}

////////////////////////
/// Simplified Time Representation
pub use api_types::availabilities::BasicWhen;

////////////////////////
/// Calculate Time Slots
// The number of 30 minute time slots to display for each day/date

pub fn num_slots(no_earlier: time::Time, no_later: time::Time) -> u16 {
    let no_earlier = {
        let no_earlier = no_earlier.as_hms();
        no_earlier.0 as u64 * 60 + no_earlier.1 as u64
    };

    let no_later = {
        let no_later = no_later.as_hms();
        no_later.0 as u64 * 60 + no_later.1 as u64
    };

    ((no_later - no_earlier) / 30) as u16
}

////////////////////////
/// Time Selector Component

#[derive(PartialEq, Properties)]
pub struct TimeSelectorProps {
    pub no_earlier: time::Time,
    pub num_slots: u16,
    pub selected: Selected,
    pub toggle: Callback<(u16, u16)>,
}

#[function_component]
pub fn TimeSelector(props: &TimeSelectorProps) -> Html {
    let TimeSelectorProps {
        no_earlier,
        num_slots,
        selected,
        toggle,
    } = props;

    let num_slots = (0..*num_slots).collect::<Vec<u16>>();

    let no_earlier = &*no_earlier;
    let no_earlier = no_earlier.clone();

    html! {
        <>
            <div class="flex flex-col p-4 pt-1 pr-0">
                <components::TimeLegend no_earlier={no_earlier} num_slots={num_slots.len() as u16} />
            </div>
            {
                for selected.iter().map(|(index, _slots)| {
                    html! {
                        <div class="flex flex-col p-4">
                            <div class="border bg-zinc-700 w-12">
                                {
                                    for num_slots.iter().map(|current_slot| {
                                        let selected = &*selected;
                                        let selected = selected.clone();

                                        html! {
                                            <Slot when={index} slot={current_slot} {toggle} {selected} />
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

////////////////////////
/// Time Slot
// The individual time slots for each individual day/date

#[derive(PartialEq, Properties)]
struct SlotProps {
    when: u16,
    slot: u16,
    selected: Selected,
    toggle: Callback<(u16, u16)>,
}

#[function_component]
fn Slot(props: &SlotProps) -> Html {
    let SlotProps {
        when,
        slot,
        selected,
        toggle,
    } = props;

    let onclick = {
        let when = when.clone();
        let slot = slot.clone();
        let toggle = toggle.clone();

        Callback::from(move |_| {
            toggle.emit((when, slot));
        })
    };

    let active = selected[&when].1.contains(&slot);

    let classes = {
        let base = classes!("h-6", "border", "cursor-pointer");

        match active {
            true => classes!(base, "bg-blue-800", "hover:bg-blue-950"),
            false => classes!(base, "bg-red-800", "hover:bg-red-950"),
        }
    };

    html! {
        <div class={classes} {onclick} />
    }
}
