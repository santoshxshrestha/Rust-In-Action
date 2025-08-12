#![allow(unused)]

use actix_web::HttpResponse;
use actix_web::get;
use actix_web::post;
use actix_web::web;
use actix_web::{self, App, HttpServer, Responder, ResponseError};
use serde::Deserialize;

// we use deserialize to the struct to access the content of the struct directly

#[derive(Deserialize)]
pub struct Data {
    name: String,
    message: String,
    id: u32,
}

#[get("path/{id}/{name}/{message}")]
pub async fn path(path: web::Path<Data>) -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(format!(
        "#{}, <br> >{}<br> ={},",
        path.id, path.name, path.message
    ))
}

#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Ok().body(format!("this is the home page"))
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(path).service(root))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
