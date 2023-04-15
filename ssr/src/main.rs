use actix_web::http::header::{HeaderValue, CACHE_CONTROL, CONTENT_TYPE};
use actix_web::*;
use actix_web::{dev::Service as _, web, App};
use std::collections::HashMap;
use yew_router::Routable;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

static DO_CACHE: &'static [&str] = &["application/wasm", "application/javascript", "text/css"];

/////////////////////////////

async fn found(
    request: HttpRequest,
    queries: web::Query<HashMap<String, String>>,
) -> impl Responder {
    HttpResponse::Ok().body(render(request.uri().to_string(), queries.into_inner()).await)
}

#[get("/{tail:.*}")]
async fn not_found(
    request: HttpRequest,
    queries: web::Query<HashMap<String, String>>,
) -> impl Responder {
    HttpResponse::NotFound().body(render(request.uri().to_string(), queries.into_inner()).await)
}

//////////////////////////////

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    console_subscriber::init();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting HTTP Server");

    HttpServer::new(|| {
        let generated = generate();

        let mut app = App::new()
            .wrap(middleware::Compress::default())
            // Middleware that caches static files
            .wrap_fn(|req, srv| {
                let fut = srv.call(req);
                async {
                    let mut res = fut.await?;
                    if let Some(content_type) = res.headers().get(CONTENT_TYPE) {
                        if let Ok(content_type) = content_type.to_str() {
                            if DO_CACHE.contains(&content_type) {
                                res.headers_mut().insert(
                                    CACHE_CONTROL,
                                    HeaderValue::from_static("public, max-age=31536000"),
                                );
                            }
                        }
                    }
                    Ok(res)
                }
            })
            .service(web::redirect("/index.html", "/"))
            .service(
                actix_web_static_files::ResourceFiles::new("/", generated)
                    .do_not_resolve_defaults()
                    .skip_handler_when_not_found(),
            );

        for route in client::Route::routes().into_iter() {
            let split = route.trim_start_matches("/").split("/");

            let mut parts: Vec<String> = Vec::new();

            for part in split.into_iter() {
                let new_part;

                if part.starts_with(":") {
                    new_part = format!("{{{}}}", part.trim_start_matches(":"));
                } else {
                    new_part = part.to_string();
                }

                parts.push(new_part.to_string());
            }

            let route = format!("/{}", parts.join("/"));

            // We dont want to include the 404 route in the list of 200 routes
            if route == "/404" {
                continue;
            }

            app = app.route(&route, web::route().to(found));
        }

        app.service(not_found)
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}

//////////////////////////////

async fn render(url: String, queries: HashMap<String, String>) -> String {
    let renderer =
        yew::ServerRenderer::<client::ServerApp>::with_props(move || client::ServerAppProps {
            url: url.into(),
            queries,
        });

    let generated = generate();

    let index_html_s = std::str::from_utf8(
        generated
            .get("index.html")
            .expect("index.html not found")
            .data,
    )
    .expect("index.html could not be parsed");

    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");
    let index_html_after = index_html_after.to_owned();

    let index_html_before = index_html_before.clone();
    let index_html_after = index_html_after.clone();

    let mut html = String::new();

    html.push_str(&index_html_before);
    html.push_str(&renderer.render().await);
    html.push_str(&index_html_after);

    html
}
