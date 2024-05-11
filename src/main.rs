extern crate tera;

use actix_web::{App, get, HttpResponse, HttpServer, middleware, web};
use actix_web::web::{Data};
use tera::{Tera};
use rust_tera::database::Database;

mod views;
mod resources;

#[get("/")]
async fn welcome() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to Actix Web and Tera Framework!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let db = Database::init().await.expect("Database Connection error");
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
            .wrap(middleware::Logger::default())
            .app_data(Data::new(tera.clone()))
            .app_data(db.clone())
            .service(views::statics::read_static)
            .service(
                web::scope("/web")
                    .configure(views::users::config),
            )
            .service(
                web::scope("/api")
                    .configure(resources::api::config),
            )
            .service(welcome)
            .route("/", web::get())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
