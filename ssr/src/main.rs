use actix_web::*;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

/////////////////////////////

#[get("/{tail:.*}")]
async fn catch_all() -> impl Responder {
    HttpResponse::Ok().body(render().await)
}

#[get("/index.html")]
async fn index_prevention() -> impl Responder {
    HttpResponse::Ok().body(render().await)
}

//////////////////////////////

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting HTTP Server");

    HttpServer::new(|| {
        let generated = generate();

        App::new()
        .service(index_prevention)
        .service(actix_web_static_files::ResourceFiles::new("/", generated).do_not_resolve_defaults().skip_handler_when_not_found())
        .service(catch_all)
        .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}

//////////////////////////////

async fn render() -> String {
    let renderer = yew::ServerRenderer::<client::App>::new();

    let generated = generate();

    let index_html_s = std::str::from_utf8(generated.get("index.html").expect("index.html not found").data).expect("index.html could not be parsed");

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