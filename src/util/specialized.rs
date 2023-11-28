use std::hash::Hash;
use std::collections::HashSet;
use crate::wallet::entity::Wallet;

fn dedup_wallet(v: &mut Vec<Wallet>) { // note the Copy constraint
    let mut uniques = HashSet::new();
    v.retain(|e| uniques.insert(e.id));
}