use actix_web;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::get;
use actix_web::web;

#[get("/count")]
pub async fn count(count: web::Data<i32>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("  <h1>{}</h1>", **count))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(count);
}
