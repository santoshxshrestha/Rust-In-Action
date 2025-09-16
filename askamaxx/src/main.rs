use actix_web::App;
use actix_web::Responder;
use actix_web::get;
use actix_web::web;
use actix_web::{self, HttpResponse, HttpServer};
use askama::Template;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub count: usize,
}

#[get("/")]
pub async fn hello(count: web::Data<Arc<AtomicUsize>>) -> impl Responder {
    let template = IndexTemplate {
        count: count.clone().fetch_add(1, Ordering::SeqCst),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let count = Arc::new(AtomicUsize::new(0));
    println!("Starting server at http://localhost:8080");
    let clone_count = count.clone();
    HttpServer::new(move || {
        App::new()
            .service(hello)
            .app_data(web::Data::new(clone_count.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
