use actix_files::{self, Files};
use maud::html;

use actix_web::{self, App, HttpResponse, HttpServer, Responder, get};
use askama::{self, Template};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    number: i32,
}

#[get("/api/count")]
pub async fn count() -> impl Responder {
    let html = html!(
        h1 {"hello world this is hello from the moud"}
    );
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html.into_string())
}

#[get("/")]
pub async fn home() -> impl Responder {
    let template = HomeTemplate { number: 0 };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(home)
            .service(count)
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
