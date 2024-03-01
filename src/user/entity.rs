use crate::util::db;
use crate::schema::users;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
#[cfg(not(test))]
use diesel_async::RunQueryDsl;
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
    pub async fn find_all() -> Result<Vec<Self>, DataError> {
        let mut conn = db::connection().await?;

        let users = users::table
            .load::<User>(&mut conn).await?;

        Ok(users)
    }

    pub async fn find(id: &Uuid) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let user = users::table
            .filter(users::public_id.eq(id))
            .first(&mut conn).await?;

        Ok(user)
    }

    pub async fn find_by_email_password(
        email: &str,
        password: &str
    ) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let user = users::table
            .filter(
                users::email.eq(email)
                    .and(users::password.eq(password))
            )
            .first(&mut conn).await?;

        Ok(user)
    }

    pub async fn find_by_internal_id(id: i32) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let user = users::table
            .filter(users::id.eq(id))
            .first(&mut conn).await?;

        Ok(user)
    }

    pub async fn create<'a>(user: &UserMessage<'a>) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;
        let user =  InsertableUser {
            public_id: &Uuid::new_v4(),
            email: user.email,
            password: user.password,
        };
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&mut conn).await?;

        Ok(user)
    }

    pub async fn update<'a>(id: &Uuid, user: &UserMessage<'a>) -> Result<Self, DataError> {
        let mut conn = db::connection().await?;

        let user = diesel::update(users::table)
            .filter(users::public_id.eq(id))
            .set(user)
            .get_result(&mut conn).await?;

        Ok(user)
    }

    #[cfg(test)]
    pub async fn delete(id: i32) -> Result<usize, DataError> {
        let mut conn = db::connection().await?;

        let res = diesel::delete(
                users::table
                    .filter(users::id.eq(id))
            )
            .execute(&mut conn).await?;

        Ok(res)
    }

    #[cfg(test)]
    pub async fn delete_self(&self) -> Result<usize, DataError> {
        User::delete(self.id)
    }

    #[cfg(test)]
    pub async fn create_test_user(
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
    pub async fn delete_all() -> Result<usize, DataError> {
        let mut conn = db::connection().await?;

        let res = diesel::delete(
            users::table
        )
            .execute(&mut conn).await?;

        Ok(res)
    }
}

/*
impl From<&UserMessage> for InsertableUser {
    async fn from(user: &UserMessage) -> Self {
        InsertableUser {
            public_id: &Uuid::new_v4(),
            email: user.email,
            password: user.password,
        }
    }
}
 */