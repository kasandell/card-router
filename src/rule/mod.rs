pub mod service;
pub mod constant;
pub mod request;
pub mod error;
#[cfg(test)]
pub mod entity;
#[cfg(not(test))]
mod entity;

mod entity_tests;
mod tests;
mod dao;
mod consant_tests;