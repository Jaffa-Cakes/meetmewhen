use super::*;

#[derive(Clone, PartialEq)]
enum Type {
    Dates = 0,
    Days = 1,
}

#[derive(Clone)]
struct CreateEventForm {
    name: NodeRef,
    r#type: NodeRef,
    no_earlier: NodeRef,
    no_later: NodeRef,
    timezone: NodeRef,
}

#[function_component]
pub fn Index() -> Html {
    let dates_selected = use_state_eq(|| Vec::<time::Date>::new());

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

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let navigator = navigator.clone();
            let form = form.clone();
            let dates_selected = dates_selected.clone();

            wait(async move {
                let name = match form.name.cast::<HtmlInputElement>() {
                    Some(input) => input.value(),
                    None => todo!("Handle name error"),
                };
                // let when = match form.when.cast::<HtmlInputElement>() {
                //     Some(input) => input.value(),
                //     None => todo!("Handle when error"),
                // };
                let dates_selected = &*dates_selected;
                let dates_selected = dates_selected.clone();
                let when = api_types::basic_event::When::Date(dates_selected);
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
                        navigator.push(&Route::Event { id: res.id });
                    }
                    Err(_) => todo!("Handle error"),
                }
            });
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
                },
                // We are adding the date
                false => {

                    let inner = &*dates_selected;
                    let mut inner = inner.clone();

                    inner.push(date);

                    dates_selected.set(inner);
                },
            }
        })
    };

    let dates_selected = &*dates_selected;
    let dates_selected = dates_selected.clone();

    html! {
        <div class="min-h-screen min-w-screen bg-zinc-900 text-zinc-100 flex flex-col justify-center">
            <form class="flex justify-around" onsubmit={onsubmit}>
                <div class="bg-zinc-800 p-5 pt-4 rounded-2xl">
                    <span class="text-xl mr-5">{ "Type:" }</span>
                    <atoms::Select r#ref={form.r#type.clone()}>
                        <option value="0">{ "Dates" }</option>
                        <option value="1">{ "Days" }</option>
                    </atoms::Select>
                    <WhenSelector r#type={form.r#type} selected={dates_selected} toggle={date_toggle} />
                </div>

                <div class="flex flex-col justify-center">
                    <label>{ "Event Name: " }<atoms::InputText r#ref={form.name}/></label>

                    <atoms::Button r#type={atoms::ButtonType::Submit}>{ "Create Event" }</atoms::Button>

                    <Link<Route> to={Route::NotFound}>
                        { "Not Found Page" }
                    </Link<Route>>
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

///////////////////////////////////

#[derive(PartialEq, Properties)]
struct WhenSelectorProps {
    r#type: NodeRef,
    selected: Vec<time::Date>,
    toggle: Callback<time::Date>,
}

#[function_component]
fn WhenSelector(props: &WhenSelectorProps) -> Html {
    let WhenSelectorProps { r#type, selected, toggle } = props;
    // let WhenSelectorProps { r#type } = props;

    // let r#type = match r#type.cast::<HtmlInputElement>() {
    //     Some(input) => {
    //         let input = match input.value().parse::<u8>() {
    //             Ok(input) => input,
    //             Err(_) => todo!("Handle no_earlier error"),
    //         };

    //         match input {
    //             0 => Type::Dates,
    //             1 => Type::Days,
    //             _ => todo!("Handle when type error"),
    //         }
    //     }
    //     None => todo!("Handle no_earlier error"),
    // };

    // match r#type {
    //     Type::Dates => html! {
    //         <components::Calendar />
    //     },
    //     Type::Days => html! {
    //         <components::Week />
    //     },
    // }

    let selected = &*selected;
    let selected = selected.clone();

    html! {
        <components::Calendar {selected} {toggle} />
    }
}