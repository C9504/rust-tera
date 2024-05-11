use actix_web::{get, Responder, web};
use actix_web::web::{Data};
use log::info;
use serde_json::json;
use surrealdb::Error;
use rust_tera::database::Database;
use rust_tera::models::{ User};

#[get("/users")]
pub async fn get_users(db: Data<Database>) -> impl Responder {
    let result: Result<Vec<User>, Error> = db.client.select("users").await;

    let mut users: Vec<User> = Vec::new();
    match result {
        Ok(rows) => {
            info!("API get_users");
            users = Some(rows).unwrap();
        }
        Err(..) => {
            println!("Error to get users");
        },
    }
    web::Json(json!({"data": users}))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
}