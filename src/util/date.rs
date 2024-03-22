use crate::rule::constant::DayOfMonth;
use chrono::{NaiveDate, Duration, Datelike};
use crate::util::error::UtilityError;


pub fn first_of_month(date: NaiveDate) -> Result<NaiveDate, UtilityError> {
    NaiveDate::from_ymd_opt(date.year(), date.month(), 1).ok_or(UtilityError::DateError(
        format!("Cannot construct first of month: {:?}", &date).into()
    ))
}

pub fn last_of_month(date: NaiveDate) -> Result<NaiveDate, UtilityError> {
    let date_construct_error = UtilityError::DateError("Cannot construct last of month".into());
    let duration = Duration::try_days(1).ok_or(UtilityError::DateError("Invalid duration".into()))?;
    match date.month() {
        12 => Ok(NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).ok_or(date_construct_error)? - duration),
        _ => Ok(NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1).ok_or(date_construct_error)? - duration)
    }
}

pub fn adjust_recurring_to_date(date: NaiveDate, day_of_month: &DayOfMonth) -> Result<NaiveDate, UtilityError> {
    //given a date and a day_of_month enum, use the month from date and day of month from day of month
    match day_of_month {
        DayOfMonth::First => first_of_month(date),
        DayOfMonth::Last => last_of_month(date),
    }
}

pub fn expiration_date_from_str_parts(year: &str, month: &str) -> Result<NaiveDate, UtilityError> {
    let month_val = month.parse::<u32>()?;
    let year_val = year.parse::<i32>()?;
    Ok(
        NaiveDate::from_ymd_opt(year_val, month_val, 1).ok_or(
            UtilityError::DateError(format!("Unable to parse date, year: {} month: {}", year, month).into())
        )?
    )


}