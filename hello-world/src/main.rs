use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn santosh() -> impl Responder {
    HttpResponse::Ok().body("Hey there this is santosh shrestha")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("The server is running...");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/santosh", web::get().to(santosh))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

