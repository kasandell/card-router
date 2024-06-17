use crate::pagination::r#type::PaginatableType;

pub struct PaginationModel <T> where T: PaginatableType {
    pub service_name: String,
    pub column_name: String,
    pub cursor_location: T
}
