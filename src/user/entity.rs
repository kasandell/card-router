use crate::api_error::ApiError;
use crate::util::db;
use crate::schema::users;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserMessage {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub public_id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]

pub struct InsertableUser {
    pub public_id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let mut conn = db::connection()?;

        let users = users::table
            .load::<User>(&mut conn)?;

        Ok(users)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;

        let user = users::table
            .filter(users::public_id.eq(id))
            .first(&mut conn)?;

        Ok(user)
    }

    pub fn find_by_internal_id(id: i32) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;

        let user = users::table
            .filter(users::id.eq(id))
            .first(&mut conn)?;

        Ok(user)
    }

    pub fn create(user: UserMessage) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;

        let user = InsertableUser::from(user);
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&mut conn)?;

        Ok(user)
    }

    pub fn update(id: Uuid, user: UserMessage) -> Result<Self, ApiError> {
        let mut conn = db::connection()?;

        let user = diesel::update(users::table)
            .filter(users::public_id.eq(id))
            .set(user)
            .get_result(&mut conn)?;

        Ok(user)
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let mut conn = db::connection()?;

        let res = diesel::delete(
                users::table
                    .filter(users::public_id.eq(id))
            )
            .execute(&mut conn)?;

        Ok(res)
    }

    pub fn delete_self(&self) -> Result<usize, ApiError> {
        User::delete(self.public_id)
    }
}

impl From<UserMessage> for InsertableUser {
    fn from(user: UserMessage) -> Self {
        InsertableUser {
            public_id: Uuid::new_v4(),
            email: user.email,
            password: user.password,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}