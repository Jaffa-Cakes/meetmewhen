use api_types::availabilities::delete;

use super::*;

////////////////////////
/// API Response Status

#[derive(PartialEq, Clone)]
enum Status {
    Waiting,
    Received(
        (
            api_types::basic_event::get::Res,
            api_types::availabilities::get::Res,
        ),
    ),
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
                        Ok(event) => {
                            match crate::api::Availabilities::get(
                                api_types::availabilities::get::Req {
                                    basic_event: id.clone(),
                                },
                            )
                            .await
                            {
                                Ok(responses) => Status::Received((event, responses)),
                                Err(_) => todo!("Handle responses error"),
                            }
                        }
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

    let loader = html! {
        <div class="grid place-content-center w-screen h-screen">
            <div class="animate-spin rounded-full h-32 w-32 border-b-2 border-zinc-100 " />
        </div>
    };

    match status.clone() {
        Status::Waiting => loader,
        Status::Received((event, responses)) => {
            html! {
                <Page {event} {responses} {checker} />
            }
        }
    }
}

////////////////////////
/// Page Content

#[derive(PartialEq, Properties)]
struct PageProps {
    event: api_types::basic_event::get::Res,
    responses: api_types::availabilities::get::Res,
    checker: Callback<()>,
}

#[function_component]
fn Page(props: &PageProps) -> Html {
    let PageProps {
        event:
            api_types::basic_event::get::Res {
                id,
                name,
                when,
                no_earlier,
                no_later,
                ..
            },
        responses: api_types::availabilities::get::Res { respondents, .. },
        checker,
    } = props;

    let num_slots = components::time_selector::num_slots(*no_earlier, *no_later);
    let selected = {
        let generated = components::time_selector::gen_selected(when);

        use_state_eq(|| generated)
    };
    let delete_show = use_state_eq(|| false);
    let user_name = use_node_ref();
    let selected_respondent = use_node_ref();

    let onsubmit = {
        let selected = selected.clone();
        let checker = checker.clone();
        let when = when.clone();
        let id = id.clone();
        let user_name = user_name.clone();
        let selected_respondent = selected_respondent.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let selected = selected.clone();
            let checker = checker.clone();
            let when = when.clone();
            let id = id.clone();
            let user_name = user_name.clone();
            let selected_respondent = selected_respondent.clone();

            wait(async move {
                let inner = &*selected;
                let inner = inner.clone();

                let selected_respondent_element =
                    match selected_respondent.cast::<HtmlInputElement>() {
                        Some(element) => element,
                        None => todo!("Handle user name error"),
                    };

                let selected_respondent = selected_respondent_element.value();

                let user_name_element = match user_name.cast::<HtmlInputElement>() {
                    Some(element) => element,
                    None => todo!("Handle user name error"),
                };

                let user_name = user_name_element.value();

                if selected_respondent == "new" {
                    crate::api::Availabilities::create(api_types::availabilities::create::Req {
                        basic_event: id,
                        name: user_name,
                        availabilities: api_types::availabilities::Availabilities(inner),
                    })
                    .await
                    .unwrap();
                } else {
                    crate::api::Availabilities::update(api_types::availabilities::update::Req {
                        id: selected_respondent.parse().unwrap(),
                        basic_event: id,
                        name: user_name,
                        availabilities: api_types::availabilities::Availabilities(inner),
                    })
                    .await
                    .unwrap();
                }

                user_name_element.set_value("");
                selected.set(components::time_selector::gen_selected(&when));
                selected_respondent_element.set_value("new");
                checker.emit(());
            });
        })
    };

    let delete_respondent = {
        let selected_respondent = selected_respondent.clone();
        let delete_show = delete_show.clone();
        let id = id.clone();
        let selected = selected.clone();
        let user_name = user_name.clone();
        let checker = checker.clone();
        let when = when.clone();

        Callback::from(move |_: MouseEvent| {
            let selected = selected.clone();
            let delete_show = delete_show.clone();
            let checker = checker.clone();
            let when = when.clone();
            let user_name = user_name.clone();
            let selected_respondent = selected_respondent.clone();
            let id = id.clone();

            let selected_respondent_element = match selected_respondent.cast::<HtmlInputElement>() {
                Some(element) => element,
                None => todo!("Handle user name error"),
            };

            let selected_respondent = selected_respondent_element.value();

            let user_name_element = match user_name.cast::<HtmlInputElement>() {
                Some(element) => element,
                None => todo!("Handle user name error"),
            };

            wait(async move {
                crate::api::Availabilities::delete(api_types::availabilities::delete::Req {
                    id: selected_respondent.parse().unwrap(),
                    basic_event: id,
                })
                .await
                .unwrap();

                user_name_element.set_value("");
                selected.set(components::time_selector::gen_selected(&when));
                selected_respondent_element.set_value("new");
                delete_show.set(false);
                checker.emit(());
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

    let refresh = {
        let checker = checker.clone();

        Callback::from(move |_| {
            checker.emit(());
        })
    };

    let change_respondent = {
        let selected_respondent = selected_respondent.clone();
        let selected = selected.clone();
        let user_name = user_name.clone();
        let when = when.clone();
        let respondents = respondents.clone();
        let delete_show = delete_show.clone();

        Callback::from(move |_| {
            let selected_respondent = match selected_respondent.cast::<HtmlInputElement>() {
                Some(selected_respondent) => selected_respondent,
                None => todo!("Handle error"),
            };

            if selected_respondent.value() == "new" {
                user_name.cast::<HtmlInputElement>().unwrap().set_value("");
                selected.set(components::time_selector::gen_selected(&when));
                delete_show.set(false);
                return;
            }

            delete_show.set(true);

            let selected_respondent = match selected_respondent.value().parse::<i32>() {
                Ok(selected_respondent) => selected_respondent,
                Err(_) => todo!("Handle error"),
            };

            let respondent_info = respondents
                .iter()
                .filter(|r| r.id == selected_respondent)
                .collect::<Vec<&api_types::availabilities::get::Respondent>>();

            let respondent_info = respondent_info.first().unwrap();

            selected.set(respondent_info.availabilities.0.clone());
            user_name
                .cast::<HtmlInputElement>()
                .unwrap()
                .set_value(&respondent_info.name);
        })
    };

    let selected = &*selected;
    let selected = selected.clone();

    let respondents = &*respondents;
    let respondents = respondents.clone();

    let no_earlier = &*no_earlier;
    let no_earlier = no_earlier.clone();

    let delete_show = &*delete_show;
    let delete_show = delete_show.clone();

    let num_days = selected.len() as u16;

    html! {
        <div class="grid place-content-center w-screen h-screen">
            <div class="flex justify-center w-screen mb-12">
                <h1 class="text-4xl font-bold">{name}</h1>
            </div>

            <div class="flex justify-around w-screen">
                <form class="flex flex-col bg-zinc-800 p-4 rounded" {onsubmit}>
                    <span class="text-center text-2xl font-bold mb-2">{"Select Your Availability"}</span>

                    <label class="mx-auto">
                        {"Respondent:"}
                        <atoms::Select class="!bg-zinc-900 ml-2" r#ref={selected_respondent} onchange={change_respondent}>
                            <option value="new" selected={true}>{"New"}</option>
                            {
                                for respondents.iter().map(|r| {
                                    html! {
                                        <option value={format!("{}", r.id)}>{&r.name}</option>
                                    }
                                })
                            }
                        </atoms::Select>
                    </label>

                    <div class="flex flex-row justify-center">
                        <components::TimeSelector {num_slots} {toggle} {selected} {no_earlier} />
                    </div>

                    <label>
                        {"Name"}
                        <atoms::InputText class="!bg-zinc-900 ml-2" r#ref={user_name} />
                    </label>

                    <atoms::Button r#type={atoms::ButtonType::Submit} class="!bg-zinc-900 hover:!bg-zinc-700">
                        {"Submit"}
                    </atoms::Button>

                    <DeleteRespondent show={delete_show} onclick={delete_respondent} />
                </form>

                <components::Respondents {respondents} {num_days} {num_slots} {refresh} {no_earlier} />
            </div>
        </div>
    }
}

////////////////////////
/// Delete a Respondent

#[derive(PartialEq, Properties)]
struct DeleteRespondentProps {
    show: bool,
    onclick: Callback<MouseEvent>,
}

#[function_component]
fn DeleteRespondent(props: &DeleteRespondentProps) -> Html {
    let DeleteRespondentProps { show, onclick } = props;

    if !show {
        return html! {};
    }

    html! {
        <atoms::Button {onclick} class="!bg-zinc-900 hover:!bg-zinc-700">
            {"Delete"}
        </atoms::Button>
    }
}
