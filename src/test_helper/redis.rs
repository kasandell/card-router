use std::ops::Add;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::redis::key::StableRedisKey;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TestStruct {
    pub field: String,
}

impl TestStruct {
    pub fn new() -> Self {
        Self {
            field: Uuid::new_v4().to_string()
        }
    }

}

pub struct TestKey{
    key: Uuid
}

impl TestKey {
    pub fn new () -> Self {
        Self {
            key: Uuid::new_v4()
        }
    }
}

impl StableRedisKey for TestKey {
    fn to_key(&self) -> String {
        "test_key_".to_string().add(self.key.to_string().as_str())
    }
}