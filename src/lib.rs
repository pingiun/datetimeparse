//! Correct ISO 8601 and RFC3999 parsing and formatting.

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
/// ## Example
/// ```rust
/// # use datetimeparse::parse_rfc3339_datetime;
/// # use datetimeparse::StandardYear;
/// let dt = parse_rfc3339_datetime("2023-09-17T09:08:58.763072Z").unwrap();
/// assert_eq!(dt.year, StandardYear::new(2023).unwrap());
/// ```
pub fn parse_rfc3339_datetime(inp: &str) -> Result<PreciseShiftedDateTime, Error<'_>> {
    let mut parser = parse::ParseContext::new_rfc3339().into_parser();
    parser.parse_precise_shifted_date_time(inp.as_bytes())?;
    Ok(parser.build_precise_shifted_date_time()?)
}

#[cfg(test)]
mod test_parse_rfc3339_datetime {
    use crate::parse_rfc3339_datetime;

    #[test]
    fn parse_all_from_file() {
        let datafile = include_str!("../data/date-test-values-rfc.txt");
        for line in datafile.lines() {
            let dt = parse_rfc3339_datetime(line);
            assert!(dt.is_ok(), "{}: {:?}", line, dt);
        }
    }
}
