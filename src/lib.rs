use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}


async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Let's start simple: we always return a 200 OK
async fn subscribe(_form:web::Form<FormData>)-> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/subscriptions", web::post().to(subscribe))
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
