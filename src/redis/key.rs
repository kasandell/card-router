use itertools::Itertools;
/// A trait representation that implements stable redis key names for common uses
pub trait StableRedisKey {
    fn to_key(&self) -> String;
}

/// Common redis key names and operations
pub enum Key<'a> {
    User(i32),
    MccMapping(&'a str),
    CardsForUser(i32),
    RulesForCards(&'a Vec<i32>),
    PassthroughCardByToken(&'a str)
}

impl StableRedisKey for Key<'_> {
    fn to_key(&self) -> String {
        match self {
            Key::User(id) => format!("user_{}", id),
            Key::MccMapping(mapping) => format!("mcc_mapping_{}", mapping),
            Key::CardsForUser(id) => format!("cards_for_user_{}", id),
            Key::RulesForCards(cards_ids) => {
                let unique: Vec<_> = cards_ids
                    .iter()
                    .unique()
                    .sorted()
                    .map(|x| x.to_string())
                    .collect();
                let joined = unique.join("-");
                format!("card_{}", joined)

            },
            Key::PassthroughCardByToken(token) => {
                format!("passthrough_card_{}", token)
            }
        }
    }
}


#[cfg(test)]
mod test {
    use crate::redis::key::{Key, StableRedisKey};

    #[test]
    fn test_user() {
        assert_eq!("user_1".to_string(), Key::User(1).to_key());
        assert_eq!("user_576".to_string(), Key::User(576).to_key());
    }

    #[test]
    fn test_mcc_mapping() {
        assert_eq!("mcc_mapping_5748".to_string(), Key::MccMapping("5748").to_key());
        assert_eq!("mcc_mapping_1234".to_string(), Key::MccMapping("1234").to_key());
    }
    #[test]
    fn test_cards_for_user() {
        assert_eq!("cards_for_user_1234".to_string(), Key::CardsForUser(1234).to_key());
        assert_eq!("cards_for_user_5678".to_string(), Key::CardsForUser(5678).to_key());

    }

    #[test]
    fn test_passthrough_card_by_token() {
        assert_eq!("passthrough_card_1234".to_string(), Key::PassthroughCardByToken("1234").to_key());
        assert_eq!("passthrough_card_1234-5678".to_string(), Key::PassthroughCardByToken("1234-5678").to_key());
    }


    #[test]
    fn test_rules_for_cards() {
        assert_eq!("card_1".to_string(), Key::RulesForCards(&vec![1, 1, 1, 1, 1]).to_key());
        assert_eq!("card_1-2".to_string(), Key::RulesForCards(&vec![1, 2, 2]).to_key());
        assert_eq!("card_1-2".to_string(), Key::RulesForCards(&vec![2, 1]).to_key());
        assert_eq!("card_1-2".to_string(), Key::RulesForCards(&vec![1, 2, 1]).to_key());
        assert_eq!("card_1-2".to_string(), Key::RulesForCards(&vec![2, 1, 1]).to_key());
        assert_eq!("card_1-2-600".to_string(), Key::RulesForCards(&vec![2, 1, 1, 600]).to_key());
    }
}