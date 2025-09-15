use actix_web::App;
use actix_web::Responder;
use actix_web::get;
use actix_web::{self, HttpResponse, HttpServer};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[get("/")]
pub async fn hello() -> impl Responder {
    let template = IndexTemplate;
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");
    HttpServer::new(move || App::new().service(hello))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
