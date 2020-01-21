// TODO merge this module with models in folder models(sub modules requests and db)

use super::password_manager::{generate_password_hash, verify_password};
use super::schema::{users, users::dsl::*};
use super::DBType;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct UserReq {
    pub username: String,
    pub password: String,
}

impl UserReq {
    pub fn register(&self, conn: &DBType) -> QueryResult<User> {
        let temp_user = UserReq {
            username: self.username.clone(),
            password: generate_password_hash(self.password.clone()),
        };
        diesel::insert_into(users::table)
            .values(temp_user)
            .get_result::<User>(conn)
    }

    pub fn login(&self, conn: &DBType) -> Result<(), ring::error::Unspecified> {
        println!("retrieveing user {}", self.username);
        let stored_user: User = users
            .filter(username.eq(&self.username))
            .first(conn)
            .unwrap();
        verify_password(stored_user.password, self.password.clone())
    }
}
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}
