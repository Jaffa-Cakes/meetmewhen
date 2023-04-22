use super::*;

////////////////////////
/// Mode Selector

#[derive(Clone, PartialEq)]
enum Type {
    Dates = 0,
    Days = 1,
}

////////////////////////
/// Form Input References

#[derive(Clone)]
struct CreateEventForm {
    name: NodeRef,
    r#type: NodeRef,
    no_earlier: NodeRef,
    no_later: NodeRef,
    timezone: NodeRef,
}

////////////////////////
/// Index Page

#[function_component]
pub fn Index() -> Html {
    let when_selected = use_state_eq(|| Type::Days);
    let dates_selected = use_state_eq(|| Vec::<time::Date>::new());
    let days_selected = use_state_eq(|| Vec::<time::Weekday>::new());

    let navigator = use_navigator().unwrap();

    let form = CreateEventForm {
        name: use_node_ref(),
        r#type: use_node_ref(),
        no_earlier: use_node_ref(),
        no_later: use_node_ref(),
        timezone: use_node_ref(),
    };

    let onsubmit = {
        let navigator = navigator.clone();
        let form = form.clone();
        let dates_selected = dates_selected.clone();
        let days_selected = days_selected.clone();
        let when_selected = when_selected.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let navigator = navigator.clone();
            let form = form.clone();
            let dates_selected = dates_selected.clone();
            let days_selected = days_selected.clone();
            let when_selected = when_selected.clone();

            wait(async move {
                let name = match form.name.cast::<HtmlInputElement>() {
                    Some(input) => input.value(),
                    None => todo!("Handle name error"),
                };

                let dates_selected = &*dates_selected;
                let dates_selected = dates_selected.clone();
                let days_selected = &*days_selected;
                let days_selected = days_selected.clone();
                let when_selected = &*when_selected;
                let when_selected = when_selected.clone();

                let when = match when_selected {
                    Type::Dates => api_types::basic_event::When::Date(dates_selected),
                    Type::Days => api_types::basic_event::When::Day(days_selected),
                };

                let no_earlier = match form.no_earlier.cast::<HtmlInputElement>() {
                    Some(input) => {
                        let input = match input.value().parse::<u8>() {
                            Ok(input) => input,
                            Err(_) => todo!("Handle no_earlier error"),
                        };

                        match time::Time::from_hms(input, 0, 0) {
                            Ok(time) => time,
                            Err(_) => todo!("Handle no_earlier error"),
                        }
                    }
                    None => todo!("Handle no_earlier error"),
                };

                let no_later = match form.no_later.cast::<HtmlInputElement>() {
                    Some(input) => {
                        let input = match input.value().parse::<u8>() {
                            Ok(input) => input,
                            Err(_) => todo!("Handle no_later error"),
                        };

                        match time::Time::from_hms(input, 0, 0) {
                            Ok(time) => time,
                            Err(_) => todo!("Handle no_later error"),
                        }
                    }
                    None => todo!("Handle no_later error"),
                };
                // let timezone = match form.timezone.cast::<HtmlInputElement>() {
                //     Some(input) => input.value(),
                //     None => todo!("Handle timezone error"),
                // };

                let req = api_types::basic_event::create::Req {
                    name,
                    when,
                    no_earlier,
                    no_later,
                    timezone: time::UtcOffset::UTC,
                };

                if !req.is_valid() {
                    todo!("Handle invalid request");
                }

                match crate::api::BasicEvent::create(req).await {
                    Ok(res) => {
                        navigator.push(&Routes::Event { id: res.id });
                    }
                    Err(_) => todo!("Handle error"),
                }
            });
        })
    };

    let when_set = {
        let when_selected = when_selected.clone();
        let form = form.clone();

        Callback::from(move |_| {
            let input = form.r#type.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();

                let value = match value.as_str() {
                    "0" => Type::Dates,
                    "1" => Type::Days,
                    _ => todo!("Handle when_set error"),
                };

                when_selected.set(value);
            }
        })
    };

    let date_toggle = {
        let dates_selected = dates_selected.clone();

        Callback::from(move |date: time::Date| {
            match dates_selected.contains(&date) {
                // We are removing the date
                true => {
                    let inner = &*dates_selected;
                    let mut inner = inner.clone();

                    let index = inner.iter().position(|i| *i == date).unwrap();

                    inner.remove(index);

                    dates_selected.set(inner);
                }
                // We are adding the date
                false => {
                    let inner = &*dates_selected;
                    let mut inner = inner.clone();

                    inner.push(date);

                    dates_selected.set(inner);
                }
            }
        })
    };

    let day_toggle = {
        let days_selected = days_selected.clone();

        Callback::from(move |day: time::Weekday| {
            match days_selected.contains(&day) {
                // We are removing the day
                true => {
                    let inner = &*days_selected;
                    let mut inner = inner.clone();

                    let index = inner.iter().position(|i| *i == day).unwrap();

                    inner.remove(index);

                    days_selected.set(inner);
                }
                // We are adding the day
                false => {
                    let inner = &*days_selected;
                    let mut inner = inner.clone();

                    inner.push(day);

                    days_selected.set(inner);
                }
            }
        })
    };

    let dates_selected = &*dates_selected;
    let dates_selected = dates_selected.clone();

    let days_selected = &*days_selected;
    let days_selected = days_selected.clone();

    let when_selected = &*when_selected;
    let when_selected = when_selected.clone();

    html! {
        <div class="min-h-screen min-w-screen bg-zinc-900 text-zinc-100 flex flex-col justify-center">
            <form class="flex justify-around" onsubmit={onsubmit}>
                <div class="bg-zinc-800 p-5 pt-4 rounded-2xl">
                    <span class="text-xl mr-5">{ "Type:" }</span>
                    <atoms::Select r#ref={form.r#type} onchange={when_set}>
                        <option value="0">{ "Dates" }</option>
                        <option value="1">{ "Days" }</option>
                    </atoms::Select>
                    <WhenSelector selected={when_selected} {dates_selected} {date_toggle} {days_selected} {day_toggle} />
                </div>

                <div class="flex flex-col justify-center">
                    <label>{ "Event Name: " }<atoms::InputText r#ref={form.name}/></label>

                    <atoms::Button r#type={atoms::ButtonType::Submit}>{ "Create Event" }</atoms::Button>

                    // <Link<Routes> to={Routes::NotFound}>
                    //     { "Not Found Page" }
                    // </Link<Routes>>
                </div>

                <div class="bg-zinc-800 p-5 pt-4 rounded-2xl">
                    <span class="text-xl mr-5">{ "No Earlier Than:" }</span>
                    <atoms::Select r#ref={form.no_earlier}>
                        <option value="0">{ "12:00 AM" }</option>
                        <option value="1">{ "1:00 AM" }</option>
                        <option value="2">{ "2:00 AM" }</option>
                        <option value="3">{ "3:00 AM" }</option>
                        <option value="4">{ "4:00 AM" }</option>
                        <option value="5">{ "5:00 AM" }</option>
                        <option value="6">{ "6:00 AM" }</option>
                        <option value="7">{ "7:00 AM" }</option>
                        <option value="8">{ "8:00 AM" }</option>
                        <option value="9" selected={true}>{ "9:00 AM" }</option>
                        <option value="10">{ "10:00 AM" }</option>
                        <option value="11">{ "11:00 AM" }</option>
                        <option value="12">{ "12:00 PM" }</option>
                        <option value="13">{ "1:00 PM" }</option>
                        <option value="14">{ "2:00 PM" }</option>
                        <option value="15">{ "3:00 PM" }</option>
                        <option value="16">{ "4:00 PM" }</option>
                        <option value="17">{ "5:00 PM" }</option>
                        <option value="18">{ "6:00 PM" }</option>
                        <option value="19">{ "7:00 PM" }</option>
                        <option value="20">{ "8:00 PM" }</option>
                        <option value="21">{ "9:00 PM" }</option>
                        <option value="22">{ "10:00 PM" }</option>
                        <option value="23">{ "11:00 PM" }</option>
                    </atoms::Select>

                    <br />

                    <span class="text-xl mr-5">{ "No Later Than:" }</span>
                    <atoms::Select r#ref={form.no_later}>
                        <option value="1">{ "1:00 AM" }</option>
                        <option value="2">{ "2:00 AM" }</option>
                        <option value="3">{ "3:00 AM" }</option>
                        <option value="4">{ "4:00 AM" }</option>
                        <option value="5">{ "5:00 AM" }</option>
                        <option value="6">{ "6:00 AM" }</option>
                        <option value="7">{ "7:00 AM" }</option>
                        <option value="8">{ "8:00 AM" }</option>
                        <option value="9">{ "9:00 AM" }</option>
                        <option value="10">{ "10:00 AM" }</option>
                        <option value="11">{ "11:00 AM" }</option>
                        <option value="12">{ "12:00 PM" }</option>
                        <option value="13">{ "1:00 PM" }</option>
                        <option value="14">{ "2:00 PM" }</option>
                        <option value="15">{ "3:00 PM" }</option>
                        <option value="16">{ "4:00 PM" }</option>
                        <option value="17" selected={true}>{ "5:00 PM" }</option>
                        <option value="18">{ "6:00 PM" }</option>
                        <option value="19">{ "7:00 PM" }</option>
                        <option value="20">{ "8:00 PM" }</option>
                        <option value="21">{ "9:00 PM" }</option>
                        <option value="22">{ "10:00 PM" }</option>
                        <option value="23">{ "11:00 PM" }</option>
                        <option value="24">{ "12:00 AM" }</option>
                    </atoms::Select>

                    <br />

                    <span class="text-xl mr-5">{ "Timezone:" }</span>
                    <atoms::Select r#ref={form.timezone}>
                        <option value="1">{ "1" }</option>
                        <option value="2">{ "2" }</option>
                    </atoms::Select>
                </div>
            </form>
        </div>
    }
}

////////////////////////
/// Mode Selector
// Displays the desired mode of input

#[derive(PartialEq, Properties)]
struct WhenSelectorProps {
    selected: Type,
    dates_selected: Vec<time::Date>,
    date_toggle: Callback<time::Date>,
    days_selected: Vec<time::Weekday>,
    day_toggle: Callback<time::Weekday>,
}

#[function_component]
fn WhenSelector(props: &WhenSelectorProps) -> Html {
    let WhenSelectorProps {
        selected,
        dates_selected,
        date_toggle,
        days_selected,
        day_toggle,
    } = props;

    let dates_selected = &*dates_selected;
    let dates_selected = dates_selected.clone();

    let days_selected = &*days_selected;
    let days_selected = days_selected.clone();

    match selected {
        Type::Dates => html! {
            <components::Calendar selected={dates_selected} toggle={date_toggle} />
        },
        Type::Days => html! {
            <components::Week selected={days_selected} toggle={day_toggle} />
        },
    }
}
