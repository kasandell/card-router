/*
pub fn dedup_wallet<'a>(v: &'a mut Vec<&Wallet>) { // note the Copy constraint
    let mut uniques = HashSet::new();
    v.retain(|e| uniques.insert(e.id));
}
 */