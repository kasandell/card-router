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

#[cfg(test)]
mod test {
    use crate::category::entity::{Category, MccMapping};
    use crate::category::model::{CategoryModel, MccMappingModel};

    #[test]
    fn test_from_mcc() {
        let mcc_entity = MccMapping {
            id: 69,
            public_id: Default::default(),
            mcc_code: "1234".to_string(),
            category_id: 1,
            created_at: Default::default(),
            updated_at: Default::default(),
        };
        let mcc_model = MccMappingModel::from(mcc_entity.clone());
        assert_eq!(mcc_entity.id, mcc_model.id);
        assert_eq!(mcc_entity.public_id, mcc_model.public_id);
        assert_eq!(mcc_entity.mcc_code, mcc_model.mcc_code);
        assert_eq!(mcc_entity.category_id, mcc_model.category_id);
    }
    
    #[test]
    fn test_from_category() {
        let category_entity = Category {
            id: 69,
            public_id: Default::default(),
            name: "lol".to_string(),
            created_at: Default::default(),
            updated_at: Default::default(),
        };
        let category_model = CategoryModel::from(category_entity.clone());
        assert_eq!(category_entity.id, category_model.id);
        assert_eq!(category_entity.public_id, category_model.public_id);
        assert_eq!(category_entity.name, category_model.name);

    }
}