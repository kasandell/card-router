use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use uuid::Uuid;

pub type PaginationLocation = (String, i32);


pub trait PaginatableType: Serialize + DeserializeOwned + ToString + FromStr + Send + 'static{
}

impl PaginatableType for Uuid {}