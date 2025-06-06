use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::health_check;
use crate::routes::subcribe;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            // .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subcriptions", web::post().to(subcribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
