use super::*;

#[derive(PartialEq, Clone)]
enum Status {
    Waiting,
    Received(api_types::basic_event::get::Res),
}

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

/////////////////////////
///
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
                <div class="grid place-content-center w-screen h-screen">
                    <h1 class="text-4xl">{ format!("{:#?}", res) }</h1>
                </div>
            }
        }
    }
}
