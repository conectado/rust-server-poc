use super::db_models::User;
use crate::password_manager::{generate_password_hash, verify_password};
use crate::schema::{users, users::dsl::*};
use crate::DBType;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
            password: generate_password_hash(&self.password[..]),
        };
        diesel::insert_into(users::table)
            .values(temp_user)
            .get_result::<User>(conn)
    }

    pub fn login(&self, conn: &DBType) -> Result<User, ring::error::Unspecified> {
        let stored_user: User = users
            .filter(username.eq(&self.username))
            .first(conn)
            .unwrap();
        verify_password(&stored_user.password[..], &self.password[..]).map(|()| stored_user)
    }
}
