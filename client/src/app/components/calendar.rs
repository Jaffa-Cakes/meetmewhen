use super::*;

static NUM_OF_ROWS: i64 = 7;

///////////////////////////////////////////

#[derive(PartialEq, Properties)]
pub struct CalendarProps {
    pub selected: Vec<time::Date>,
    pub toggle: Callback<time::Date>,
}

#[function_component]
pub fn Calendar(props: &CalendarProps) -> Html {
    let CalendarProps { selected, toggle } = props;

    let anchor = {
        let mut anchor = time::Date::from_calendar_date(2022, time::Month::December, 15).unwrap();

        while anchor.weekday() != time::Weekday::Monday {
            anchor = anchor - time::Duration::days(1);
        }

        anchor
    };

    let mut dates: Vec<time::Date> = Vec::new();

    for i in 0..(NUM_OF_ROWS * 7) {
        dates.push(anchor + time::Duration::days(i));
    }

    let day_names = ["M", "T", "W", "T", "F", "S", "S"];

    html! {
        <div class="flex flex-col gap-1 select-none">
            <div class="flex flex-row gap-1">
                <div class="w-16 h-6"></div>
                {
                    for day_names.iter().map(|name| {
                        html! {
                            <Heading name={name.to_string()} />
                        }
                    })
                }
                <div class="w-16 h-6"></div>
            </div>
            {
                for dates.chunks(7).map(|chunk| {
                    let selected = &*selected;
                    let selected = selected.clone();
                    
                    html! {
                        <Row dates={chunk.to_vec()} {selected} {toggle} />
                    }
                })
            }
        </div>
    }
}

///////////////////////////////////////////

#[derive(PartialEq, Properties)]
struct HeadingProps {
    name: String,
}

#[function_component]
fn Heading(props: &HeadingProps) -> Html {
    let HeadingProps { name } = props;

    html! {
        <div class="w-6 h-6 grid place-content-center"><span class="text-md">{ name }</span></div>
    }
}

///////////////////////////////////////////

#[derive(PartialEq, Properties)]
struct RowProps {
    dates: Vec<time::Date>,
    selected: Vec<time::Date>,
    toggle: Callback<time::Date>,
}

#[function_component]
fn Row(props: &RowProps) -> Html {
    let RowProps { dates, selected, toggle } = props;

    let month_note = match dates.first().unwrap().month() != dates.last().unwrap().month() {
        true => {

            let m1 = &format!("{}", dates.first().unwrap().month())[..3];
            let m2 = &format!("{}", dates.last().unwrap().month())[..3];

            format!("{}/{}", m1, m2)
        },
        false => {
            let m = &format!("{}", dates.first().unwrap().month())[..3];

            m.to_string()
        },
    };

    let year_note = match dates.first().unwrap().year() != dates.last().unwrap().year() {
        true => {

            let m1 = format!("{}", dates.first().unwrap().year());
            let m2 = &format!("{}", dates.last().unwrap().year())[2..];

            format!("{}/{}", m1, m2)
        },
        false => {
            format!("{}", dates.first().unwrap().year())
        },
    };

    html! {
        <div class="flex flex-row gap-1">
            <div class="w-16 h-6 flex justify-end"><span class="text-md">{ month_note }</span></div>
            {
                for dates.iter().map(|date| {
                    let selected = &*selected;
                    let selected = selected.clone();

                    html! {
                        <Date date={*date} {selected} {toggle} />
                    }
                })
            }
            <div class="w-16 h-6 flex justify-start"><span class="text-md">{ year_note }</span></div>
        </div>
    }
}

///////////////////////////////////////////

#[derive(PartialEq, Properties)]
struct DateProps {
    date: time::Date,
    selected: Vec<time::Date>,
    toggle: Callback<time::Date>,
}

#[function_component]
fn Date(props: &DateProps) -> Html {
    let DateProps { date, selected, toggle } = props;

    let onclick = {
        let date = date.clone();
        let toggle = toggle.clone();

        Callback::from(move |_| {
            toggle.emit(date.clone());
        })
    };

    match selected.contains(date) {
        true => {
            html! {
                <div class="w-6 h-6 bg-blue-800 hover:bg-blue-950 grid place-content-center border border-blue-800 cursor-pointer" {onclick}>
                    <span class="text-md">{ format!("{}", date.day()) }</span>
                </div>
            }
        },
        false => {
            html! {
                <div class="w-6 h-6 bg-red-800 hover:bg-red-950 grid place-content-center border border-blue-800 cursor-pointer" {onclick}>
                    <span class="text-md">{ format!("{}", date.day()) }</span>
                </div>
            }
        },
    }
}