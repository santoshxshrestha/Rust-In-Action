use actix_web::App;
use actix_web::Responder;
use actix_web::get;
use actix_web::post;
use actix_web::web;
use actix_web::{self, HttpResponse, HttpServer};
use askama::Template;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[post("/count")]
pub async fn count(count: web::Data<Arc<AtomicUsize>>) -> impl Responder {
    let current_count = count.fetch_add(1, Ordering::AcqRel);
    HttpResponse::Ok().body(format!("count: {}", current_count))
}

#[get("/")]
pub async fn hello() -> impl Responder {
    let template = IndexTemplate;
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let num = Arc::new(AtomicUsize::new(0));
    let nums = Arc::new(32);
    println!("Starting server at http://localhost:8080");
    let clone_count = num.clone();
    let cloen_of_nums = nums.clone();
    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(count)
            .app_data(web::Data::new(cloen_of_nums.clone()))
            .app_data(web::Data::new(clone_count.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
