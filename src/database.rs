use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::{Error, Surreal};
//use surrealdb::opt::auth::Root;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub namespace: String,
    pub db_name: String,
}

impl Database {
    pub async fn init(url: &str, ns: &str, db: &str) -> Result<Self, Error> {
        let client = Surreal::new::<Ws>(url).await?;
        /*client.signin(Root {
            username: "root",
            password: "root",
        }).await?;*/
        client.use_ns(ns).use_db(db).await.unwrap();
        Ok(Database {
            client,
            namespace: String::from(ns),
            db_name: String::from(db),
        })
    }
}