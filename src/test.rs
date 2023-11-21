use crate::util::db;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use dotenv::dotenv;

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

trait BodyTest {
    fn as_str(&self) -> &str;
}

impl BodyTest for Bytes {
    fn as_str(&self) -> &str {
        std::str::from_utf8(self).unwrap()
    }
}