use std::fmt;
use std::io::Write;
use diesel::backend::Backend;
use diesel::{deserialize, serialize};
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Integer;

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

impl From<&Category> for i32 {
    fn from(value: &Category) -> Self {
        match *value {
            Category::Hotels => 1,
            Category::CarRental => 2,
            Category::Airlines => 3,
            Category::Dining => 4,
            Category::SportingGoods => 5,
            Category::Transportation => 6
        }
    }
}


impl FromSql<Integer, Pg> for Category {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        //specify endian?
        let value = i32::from_sql(bytes)?;
        match value {
            1 => Ok(Category::Hotels),
            2 => Ok(Category::CarRental),
            3 => Ok(Category::Airlines),
            4 => Ok(Category::Dining),
            5 => Ok(Category::SportingGoods),
            6 => Ok(Category::Transportation),
            _ => Err(format!("Unknown value for RuleStatus found").into()),
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Category::Hotels => "Hotels",
            Category::CarRental => "Car Rental",
            Category::Airlines => "Airlines",
            Category::Dining => "Dining",
            Category::SportingGoods => "Sporting Goods",
            Category::Transportation => "Transportation",
        })
    }
}


impl ToSql<Integer, Pg> for Category {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        ToSql::<Integer, Pg>::to_sql(&*self, out)?;
        Ok(IsNull::No)
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