use crate::util::db;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use dotenv::dotenv;
use actix_web::web::Bytes;
use serde_json::Value;

lazy_static! {
   static ref INITIATED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

#[cfg(test)]
pub fn init() {
   let mut initiated = INITIATED.lock().unwrap();
   if *initiated == false {
       dotenv().ok();
       db::init();
       *initiated = true;
   }
}


#[cfg(test)]
pub trait BodyTest {
    fn as_str(&self) -> &str;
    fn as_json(&self) -> Value;
}

#[cfg(test)]
impl BodyTest for Bytes {
    fn as_str(&self) -> &str {
        std::str::from_utf8(self).unwrap()
    }

    fn as_json(&self) -> Value {
        serde_json::from_str(self.as_str()).unwrap()
    }
}