use crate::rule_engine::constant::DayOfMonth;
use chrono::{NaiveDate, Duration, Datelike};
use crate::api_error::ApiError;
use crate::data_error::DataError;
use crate::error_type::ErrorType;

pub fn first_of_month(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.year(), date.month(), 1).expect("should create date")
}

pub fn last_of_month(date: NaiveDate) -> NaiveDate {
    match date.month() {
        12 => NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).expect("should create date") - Duration::days(1),
        _ => NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1).expect("should create date") - Duration::days(1)
    }
}

pub fn adjust_recurring_to_date(date: NaiveDate, day_of_month: DayOfMonth) -> NaiveDate {
    //given a date and a day_of_month enum, use the month from date and day of month from day of month
    match day_of_month {
        DayOfMonth::FIRST => first_of_month(date),
        DayOfMonth::LAST => last_of_month(date),
    }
}

pub fn expiration_date_from_str_parts(year: &str, month: &str) -> Result<NaiveDate, DataError> {
    let month_val = month.parse::<u32>()?;
    let year_val = year.parse::<i32>()?;
    Ok(
        NaiveDate::from_ymd_opt(year_val, month_val, 1).ok_or(
            DataError::new(ErrorType::InternalServerError, "Parse error".to_string())
        )?
    )


}