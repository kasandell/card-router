use uuid::Uuid;
use crate::category::entity::{Category, MccMapping};

#[derive(Debug)]
pub struct CategoryModel {
    pub id: i32,
    pub public_id: Uuid,
    pub name: String,
}


#[derive(Debug)]
pub struct MccMappingModel {
    pub id: i32,
    pub public_id: Uuid,
    pub mcc_code: String,
    pub category_id: i32,
}

impl From<Category> for CategoryModel {
    fn from(value: Category) -> Self {
        CategoryModel {
            id: value.id,
            public_id: value.public_id,
            name: value.name
        }
    }
}

impl From<MccMapping> for MccMappingModel {
    fn from(value: MccMapping) -> Self {
        MccMappingModel {
            id: value.id,
            public_id: value.public_id,
            mcc_code: value.mcc_code,
            category_id: value.category_id
        }
    }
}