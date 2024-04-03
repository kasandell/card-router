use crate::rule::constant::DayOfMonth;
use chrono::{NaiveDate, Duration, Datelike};
use crate::util::error::UtilityError;

const ADJUST_TO_CURRENT_CENTURY: i32 = 2000;

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
    let month_val = month.parse::<u32>().map_err(|e| UtilityError::DateError(e.into()))?;
    let mut year_val = year.parse::<i32>().map_err(|e| UtilityError::DateError(e.into()))?;
    // if given a 2 digit year, adjust it
    if (year.len() == 2) {
        year_val += ADJUST_TO_CURRENT_CENTURY;
    } else if (year.len() == 4) {
    } else {
        return Err(UtilityError::DateError(format!("{} is not a valid year", year).into()))
    }
    Ok(
        NaiveDate::from_ymd_opt(year_val, month_val, 1).ok_or(
            UtilityError::DateError(format!("Unable to parse date, year: {} month: {}", year, month).into())
        )?
    )
}

#[cfg(test)]
mod test {
    use std::ops::Add;
    use chrono::NaiveDate;
    use crate::rule::constant::DayOfMonth;
    use crate::util::date::{adjust_recurring_to_date, expiration_date_from_str_parts, first_of_month, last_of_month};
    use crate::util::error::UtilityError;

    const DAYS_OF_MONTHS: &'static [u32; 12] = &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const LEAP_DAYS_OF_MONTHS: &'static [u32; 12] = &[31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const REGULAR_YEAR: i32 = 2023;
    const LEAP_YEAR: i32 = 2024;
    const MONTHS: usize = 12;

    #[test]
    fn test_first_of_month_leap_year() {
        for month in 1..=MONTHS {
            for day in 1..= LEAP_DAYS_OF_MONTHS[month-1] {
                assert_eq!(
                    NaiveDate::from_ymd_opt(LEAP_YEAR, month as u32, 01).expect("gets date"),
                    first_of_month(
                        NaiveDate::from_ymd_opt(LEAP_YEAR, month as u32, day).expect("gets date")
                    ).expect("gets date")
                );
            }
        }
    }

    #[test]
    fn test_first_of_month_regular_year() {
        for month in 1..=MONTHS {
            for day in 1..= DAYS_OF_MONTHS[month-1] {
                assert_eq!(
                    NaiveDate::from_ymd_opt(REGULAR_YEAR, month as u32, 01).expect("gets date"),
                    first_of_month(
                        NaiveDate::from_ymd_opt(REGULAR_YEAR, month as u32, day).expect("gets date")
                    ).expect("gets date")
                );
            }
        }
    }

    #[test]
    fn test_last_of_month_regular_year() {
        for month in 1..=MONTHS {
            let last_day = DAYS_OF_MONTHS[month-1];
            for day in 1..=last_day {
                assert_eq!(
                    NaiveDate::from_ymd_opt(REGULAR_YEAR, month as u32, last_day).expect("gets date"),
                    last_of_month(
                        NaiveDate::from_ymd_opt(REGULAR_YEAR, month as u32, day).expect("gets date")
                    ).expect("gets date")
                );
            }
        }
    }

    #[test]
    fn test_last_of_month_leap_year() {
        for month in 1..=MONTHS {
            let last_day = LEAP_DAYS_OF_MONTHS[month-1];
            for day in 1..=last_day {
                assert_eq!(
                    NaiveDate::from_ymd_opt(LEAP_YEAR, month as u32, last_day).expect("gets date"),
                    last_of_month(
                        NaiveDate::from_ymd_opt(LEAP_YEAR, month as u32, day).expect("gets date")
                    ).expect("gets date")
                );
            }
        }
    }

    #[test]
    fn test_expiration_date_from_parts() {
        for month in 1..=MONTHS {
            assert_eq!(
                NaiveDate::from_ymd_opt(2024, month as u32, 1).expect("gets date"),
                expiration_date_from_str_parts("2024", &month.to_string()).expect("gets date")
            );

            assert_eq!(
                NaiveDate::from_ymd_opt(2024, month as u32, 1).expect("gets date"),
                expiration_date_from_str_parts("24", &month.to_string()).expect("gets date")
            );

            let zero_padded = if month < 10 {
                "0".to_string().add(&month.to_string())
            } else {
                month.to_string()
            };
            assert_eq!(
                NaiveDate::from_ymd_opt(2024, month as u32, 1).expect("gets date"),
                expiration_date_from_str_parts("2024", &zero_padded).expect("gets date")
            );

            assert_eq!(
                NaiveDate::from_ymd_opt(2024, month as u32, 1).expect("gets date"),
                expiration_date_from_str_parts("24", &zero_padded).expect("gets date")
            );
        }
    }

    #[test]
    fn test_expiration_date_from_parts_fails_year_parse() {
        assert_eq!(
            UtilityError::DateError("test".into()),
            expiration_date_from_str_parts("202", "1").expect_err("should be an error for invalid year")
        );

        assert_eq!(
            UtilityError::DateError("test".into()),
            expiration_date_from_str_parts("test", "1").expect_err("should be an error for invalid year")
        );

    }

    #[test]
    fn test_expiration_date_from_parts_fails_month_parse() {
        assert_eq!(
            UtilityError::DateError("test".into()),
            expiration_date_from_str_parts("2024", "whee").expect_err("should be an error for invalid year")
        );
        assert_eq!(
            UtilityError::DateError("test".into()),
            expiration_date_from_str_parts("2024", "test").expect_err("should be an error for invalid year")
        );
    }

    #[test]
    fn test_expiration_date_from_parts_fails_month_out_of_range() {
        assert_eq!(
            UtilityError::DateError("test".into()),
            expiration_date_from_str_parts("2024", "0").expect_err("should be an error for invalid year")
        );

        assert_eq!(
            UtilityError::DateError("test".into()),
            expiration_date_from_str_parts("2024", "00").expect_err("should be an error for invalid year")
        );

        assert_eq!(
            UtilityError::DateError("test".into()),
            expiration_date_from_str_parts("2024", "-1").expect_err("should be an error for invalid year")
        );

        assert_eq!(
            UtilityError::DateError("test".into()),
            expiration_date_from_str_parts("2024", "100").expect_err("should be an error for invalid year")
        );
        assert_eq!(
            UtilityError::DateError("test".into()),
            expiration_date_from_str_parts("2024", "13").expect_err("should be an error for invalid year")
        );
    }

    #[test]
    fn test_adjust_recurring_to_date_first() {
        for month in 1..=MONTHS {
            let last_day = DAYS_OF_MONTHS[month-1];
            for day in 1..=last_day {
                assert_eq!(
                    NaiveDate::from_ymd_opt(REGULAR_YEAR, month as u32, 1).expect("gets date"),
                    adjust_recurring_to_date(
                        NaiveDate::from_ymd_opt(REGULAR_YEAR, month as u32, day).expect("gets date"),
                        &DayOfMonth::First
                    ).expect("gets date")
                );
            }
        }
    }

    #[test]
    fn test_adjust_recurring_to_date_last() {
        for month in 1..=MONTHS {
            let last_day = DAYS_OF_MONTHS[month-1];
            for day in 1..=last_day {
                assert_eq!(
                    NaiveDate::from_ymd_opt(REGULAR_YEAR, month as u32, last_day).expect("gets date"),
                    adjust_recurring_to_date(
                        NaiveDate::from_ymd_opt(REGULAR_YEAR, month as u32, day).expect("gets date"),
                        &DayOfMonth::Last
                    ).expect("gets date")
                );
            }
        }
    }
}