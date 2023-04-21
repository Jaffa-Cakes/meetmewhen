use super::*;

////////////////////////
/// Respondents Component

#[derive(Properties, PartialEq)]
pub struct Props {
    pub respondents: Vec<api_types::availabilities::get::Respondent>,
    pub num_slots: u16,
    pub num_days: u16,
    pub refresh: Callback<MouseEvent>,
    pub no_earlier: time::Time,
}

#[function_component]
pub fn Respondents(props: &Props) -> Html {
    let Props {
        respondents,
        num_slots,
        num_days,
        refresh,
        no_earlier,
    } = props;

    let hovered: UseStateHandle<Option<(u16, u16)>> = { use_state_eq(|| None) };

    let days_iter = (0..*num_days).collect::<Vec<u16>>();
    let highest_count = {
        let mut highest = 1;

        if respondents.len() > 0 {
            for d in 0..*num_days {
                for s in 0..*num_slots {
                    let mut slot_count = 0;

                    for r in respondents.iter() {
                        let availabilities = &r.availabilities.0.get(&d).unwrap().1;

                        if availabilities.contains(&s) {
                            slot_count += 1;
                        }
                    }

                    if slot_count > highest {
                        highest = slot_count;
                    }
                }
            }
        }

        highest
    };

    let set_hovered = {
        let hovered = hovered.clone();

        Callback::from(move |new: Option<(u16, u16)>| {
            hovered.set(new);
        })
    };

    let respondents = &*respondents;
    let respondents = respondents.clone();
    let hovered = &*hovered;
    let hovered = hovered.clone();
    let no_earlier = &*no_earlier;
    let no_earlier = no_earlier.clone();

    html! {
        <div class="bg-zinc-800 rounded-lg p-4">
            <h2 class="text-2xl font-bold text-zinc-100 text-center mb-4">{"Responses"}</h2>
            <div class="flex space-x-2 justify-center">
                <div class="flex flex-col p-4 pt-1 pr-0">
                    <components::TimeLegend no_earlier={no_earlier} num_slots={num_slots} />
                </div>
                {
                    for days_iter.iter().map(|day| {
                        let set_hovered = set_hovered.clone();

                        html! {
                            <Day respondents={respondents.clone()} {day} {num_slots} {highest_count} {set_hovered} />
                        }
                    })
                }
            </div>

            <atoms::Button onclick={refresh} class="!bg-zinc-900 hover:!bg-zinc-700 w-full mb-2">
                {"Refresh"}
            </atoms::Button>

            <div class="flex justify-center">
                <Info respondents={respondents.clone()} {hovered} />
            </div>
        </div>
    }
}

////////////////////////
/// Individual Days
// Highest count is the largest number of people available on a given day

#[derive(Properties, PartialEq)]
struct DayProps {
    respondents: Vec<api_types::availabilities::get::Respondent>,
    num_slots: u16,
    day: u16,
    highest_count: u16,
    set_hovered: Callback<Option<(u16, u16)>>,
}

#[function_component]
fn Day(props: &DayProps) -> Html {
    let DayProps {
        respondents,
        num_slots,
        day,
        highest_count,
        set_hovered,
    } = props;

    let slots_iter = (0..*num_slots).collect::<Vec<u16>>();

    html! {
        <div class="p-4">
            <div class="flex flex-col border bg-zinc-700">
                {
                    for slots_iter.iter().map(|slot| {
                        let respondents = &*respondents;
                        let respondents = respondents.clone();

                        html! {
                            <Slot {respondents} {day} {slot} {highest_count} {set_hovered} />
                        }
                    })
                }
            </div>
        </div>
    }
}

////////////////////////
/// Individual Slots

#[derive(Properties, PartialEq)]
struct SlotProps {
    respondents: Vec<api_types::availabilities::get::Respondent>,
    slot: u16,
    day: u16,
    highest_count: u16,
    set_hovered: Callback<Option<(u16, u16)>>,
}

#[function_component]
fn Slot(props: &SlotProps) -> Html {
    let SlotProps {
        respondents,
        slot,
        day,
        highest_count,
        set_hovered,
    } = props;
    /* If we want to calculate opacity based on the total number of respondents
    instead of based on the maximum number of respondents that selected an individual
    slot, uncomment this and comment out the line underneath.

    This should be made a UI option in the future so the user is able to view both. */
    // let total_resp = respondents.len();
    let total_resp = *highest_count;

    let total_selected = respondents
        .iter()
        .filter(|resp| {
            let availabilities = &resp.availabilities.0;
            let today = &availabilities.get(&day).unwrap().1;

            today.contains(slot)
        })
        .count();

    let percentage = (total_selected as f64 / total_resp as f64) * 100.0;

    let style = format!("opacity: {}%;", percentage);

    let onmouseenter = {
        let day = day.clone();
        let slot = slot.clone();
        let set_hovered = set_hovered.clone();

        Callback::from(move |_| {
            set_hovered.emit(Some((day, slot)));
        })
    };

    let onmouseleave = {
        let set_hovered = set_hovered.clone();

        Callback::from(move |_| {
            set_hovered.emit(None);
        })
    };

    html! {
        <div class="border h-6 w-12">
            <div class="bg-white h-full w-full" {style} {onmouseenter} {onmouseleave} />
        </div>
    }
}

////////////////////////
/// Info Panel

#[derive(Properties, PartialEq)]
struct InfoProps {
    respondents: Vec<api_types::availabilities::get::Respondent>,
    hovered: Option<(u16, u16)>,
}

#[function_component]
fn Info(props: &InfoProps) -> Html {
    let InfoProps {
        respondents,
        hovered,
    } = props;

    match hovered {
        Some((day, slot)) => {
            let mut available = vec![];
            let mut unavailable = vec![];

            for r in respondents.iter() {
                let availabilities = &r.availabilities.0.get(&day).unwrap().1;
                let name = r.name.clone();

                match availabilities.contains(&slot) {
                    true => available.push(name),
                    false => unavailable.push(name),
                };
            }

            html! {
                <div class="h-64 w-48 mx-auto">
                    <h1 class="font-bold text-lg text-center">{"Respondents"}</h1>

                    <div class="flex space-x-4">
                        <div class="flex flex-col space-y-2">
                            <span class="font-bold text-center">{"Available"}</span>
                            {
                                for available.iter().map(|name| {
                                    html! {
                                        <span>{name}</span>
                                    }
                                })
                            }
                        </div>
                        <div class="flex flex-col space-y-2">
                            <span class="font-bold text-center">{"Unavailable"}</span>
                            {
                                for unavailable.iter().map(|name| {
                                    html! {
                                        <span>{name}</span>
                                    }
                                })
                            }
                        </div>
                    </div>
                </div>
            }
        }
        None => html! { <div class="h-64 w-48" /> },
    }
}
