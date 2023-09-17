//! Correct ISO 8601 and RFC3999 parsing and formatting.
//!
//! ## Features
//! - `chrono` - Enable chrono conversion

mod combined;
mod components;
mod parse;

mod parse_utils;

pub use components::{
    Day, Hour, Minute, Month, Nanosecond, NonNegative, Second, StandardYear, WithNegative, Year,
};

pub use combined::{
    LocalDate, LocalDateTime, LocalTime, PreciseLocalDateTime, PreciseLocalTime,
    PreciseShiftedDateTime, ShiftedDateTime,
};

/// ISO 8601, 4.3 duration elements
pub mod duration {
    pub use crate::components::{
        DayDuration, HourDuration, MinuteDuration, MonthDuration, SecondDuration, YearDuration,
    };
}

#[derive(Debug)]
#[non_exhaustive]
pub struct Error<'a> {
    pub kind: ErrorKind<'a>,
}

#[derive(Debug)]
pub enum ErrorKind<'a> {
    ParseError(parse_utils::ParseError<'a>),
    BuildError(parse::BuildError),
}

impl<'a> From<parse::BuildError> for Error<'a> {
    fn from(value: parse::BuildError) -> Self {
        Error {
            kind: ErrorKind::BuildError(value),
        }
    }
}

impl<'a> From<parse_utils::ParseError<'a>> for Error<'a> {
    fn from(value: parse_utils::ParseError<'a>) -> Self {
        Error {
            kind: ErrorKind::ParseError(value),
        }
    }
}

/// Parse a RFC3339 formatted datetime string.
///
/// This follows the liberal parsing rules of RFC3339, and will accept
/// both uppercase and lowercase letters for the T and Z separators. Also,
/// the fractional seconds part is optional and the T separator can be replaced
/// with a space.
///
/// ## Example
/// ```rust
/// # use datetimeparse::parse_rfc3339_datetime;
/// # use datetimeparse::{StandardYear, Month, Day, Hour, Minute, Second, Nanosecond};
/// let dt = parse_rfc3339_datetime("2023-09-17T09:08:58.763072Z").unwrap();
/// assert_eq!(dt.year, StandardYear::new(2023).unwrap());
/// assert_eq!(dt.month, Month::new(9).unwrap());
/// assert_eq!(dt.day, Day::new(17).unwrap());
/// assert_eq!(dt.hour, Hour::new(9).unwrap());
/// assert_eq!(dt.minute, Minute::new(8).unwrap());
/// assert_eq!(dt.second, Second::new(58).unwrap());
/// assert_eq!(dt.nanosecond, Nanosecond::new(763072000).unwrap());
/// ```
pub fn parse_rfc3339_datetime(inp: &str) -> Result<PreciseShiftedDateTime, Error<'_>> {
    let mut parser = parse::ParseContext::new_rfc3339().into_parser();
    parser.parse_precise_shifted_date_time(inp.as_bytes())?;
    Ok(parser.build_precise_shifted_date_time()?)
}

/// Parse a RFC3339 formatted date string.
///
/// ## Example
/// ```rust
/// # use datetimeparse::parse_rfc3339_date;
/// # use datetimeparse::{StandardYear, Month, Day};
/// let dt = parse_rfc3339_date("2023-09-17").unwrap();
/// assert_eq!(dt.year, StandardYear::new(2023).unwrap());
/// assert_eq!(dt.month, Month::new(9).unwrap());
/// assert_eq!(dt.day, Day::new(17).unwrap());
/// ```
pub fn parse_rfc3339_date(inp: &str) -> Result<LocalDate, Error<'_>> {
    let mut parser = parse::ParseContext::new_rfc3339().into_parser();
    parser.parse_date(inp.as_bytes())?;
    Ok(parser.build_date()?)
}

/// Parse a RFC3339 formatted time string.
///
/// ## Example
/// ```rust
/// # use datetimeparse::parse_rfc3339_time;
/// # use datetimeparse::{Hour, Minute, Second, Nanosecond};
/// let dt = parse_rfc3339_time("09:08:58.763072").unwrap();
/// assert_eq!(dt.hour, Hour::new(9).unwrap());
/// assert_eq!(dt.minute, Minute::new(8).unwrap());
/// assert_eq!(dt.second, Second::new(58).unwrap());
/// assert_eq!(dt.nanosecond, Nanosecond::new(763072000).unwrap());
/// ```
pub fn parse_rfc3339_time(inp: &str) -> Result<PreciseLocalTime, Error<'_>> {
    let mut parser = parse::ParseContext::new_rfc3339().into_parser();
    parser.parse_precise_local_time(inp.as_bytes())?;
    Ok(parser.build_precise_local_time()?)
}

#[cfg(test)]
mod test_parse_rfc3339_datetime {
    use crate::{parse_rfc3339_datetime, parse_rfc3339_time};

    #[test]
    fn parse_all_datetime_from_file() {
        let datafile = include_str!("../data/datetime-test-values-rfc.txt");
        for line in datafile.lines() {
            let dt = parse_rfc3339_datetime(line);
            assert!(dt.is_ok(), "{}: {:?}", line, dt);
        }
    }

    #[test]
    fn parse_all_time_from_file() {
        let datafile = include_str!("../data/time-test-values-rfc.txt");
        for line in datafile.lines() {
            let dt = parse_rfc3339_time(line);
            assert!(dt.is_ok(), "{}: {:?}", line, dt);
        }
    }
}
