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

#[cfg(test)]
mod test {
    use crate::category::constant::Category;

    #[test]
    fn test_conversions() {
        assert_eq!(1, Category::Hotels as i32);
        assert_eq!(2, Category::CarRental as i32);
        assert_eq!(3, Category::Airlines as i32);
        assert_eq!(4, Category::Dining as i32);
        assert_eq!(5, Category::SportingGoods as i32);
        assert_eq!(6, Category::Transportation as i32);
    }
}