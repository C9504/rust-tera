extern crate tera;

use actix_web::{App, get, HttpResponse, HttpServer, web};
use actix_web::web::Data;
use tera::{Tera};
use rust_tera::database::Database;

mod views;

#[get("/")]
async fn welcome() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to Actix Web and Tera Framework!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init("127.0.0.1:8000", "negrdo", "rust_tera")
        .await
        .expect("Database Connection error");
    let db = Data::new(db);

    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Template error"));
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(tera.clone()))
            .app_data(db.clone())
            .service(views::statics::read_static)
            .service(
                web::scope("/web")
                    .configure(views::users::config),
            )
            .service(welcome)
            .route("/", web::get())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
