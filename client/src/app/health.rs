use super::*;

#[derive(PartialEq, Clone)]
enum Status {
    Waiting,
    #[cfg(target_arch = "wasm32")]
    Healthy,
    #[cfg(target_arch = "wasm32")]
    Unhealthy,
}

#[derive(PartialEq, Properties)]
struct HealthCheckProps {
    status: Status,
    checker: Callback<()>,
}
#[function_component(Health)]
pub fn health() -> Html {
    let status = use_state_eq(|| Status::Waiting);

    let checker = {
        let status = status.clone();

        Callback::from(move |_| {
            let status = status.clone();

            #[cfg(target_arch = "wasm32")]
            wait(async move {
                status.set(match api::Health::is_ok().await {
                    true => Status::Healthy,
                    false => Status::Unhealthy,
                });
            });

            #[cfg(not(target_arch = "wasm32"))]
            status.set(Status::Waiting);
        })
    };

    let status = &*status;
    let status = status.clone();

    html! {
        <div class="min-h-screen min-w-screen bg-zinc-900 text-zinc-100 flex">
            <HealthCheck {status} {checker}/>
        </div>
    }
}

#[function_component(HealthCheck)]
fn health_check(props: &HealthCheckProps) -> Html {
    let HealthCheckProps { status, checker } = props;

    if status == &Status::Waiting {
        checker.emit(());
    }

    match *status {
        Status::Waiting => {
            html! {
                <div class="grid place-content-center w-screen h-screen">
                    <div class="animate-spin rounded-full h-32 w-32 border-b-2 border-zinc-100 " />
                </div>
            }
        }
        #[cfg(target_arch = "wasm32")]
        Status::Unhealthy => {
            html! {
                <div class="grid place-content-center w-screen h-screen">
                    <h1 class="text-4xl">{ "Health Check Failed" }</h1>
                </div>
            }
        }
        #[cfg(target_arch = "wasm32")]
        Status::Healthy => {
            html! {
                <div class="grid place-content-center w-screen h-screen">
                    <h1 class="text-4xl">{ "Health Check Success" }</h1>
                </div>
            }
        }
    }
}