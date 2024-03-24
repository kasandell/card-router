#[derive(Debug, Clone)]
pub enum Category {
    Hotels = 1,
    CarRental = 2,
    Airlines = 3,
    Dining = 4,
    SportingGoods = 5,
    Transportation = 6,
}

impl From<Category> for i32 {
    fn from(value: Category) -> Self {
        match value {
            Category::Hotels => 1,
            Category::CarRental => 2,
            Category::Airlines => 3,
            Category::Dining => 4,
            Category::SportingGoods => 5,
            Category::Transportation => 6
        }
    }
}