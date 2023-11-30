const BIPS: i32 = 10000;
const CENTS_TO_DOLLAR: i32 = 100;

pub fn get_cents_of_cashback(amount_cents: i32, cashback_percentage_bips: i32) -> i32 {
    (amount_cents as f64 * cashback_percentage_bips as f64 / BIPS as f64) as i32

}

pub fn get_number_of_points(amount_cents: i32, points_multiplier: i32) -> i32 {
    (amount_cents as f64 * points_multiplier as f64 / CENTS_TO_DOLLAR as f64) as i32
}