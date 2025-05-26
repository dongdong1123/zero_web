use actix_web::dev::Server;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FromData {
    email: String,
    name: String,
}

#[allow(dead_code)]
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

// fn index(from: web::Form<FromData>) -> String {
//     format!("Welcome {} !", from.name)
// }

async fn subcribe(_from: web::Form<FromData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            // .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subcriptions", web::post().to(subcribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
