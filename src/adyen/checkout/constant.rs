#[derive(Debug, PartialEq)]
pub enum AuthorizationStatus {
    AUTHORISED,
    PENDING,
    DENIED
}