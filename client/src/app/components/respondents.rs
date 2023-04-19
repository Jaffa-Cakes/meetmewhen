use super::*;

////////////////////////
/// Respondents Component

#[derive(Properties, PartialEq)]
pub struct Props {
    pub respondents: Vec<api_types::availabilities::get::Respondent>,
    pub num_slots: u16,
    pub num_days: u16,
}

#[function_component]
pub fn Respondents(props: &Props) -> Html {
    let Props {
        respondents,
        num_slots,
        num_days,
    } = props;

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

    html! {
        <div class="bg-zinc-800 rounded-lg p-4">
            <h2 class="text-2xl font-bold text-zinc-100">{"Responses"}</h2>
            <div class="flex space-x-2">
                {
                    for days_iter.iter().map(|day| {
                        let respondents = &*respondents;
                        let respondents = respondents.clone();

                        html! {
                            <Day {respondents} {day} {num_slots} {highest_count} />
                        }
                    })
                }
            </div>
        </div>
    }
}

////////////////////////
/// Individual Days
// Highest count is the largest number of people available on a given day

#[derive(Properties, PartialEq)]
struct DayProps {
    pub respondents: Vec<api_types::availabilities::get::Respondent>,
    pub num_slots: u16,
    pub day: u16,
    pub highest_count: u16,
}

#[function_component]
fn Day(props: &DayProps) -> Html {
    let DayProps {
        respondents,
        num_slots,
        day,
        highest_count,
    } = props;

    let slots_iter = (0..*num_slots).collect::<Vec<u16>>();

    html! {
        <div class="flex flex-col bg-zinc-700">
            {
                for slots_iter.iter().map(|slot| {
                    let respondents = &*respondents;
                    let respondents = respondents.clone();

                    html! {
                        <Slot {respondents} {day} {slot} {highest_count} />
                    }
                })
            }
        </div>
    }
}

////////////////////////
/// Individual Slots

#[derive(Properties, PartialEq)]
struct SlotProps {
    pub respondents: Vec<api_types::availabilities::get::Respondent>,
    pub slot: u16,
    pub day: u16,
    pub highest_count: u16,
}

#[function_component]
fn Slot(props: &SlotProps) -> Html {
    let SlotProps {
        respondents,
        slot,
        day,
        highest_count,
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

    html! {
        <div class="bg-white h-6 w-12" {style}>

        </div>
    }
}
