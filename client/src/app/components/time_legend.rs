use super::*;

////////////////////////
/// Time Legend Component
// Generates a legend of time slots

#[derive(PartialEq, Properties)]
pub struct Props {
    pub no_earlier: time::Time,
    pub num_slots: u16,
}

#[function_component]
pub fn TimeLegend(props: &Props) -> Html {
    let Props {
        no_earlier,
        num_slots,
    } = props;

    let num_slots = (0..*num_slots).collect::<Vec<u16>>();

    html! {
        <>
            {
                for num_slots.iter().map(|current_slot| {
                    if current_slot % 2 != 0 {
                        return html! {};
                    }

                    let time = *no_earlier + time::Duration::minutes(*current_slot as i64 * 30);
                    let time = time.as_hms();
                    let time = format!("{:02}:{:02}", time.0, time.1);

                    html! {
                        <>
                            <div class="flex justify-end">
                                <span class="h-6">{time}</span>
                            </div>
                            <div class="h-6" />
                        </>
                    }
                })
            }
        </>
    }
}
