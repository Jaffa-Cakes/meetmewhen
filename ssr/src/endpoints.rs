use super::*;

pub async fn found(
    request: HttpRequest,
    queries: web::Query<HashMap<String, String>>,
) -> impl Responder {
    HttpResponse::Ok().body(render(request.uri().to_string(), queries.into_inner()).await)
}

#[get("/{tail:.*}")]
pub async fn not_found(
    request: HttpRequest,
    queries: web::Query<HashMap<String, String>>,
) -> impl Responder {
    HttpResponse::NotFound().body(render(request.uri().to_string(), queries.into_inner()).await)
}
