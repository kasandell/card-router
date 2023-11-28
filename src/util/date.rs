use crate::rule_engine::constant::DayOfMonth;
use chrono::{NaiveDate, Duration, Datelike};

pub fn first_of_month(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd(date.year(), date.month(), 1)
}

pub fn last_of_month(date: NaiveDate) -> NaiveDate {
    match date.month() {
        12 => NaiveDate::from_ymd(date.year() + 1, 1, 1) - Duration::days(1),
        _ => NaiveDate::from_ymd(date.year(), date.month() + 1, 1) - Duration::days(1)
    }
}

pub fn adjust_recurring_to_date(date: NaiveDate, day_of_month: DayOfMonth) -> NaiveDate {
    //given a date and a day_of_month enum, use the month from date and day of month from day of month
    match day_of_month {
        DayOfMonth::FIRST => first_of_month(date),
        DayOfMonth::LAST => last_of_month(date),
        _ => last_of_month(date)
    }
}