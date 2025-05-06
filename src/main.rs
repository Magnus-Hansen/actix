mod handlers;
mod models;
mod routes;

use actix_web::{App, HttpServer};
use routes::user_routes::init;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(init) // Initialize routes
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}