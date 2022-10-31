use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use middlewares::test;
use routes::index;

mod middlewares;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default())
            .wrap(Logger::default())
            .wrap(test::SayHi)
            .configure(index::init)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
