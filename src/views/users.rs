use actix_web::{get, HttpResponse, post, web};
use actix_web::web::{Data, Form};
use surrealdb::Error;
use tera::{Context, Tera};
use rust_tera::database::Database;
use rust_tera::models::{NewUser, User};

#[get("/users")]
pub async fn get_users(tera: Data<Tera>, db: Data<Database>) -> HttpResponse {
    let mut context = Context::new();
    let result: Result<Vec<User>, Error> = db.client.select("users").await;

    let mut users: Vec<User> = Vec::new();
    match result {
        Ok(rows) => {
            users = Some(rows).unwrap();
        }
        Err(..) => println!("Error to get users"),
    }

    context.insert("users", &users);

    match tera.render("users/index.html", &context) {
        Ok(view) => HttpResponse::Ok().content_type("text/html").body(view),
        Err(e) => {
            println!("Error to render template: {}", e);
            HttpResponse::InternalServerError().body("Error to render template")
        }
    }
}

#[get("/users/create")]
pub async fn create(tera: Data<Tera>) -> HttpResponse {
    let context = Context::new();
    match tera.render("users/create.html", &context) {
        Ok(view) => HttpResponse::Ok().body(view),
        Err(e) => {
            println!("Error to render template: {}", e);
            HttpResponse::InternalServerError().body("Error to render template")
        }
    }
}

#[post("/users/create")]
pub async fn store(request: Form<NewUser>, tera: Data<Tera>, db: Data<Database>) -> HttpResponse {
    let created: bool;
    let new_uuid = uuid::Uuid::new_v4().to_string();
    println!("UUID: {:?}", new_uuid);
    let mut context = Context::new();
    let inserts: Result<Option<User>, Error> = db.client.create(("users", new_uuid.clone().to_string()))
        .content(User::new(
            new_uuid.clone().to_string(),
            request.name.to_string(),
            request.username.to_string(),
            request.age.to_owned(),
        )).await;
    match inserts {
        Ok(rows) => {
            println!("UUID: {:?}", rows.unwrap());
            created = true
        }
        Err(..) => {
            created = false
        }
    }
    context.insert("created", &created);
    match tera.render("users/create.html", &context) {
        Ok(view) => HttpResponse::Ok().content_type("text/html").body(view),
        Err(e) => {
            println!("Error to render template: {}", e);
            HttpResponse::InternalServerError().body("Error to render template")
        }
    }
}

#[get("/users/{uuid}/edit")]
pub async fn edit(path: web::Path<(String, )>, db: Data<Database>, tera: Data<Tera>) -> HttpResponse {
    let mut context = Context::new();
    let uuid = path.into_inner().0;
    let inserts: Result<Option<User>, Error> = db.client.select(("users", uuid.clone().to_string().as_str())).await;
    let user = match inserts {
        Ok(rows) => {
            Some(rows).unwrap()
        }
        Err(..) => {
            None
        }
    };
    context.insert("user", &user);
    match tera.render("users/edit.html", &context) {
        Ok(view) => HttpResponse::Ok().content_type("text/html").body(view),
        Err(e) => {
            println!("Error to render template: {}", e);
            HttpResponse::InternalServerError().body("Error to render template")
        }
    }
}

#[post("/users/{uuid}/edit")]
pub async fn update(path: web::Path<(String, )>, request: Form<NewUser>, db: Data<Database>, tera: Data<Tera>) -> HttpResponse {
    let updated: bool;
    let mut context = Context::new();
    let uuid = path.into_inner().0;
    let inserts: Result<Option<User>, Error> = db.client.update(("users", uuid.clone().to_string()))
        .merge(NewUser {
            name: request.name.to_string(),
            username: request.username.to_string(),
            age: request.age.to_owned(),
        }).await;
    let user = match inserts {
        Ok(rows) => {
            updated = true;
            Some(rows).unwrap()
        },
        Err(..) => {
            updated = false;
            Some(User::new("".to_string(), "".to_string(), "".to_string(), 0))
        }
    };
    context.insert("updated", &updated);
    context.insert("user", &user);
    match tera.render("users/edit.html", &context) {
        Ok(view) => HttpResponse::Ok().content_type("text/html").body(view),
        Err(e) => {
            println!("Error to render template: {}", e);
            HttpResponse::InternalServerError().body("Error to render template")
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users)
        .service(create)
        .service(store)
        .service(edit)
        .service(update);
}