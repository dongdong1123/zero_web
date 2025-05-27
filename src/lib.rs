pub mod configuration;
pub mod routes;
pub mod startup;

// pub use configuration::*;
// pub use routes::*;
// pub use startup::*;

// use actix_web::dev::Server;
// use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
// use std::net::TcpListener;

// #[allow(dead_code)]
// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

// fn index(from: web::Form<FromData>) -> String {
//     format!("Welcome {} !", from.name)
// }
