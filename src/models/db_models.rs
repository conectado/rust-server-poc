use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}
