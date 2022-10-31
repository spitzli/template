use actix_web::{get, web::ServiceConfig};

#[get("/")]
pub async fn index() -> &'static str {
    "Hello world!"
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
}
