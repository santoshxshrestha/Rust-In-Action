#![allow(unused)]
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::get;
use actix_web::web;
use actix_web::{self, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Query {
    user_id: usize,
    user_name: String,
    message: String,
}

#[get("/")]
pub async fn root(query: web::Query<Query>) -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(format!(
        "<h1>#{}<br><h1> > {}<br><h2> ={}",
        query.user_id, query.user_name, query.message
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(root))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
