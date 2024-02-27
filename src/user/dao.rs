use uuid::Uuid;
use crate::data_error::DataError;
use crate::user::entity::{User, UserMessage};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait UserDaoTrait {
    fn find_all(&self) -> Result<Vec<User>, DataError>;
    fn find(&self, id: Uuid) -> Result<User, DataError>;
    fn find_by_email_password(
        &self,
        email: &str,
        password: &str
    ) -> Result<User, DataError>;
    fn find_by_internal_id(&self, id: i32) -> Result<User, DataError>;
    fn create(&self, user: UserMessage) -> Result<User, DataError>;
    fn update(&self, id: Uuid, user: UserMessage) -> Result<User, DataError>;
}

pub struct UserDao {}

impl UserDao {
    pub fn new() -> Self {
        Self {}
    }
}

impl UserDaoTrait for UserDao {
    fn find_all(&self) -> Result<Vec<User>, DataError> {
        User::find_all()
    }

    fn find(&self, id: Uuid) -> Result<User, DataError> {
        User::find(id)
    }

    fn find_by_email_password(
        &self,
        email: &str,
        password: &str
    ) -> Result<User, DataError> {
        User::find_by_email_password(email, password)
    }

    fn find_by_internal_id(&self, id: i32) -> Result<User, DataError> {
        User::find_by_internal_id(id)
    }

    fn create(&self, user: UserMessage) -> Result<User, DataError> {
        User::create(user)
    }

    fn update(&self, id: Uuid, user: UserMessage) -> Result<User, DataError> {
        User::update(id, user)
    }
}