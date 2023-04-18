use super::*;

////////////////////////
/// API Response Status

#[derive(PartialEq, Clone)]
enum Status {
    Waiting,
    Received(api_types::basic_event::get::Res),
}

////////////////////////
/// Event Page
// Displays the details of an existing event and allows the user to enter their availabilities

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component]
pub fn Event(props: &Props) -> Html {
    let Props { id } = props;

    let status = use_state_eq(|| Status::Waiting);

    let checker = {
        let id = id.clone();
        let status = status.clone();

        Callback::from(move |_| {
            let id = id.clone();
            let status = status.clone();

            wait(async move {
                status.set(
                    match crate::api::BasicEvent::get(api_types::basic_event::get::Req {
                        id: id.clone(),
                    })
                    .await
                    {
                        Ok(res) => Status::Received(res),
                        Err(_) => todo!("Handle event not existing"),
                    },
                );
            });
        })
    };

    let status = &*status;
    let status = status.clone();

    html! {
        <div class="min-h-screen min-w-screen bg-zinc-900 text-zinc-100 flex flex-col justify-center">
            <div class="flex justify-center">
                <Loader {status} {checker}/>
            </div>
        </div>
    }
}

////////////////////////
/// Loader
// Waits for the API response and then displays the page

#[derive(PartialEq, Properties)]
struct LoaderProps {
    status: Status,
    checker: Callback<()>,
}

#[function_component]
fn Loader(props: &LoaderProps) -> Html {
    let LoaderProps { status, checker } = props;

    if status == &Status::Waiting {
        checker.emit(());
    }

    match status.clone() {
        Status::Waiting => {
            html! {
                <div class="grid place-content-center w-screen h-screen">
                    <div class="animate-spin rounded-full h-32 w-32 border-b-2 border-zinc-100 " />
                </div>
            }
        }
        Status::Received(res) => {
            html! {
                <Page {res} />
            }
        }
    }
}

////////////////////////
/// Page Content

#[derive(PartialEq, Properties)]
struct PageProps {
    res: api_types::basic_event::get::Res,
}

#[function_component]
fn Page(props: &PageProps) -> Html {
    let PageProps {
        res:
            api_types::basic_event::get::Res {
                id,
                name,
                when,
                no_earlier,
                no_later,
                timezone,
                created,
            },
    } = props;

    let times_selected = {
        let when = match when {
            api_types::basic_event::When::Date(_) => api_types::basic_event::When::Date(vec![]),
            api_types::basic_event::When::Day(_) => api_types::basic_event::When::Day(vec![]),
        };

        use_state_eq(|| when)
    };

    let time_toggle = {
        let times_selected = times_selected.clone();

        Callback::from(move |time: api_types::basic_event::When| {
            let time = match time {
                api_types::basic_event::When::Date(time) => {
                    if let api_types::basic_event::When::Date(times_unwrapped) = &*times_selected {
                        let time = time.first().unwrap();

                        match times_unwrapped.contains(&time) {
                            // We are removing the time
                            true => {
                                let inner = &*times_unwrapped;
                                let mut inner = inner.clone();

                                let index = inner.iter().position(|i| *i == *time).unwrap();

                                inner.remove(index);

                                times_selected.set(api_types::basic_event::When::Date(inner));
                            }
                            // We are adding the time
                            false => {
                                let inner = &*times_unwrapped;
                                let mut inner = inner.clone();

                                inner.push(*time);

                                times_selected.set(api_types::basic_event::When::Date(inner));
                            }
                        }
                    } else {
                        panic!("This should never happen!");
                    }
                }
                api_types::basic_event::When::Day(time) => {
                    if let api_types::basic_event::When::Day(times_unwrapped) = &*times_selected {
                        let time = time.first().unwrap();

                        match times_unwrapped.contains(&time) {
                            // We are removing the time
                            true => {
                                let inner = &*times_unwrapped;
                                let mut inner = inner.clone();

                                let index = inner.iter().position(|i| *i == *time).unwrap();

                                inner.remove(index);

                                times_selected.set(api_types::basic_event::When::Day(inner));
                            }
                            // We are adding the time
                            false => {
                                let inner = &*times_unwrapped;
                                let mut inner = inner.clone();

                                inner.push(*time);

                                times_selected.set(api_types::basic_event::When::Day(inner));
                            }
                        }
                    } else {
                        panic!("This should never happen!");
                    }
                }
            };
        })
    };

    let when = when.clone();
    let no_earlier = no_earlier.clone();
    let no_later = no_later.clone();

    let times_selected = &*times_selected;
    let times_selected = times_selected.clone();

    html! {
        <div class="grid place-content-center w-screen h-screen">
            <div class="flex justify-around w-screen">
                <div class="flex flex-col bg-zinc-800">
                    <div class="text-2xl font-bold">{name}</div>
                </div>

                <div class="flex flex-row bg-zinc-800">
                    <components::TimeSelector {when} {no_earlier} {no_later} toggle={time_toggle} selected={times_selected} />
                </div>
            </div>
        </div>
    }
}
