#![allow(unused)]
use actix_web::App;
use actix_web::Responder;
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::post;
use actix_web::web;
use actix_web::web::Form;
use actix_web::{self, HttpResponse, HttpServer};
use askama::Template;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::Mutex;
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

#[derive(Clone, Deserialize)]
pub struct ContactInfo {
    #[serde(default)]
    name: String,

    #[serde(default)]
    email: String,
}

impl ContactInfo {
    pub fn new(name: &str, email: &str) -> Self {
        ContactInfo {
            name: name.to_string(),
            email: email.to_string(),
        }
    }
}

#[derive(Template)]
#[template(path = "contacts.html")]
pub struct ContactsTemplate {
    contacts: Vec<ContactInfo>,
}

#[get("/contacts")]
pub async fn contacts(details: web::Data<Arc<Mutex<Vec<ContactInfo>>>>) -> impl Responder {
    let data = details.lock().unwrap();
    let template = ContactsTemplate {
        contacts: data.to_vec(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[post("/contacts")]
pub async fn add_contact(
    details: web::Data<Arc<Mutex<Vec<ContactInfo>>>>,
    form: Form<ContactInfo>,
) -> impl Responder {
    let mut details = details.lock().unwrap();
    if details.iter().any(|c| c.email == form.email) {
        return HttpResponse::BadRequest().body("Email already exists");
    } else {
        details.push(ContactInfo::new(&form.name, &form.email));
        HttpResponse::SeeOther()
            .append_header(("Location", "/contacts"))
            .finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let num = Arc::new(AtomicUsize::new(0));
    let contact_details: Arc<Mutex<Vec<ContactInfo>>> = Arc::new(Mutex::new(vec![
        ContactInfo::new("Alice", "alice@gmail.com"),
        ContactInfo::new("Bob", "bobthe_builder@gmail.com"),
        ContactInfo::new("kolam", "kolam@gmail.com"),
    ]));

    println!("Starting server at http://localhost:8080");
    let clone_count = num.clone();
    let clone_contacts = Arc::clone(&contact_details);
    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(count)
            .service(contacts)
            .service(add_contact)
            .app_data(web::Data::new(clone_contacts.clone()))
            .app_data(web::Data::new(clone_count.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
