use crate::util::db;
use crate::schema::users;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::data_error::DataError;

#[derive(Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserMessage<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub public_id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsertableUser<'a> {
    pub public_id: &'a Uuid,
    pub email: &'a str,
    pub password: &'a str,
}

impl User {
    pub fn find_all() -> Result<Vec<Self>, DataError> {
        let mut conn = db::connection()?;

        let users = users::table
            .load::<User>(&mut conn)?;

        Ok(users)
    }

    pub fn find(id: &Uuid) -> Result<Self, DataError> {
        let mut conn = db::connection()?;

        let user = users::table
            .filter(users::public_id.eq(id))
            .first(&mut conn)?;

        Ok(user)
    }

    pub fn find_by_email_password(
        email: &str,
        password: &str
    ) -> Result<Self, DataError> {
        let mut conn = db::connection()?;

        let user = users::table
            .filter(
                users::email.eq(email)
                    .and(users::password.eq(password))
            )
            .first(&mut conn)?;

        Ok(user)
    }

    pub fn find_by_internal_id(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection()?;

        let user = users::table
            .filter(users::id.eq(id))
            .first(&mut conn)?;

        Ok(user)
    }

    pub fn create(user: &UserMessage) -> Result<Self, DataError> {
        let mut conn = db::connection()?;
        let user =  InsertableUser {
            public_id: &Uuid::new_v4(),
            email: user.email,
            password: user.password,
        };
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&mut conn)?;

        Ok(user)
    }

    pub fn update(id: &Uuid, user: &UserMessage) -> Result<Self, DataError> {
        let mut conn = db::connection()?;

        let user = diesel::update(users::table)
            .filter(users::public_id.eq(id))
            .set(user)
            .get_result(&mut conn)?;

        Ok(user)
    }

    #[cfg(test)]
    pub fn delete(id: i32) -> Result<usize, DataError> {
        let mut conn = db::connection()?;

        let res = diesel::delete(
                users::table
                    .filter(users::id.eq(id))
            )
            .execute(&mut conn)?;

        Ok(res)
    }

    #[cfg(test)]
    pub fn delete_self(&self) -> Result<usize, DataError> {
        User::delete(self.id)
    }

    #[cfg(test)]
    pub fn create_test_user(
        id: i32,
    ) -> Self {
        User {
            id: id,
            public_id: Uuid::new_v4(),
            email: "test@test.com".to_string(),
            password: "TestPassword".to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }

    #[cfg(test)]
    pub fn delete_all() -> Result<usize, DataError> {
        let mut conn = db::connection()?;

        let res = diesel::delete(
            users::table
        )
            .execute(&mut conn)?;

        Ok(res)
    }
}

/*
impl From<&UserMessage> for InsertableUser {
    fn from(user: &UserMessage) -> Self {
        InsertableUser {
            public_id: &Uuid::new_v4(),
            email: user.email,
            password: user.password,
        }
    }
}
 */