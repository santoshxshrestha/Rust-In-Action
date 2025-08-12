mod config;
use actix_web::{self, App, HttpResponse, HttpServer, Responder};
use actix_web::{get, web};
use config::config;

#[derive(Clone)]
pub struct DataContent {
    pub name: String,
    pub message: String,
}

#[get("/")]
pub async fn home(santosh: web::Data<DataContent>) -> impl Responder {
    let name = &santosh.name;
    let message = &santosh.message;

    HttpResponse::Ok().content_type("text/html").body(format!(
        "<h1>author>{}</h1> <br> <h2>message>{}</h2>",
        name, message
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let number = 12;

    let santosh = DataContent {
        name: "Santosh".to_string(),
        message: "Hello there what is going on".to_string(),
    };

    HttpServer::new(move || {
        App::new()
            .configure(config)
            .service(home)
            .app_data(web::Data::new(santosh.clone()))
            .app_data(web::Data::new(number))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
