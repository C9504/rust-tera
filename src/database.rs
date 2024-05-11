use std::env;
use dotenvy::dotenv;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::{Error, Surreal};
use surrealdb::opt::auth::Root;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub namespace: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        dotenv().ok();
        let environment = env::var("ENVIRONMENT").expect("ENVIRONMENT must be set");
        let db_url = env::var("SURREAL_URL").expect("SURREAL_URL must be set");
        let db_user = env::var("SURREAL_USER").expect("SURREAL_USER must be set");
        let db_password = env::var("SURREAL_PASSWORD").expect("SURREAL_PASSWORD must be set");
        let db_namespace = env::var("SURREAL_NAMESPACE").expect("SURREAL_NAMESPACE must be set");
        let db_name = env::var("SURREAL_DB").expect("SURREAL_DB must be set");
        let client = Surreal::new::<Ws>(db_url).await?;
        if environment == "production" {
            client.signin(Root {
                username: &db_user,
                password: &db_password,
            }).await?;
        }
        client.use_ns(db_namespace.clone()).use_db(db_name.clone()).await.unwrap();
        Ok(Database {
            client,
            namespace: String::from(&db_namespace.clone()),
            db_name: String::from(&db_name.clone()),
        })
    }
}