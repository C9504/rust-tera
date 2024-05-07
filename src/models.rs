use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub username: String,
    pub age: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewUser {
    pub name: String,
    pub username: String,
    pub age: i32,
}

impl User {
    pub fn new(uuid: String, name: String, username: String, age: i32) -> User {
        User {
            uuid,
            name,
            username,
            age
        }
    }
}