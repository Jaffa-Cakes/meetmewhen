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
                ..
            },
    } = props;

    let num_slots = components::time_selector::num_slots(*no_earlier, *no_later);
    let selected = {
        let generated = components::time_selector::gen_selected(when);

        use_state_eq(|| generated)
    };

    let onsubmit = {
        let selected = selected.clone();
        let id = id.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let selected = selected.clone();
            let id = id.clone();

            wait(async move {
                let selected = &*selected;
                let selected = selected.clone();

                let res =
                    crate::api::Availabilities::create(api_types::availabilities::create::Req {
                        basic_event: id,
                        name: "Jedd".to_string(),
                        availabilities: api_types::availabilities::Availabilities(selected),
                    })
                    .await
                    .unwrap();

                log::info!("{:?}", res);
            });
        })
    };

    let toggle = {
        let selected = selected.clone();

        Callback::from(move |time: (u16, u16)| {
            let inner = &*selected;
            let mut inner = inner.clone();

            let indexed = &mut inner.get_mut(&time.0).unwrap().1;

            match indexed.contains(&time.1) {
                // We are removing the time
                true => {
                    let to_delete = indexed.iter().position(|&r| r == time.1).unwrap();

                    indexed.remove(to_delete);

                    selected.set(inner);
                }
                // We are adding the time
                false => {
                    indexed.push(time.1.into());

                    selected.set(inner);
                }
            }
        })
    };

    let selected = &*selected;
    let selected = selected.clone();

    html! {
        <div class="grid place-content-center w-screen h-screen">
            <form class="flex justify-around w-screen" {onsubmit}>
                <div class="flex flex-col bg-zinc-800">
                    <div class="text-2xl font-bold">{name}</div>
                </div>

                <div class="flex flex-row bg-zinc-800">
                    <components::TimeSelector {num_slots} {toggle} {selected} />
                </div>

                <atoms::Button r#type={atoms::ButtonType::Submit}>
                    {"Submit"}
                </atoms::Button>
            </form>
        </div>
    }
}
