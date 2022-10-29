use actix_web::{App, HttpServer, middleware::Logger, web, HttpResponse};
use log::{debug, error, log_enabled, info, Level};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    debug!("Starting server");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::to(|| HttpResponse::Ok()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
