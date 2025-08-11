#![allow(unused)]
use std::sync::Arc;

use actix_web::{self, App, HttpResponse, HttpServer, Responder};
use actix_web::{get, web};

#[derive(Clone)]
pub struct DataContent {
    pub name: String,
    pub message: String,
}

#[get("/")]
pub async fn home(santosh: web::Data<DataContent>) -> impl Responder {
    let name = &santosh.name;
    let message = &santosh.message;

    HttpResponse::Ok().content_type("html/text").body(format!(
        "<h1>author>{}</h1> <br> <h2>message>{}</h2>",
        name, message
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let santosh = DataContent {
        name: "Santosh".to_string(),
        message: "Hello there what is going on".to_string(),
    };

    HttpServer::new(move || {
        App::new()
            .service(home)
            .app_data(web::Data::new(santosh.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
