#![allow(unused)]
use actix_files;
use actix_web::{self,web, App, HttpResponse, HttpServer, Responder, get, post};
use actix_multipart::Multipart;
use askama::Template;
use futures_util::stream::StreamExt as _;
use std::io::Write;
use std::fs;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    content: String,
}

impl Index {
    fn new(content: String) -> Self {
        Self { content }
    }
}

#[get("/")]
async fn index() -> impl Responder {
    let template = Index::new("hello there this is the content in the index page".to_string());
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[post("/upload")]
async fn upload(mut payload: Multipart) -> impl Responder {
    // Ensure directory exists
    fs::create_dir_all("./uploads").unwrap();

    // Iterate over multipart stream
    while let Some(mut field) = payload.next().await {
        let mut field = field.unwrap();
        let content_disposition = field.content_disposition().unwrap();
        let filename = content_disposition
            .get_filename()
            .map(|f| f.to_string())
            .unwrap_or_else(|| "upload.file".to_string());

        let filepath = format!("./uploads/{}", filename);
        let mut f = fs::File::create(filepath).unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).unwrap();
        }
    }
    HttpResponse::Ok().body("File uploaded!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 http server is running in 8080");
    HttpServer::new(move || App::new()
        .service(index)
        .service(upload)
        // If you want to serve the uploaded files
        .service(actix_files::Files::new("/uploads", "./uploads").show_files_listing())
    )
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
