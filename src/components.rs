use core::{fmt, num, str};

#[derive(Debug)]
pub enum Error {
    RangeError,
    ParseIntError(num::ParseIntError),
    ParseError,
}

/// Marker struct for [`Year`] to signify no negative possibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonNegative;
/// Marker struct for [`Year`] to signify a negative possibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WithNegative;

/// Extendable year to allow for negative years and more than 4 digits
///
/// See [`StandardYear`] for simple year usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Year<const N: usize = 4, T = NonNegative>(i32, T);

/// Standard 4 digit, non-negative year
///
/// See [`Year`]
pub type StandardYear = Year<4, NonNegative>;

impl<const N: usize> Year<N, WithNegative> {
    pub fn new(year: i32) -> Result<Self, Error> {
        if year == 0 {
            return Ok(Self(year, WithNegative));
        }
        let year = match year.checked_abs().and_then(|x| x.checked_ilog10()) {
            Some(num) if (num as usize) < N => year,
            Some(_) => return Err(Error::RangeError),
            None => return Err(Error::RangeError),
        };
        Ok(Self(year, WithNegative))
    }
}

impl<const N: usize> Year<N, NonNegative> {
    pub fn new(year: i32) -> Result<Self, Error> {
        if year < 0 {
            return Err(Error::RangeError);
        }
        if year == 0 {
            return Ok(Self(year, NonNegative));
        }
        let year = match year.checked_abs().and_then(|x| x.checked_ilog10()) {
            Some(num) if (num as usize) < N => year,
            Some(_) => return Err(Error::RangeError),
            None => return Err(Error::RangeError),
        };
        Ok(Self(year, NonNegative))
    }
}

#[cfg(test)]
mod year_test {
    use super::{StandardYear, WithNegative, Year};

    #[test]
    fn test_year_4_digits() {
        assert!(StandardYear::new(0).is_ok());
        assert!(StandardYear::new(1).is_ok());
        assert!(StandardYear::new(9999).is_ok());
        assert!(StandardYear::new(10000).is_err());
        assert_eq!(format!("{}", StandardYear::new(1).unwrap()), "0001");
    }

    #[test]
    fn test_negative_years() {
        assert!(StandardYear::new(-1).is_err());
        assert!(Year::<6, WithNegative>::new(-1).is_ok());
        assert_eq!(
            format!("{}", Year::<6, WithNegative>::new(-1).unwrap()),
            "-000001"
        );
    }

    #[test]
    fn test_big_years() {
        assert!(Year::<6, WithNegative>::new(100000).is_ok());
        assert_eq!(
            format!("{}", Year::<6, WithNegative>::new(1).unwrap()),
            "+000001"
        );
    }
}

impl<const N: usize> fmt::Display for Year<N, NonNegative> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>width$}", self.0, width = N)
    }
}

impl<const N: usize> fmt::Display for Year<N, WithNegative> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{:0>width$}",
            if self.0 < 0 { "-" } else { "+" },
            (self.0 as i64).abs(),
            width = N
        )
    }
}

macro_rules! impl_try_from {
    ($primitive:ty, $structtype:ident) => {
        impl TryFrom<$primitive> for $structtype {
            type Error = Error;
            #[allow(unused_comparisons)]
            fn try_from(value: $primitive) -> Result<Self, Self::Error> {
                if value < 0 {
                    return Err(Error::RangeError);
                }
                $structtype::new(value as i32)
            }
        }
    };
}

impl_try_from!(u8, StandardYear);
impl_try_from!(u16, StandardYear);
impl_try_from!(u32, StandardYear);
impl_try_from!(u64, StandardYear);
impl_try_from!(i8, StandardYear);
impl_try_from!(i16, StandardYear);
impl_try_from!(i32, StandardYear);
impl_try_from!(i64, StandardYear);

macro_rules! impl_into {
    ($primitive:ty, $structtype:ident) => {
        impl Into<$primitive> for $structtype {
            fn into(self) -> $primitive {
                self.0 as $primitive
            }
        }
    };
}

impl_into!(i32, Year);
impl_into!(i64, Year);

/// An amount of years
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct YearDuration(u64);

impl YearDuration {
    pub fn new(year: u64) -> Self {
        Self(year)
    }
}

impl fmt::Display for YearDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}Y", self.0))
    }
}

impl str::FromStr for YearDuration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.strip_suffix("Y")
            .ok_or_else(|| Error::ParseError)
            .and_then(|s| s.parse().map_err(|e| Error::ParseIntError(e)))
            .map(|x| Self::new(x))
    }
}

macro_rules! impl_from {
    ($primitive:ty, $structtype:ident) => {
        impl From<$primitive> for $structtype {
            fn from(value: $primitive) -> Self {
                Self::new(value as u64)
            }
        }
    };
}

impl_from!(u8, YearDuration);
impl_from!(u16, YearDuration);
impl_from!(u32, YearDuration);
impl_from!(u64, YearDuration);

impl_into!(u64, YearDuration);

/// Month of the year (1-12)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Month(u8);

impl Month {
    pub fn new(month: u64) -> Result<Self, Error> {
        if month == 0 {
            return Err(Error::RangeError);
        }
        if month > 12 {
            return Err(Error::RangeError);
        }
        Ok(Self(month as u8))
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}", self.0)
    }
}

macro_rules! impl_try_from {
    ($primitive:ty, $structtype:ident) => {
        impl TryFrom<$primitive> for $structtype {
            type Error = Error;
            #[allow(unused_comparisons)]
            fn try_from(value: $primitive) -> Result<Self, Self::Error> {
                if value < 0 {
                    return Err(Error::RangeError);
                }
                $structtype::new(value as u64)
            }
        }
    };
}

impl_try_from!(u8, Month);
impl_try_from!(u16, Month);
impl_try_from!(u32, Month);
impl_try_from!(u64, Month);
impl_try_from!(i8, Month);
impl_try_from!(i16, Month);
impl_try_from!(i32, Month);
impl_try_from!(i64, Month);

impl_into!(u8, Month);
impl_into!(u16, Month);
impl_into!(u32, Month);
impl_into!(u64, Month);
impl_into!(i16, Month);
impl_into!(i32, Month);
impl_into!(i64, Month);

/// An amount of months
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MonthDuration(u64);

impl MonthDuration {
    pub fn new(month: u64) -> Self {
        Self(month)
    }
}

impl fmt::Display for MonthDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}M", self.0))
    }
}

impl str::FromStr for MonthDuration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.strip_suffix("M")
            .ok_or_else(|| Error::ParseError)
            .and_then(|s| s.parse().map_err(|e| Error::ParseIntError(e)))
            .map(|x| Self::new(x))
    }
}

macro_rules! impl_from {
    ($primitive:ty, $structtype:ident) => {
        impl From<$primitive> for $structtype {
            fn from(value: $primitive) -> Self {
                Self::new(value as u64)
            }
        }
    };
}

impl_from!(u8, MonthDuration);
impl_from!(u16, MonthDuration);
impl_from!(u32, MonthDuration);
impl_from!(u64, MonthDuration);

impl_into!(u64, MonthDuration);

/// Week of the year (1-53)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Week(u8);

impl Week {
    pub fn new(week: u64) -> Result<Self, Error> {
        if week > 53 {
            return Err(Error::RangeError);
        }
        Ok(Self(week as u8))
    }
}

impl fmt::Display for Week {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "W{:0>2}", self.0)
    }
}

impl_try_from!(u8, Week);
impl_try_from!(u16, Week);
impl_try_from!(u32, Week);
impl_try_from!(u64, Week);
impl_try_from!(i8, Week);
impl_try_from!(i16, Week);
impl_try_from!(i32, Week);
impl_try_from!(i64, Week);

impl_into!(u8, Week);
impl_into!(u16, Week);
impl_into!(u32, Week);
impl_into!(u64, Week);
impl_into!(i16, Week);
impl_into!(i32, Week);
impl_into!(i64, Week);

/// An amount of weeks
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WeekDuration(u64);

impl WeekDuration {
    pub fn new(week: u64) -> Self {
        Self(week)
    }
}

impl fmt::Display for WeekDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}W", self.0)
    }
}

impl str::FromStr for WeekDuration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.strip_suffix("W")
            .ok_or_else(|| Error::ParseError)
            .and_then(|s| s.parse().map_err(|e| Error::ParseIntError(e)))
            .map(|x| Self::new(x))
    }
}

impl Into<std::time::Duration> for WeekDuration {
    fn into(self) -> std::time::Duration {
        std::time::Duration::from_secs(self.0 * 60 * 60 * 24 * 7)
    }
}

impl_from!(u8, WeekDuration);
impl_from!(u16, WeekDuration);
impl_from!(u32, WeekDuration);
impl_from!(u64, WeekDuration);

impl_into!(u64, WeekDuration);

/// Day of the month (1-31)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Day(u8);

impl Day {
    pub fn new(day: u64) -> Result<Self, Error> {
        if day == 0 {
            return Err(Error::RangeError);
        }
        if day > 31 {
            return Err(Error::RangeError);
        }
        Ok(Self(day as u8))
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}", self.0)
    }
}

macro_rules! impl_try_from {
    ($primitive:ty, $structtype:ident) => {
        impl TryFrom<$primitive> for $structtype {
            type Error = Error;
            #[allow(unused_comparisons)]
            fn try_from(value: $primitive) -> Result<Self, Self::Error> {
                if value < 0 {
                    return Err(Error::RangeError);
                }
                $structtype::new(value as u64)
            }
        }
    };
}

impl_try_from!(u8, Day);
impl_try_from!(u16, Day);
impl_try_from!(u32, Day);
impl_try_from!(u64, Day);
impl_try_from!(i8, Day);
impl_try_from!(i16, Day);
impl_try_from!(i32, Day);
impl_try_from!(i64, Day);

impl_into!(u8, Day);
impl_into!(u16, Day);
impl_into!(u32, Day);
impl_into!(u64, Day);
impl_into!(i16, Day);
impl_into!(i32, Day);
impl_into!(i64, Day);

/// An amount of days
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DayDuration(u64);

impl DayDuration {
    pub fn new(day: u64) -> Self {
        Self(day)
    }
}

impl fmt::Display for DayDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}D", self.0)
    }
}

impl str::FromStr for DayDuration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.strip_suffix("D")
            .ok_or_else(|| Error::ParseError)
            .and_then(|s| s.parse().map_err(|e| Error::ParseIntError(e)))
            .map(|x| Self::new(x))
    }
}

impl Into<std::time::Duration> for DayDuration {
    fn into(self) -> std::time::Duration {
        std::time::Duration::from_secs(self.0 * 60 * 60 * 24)
    }
}

impl_from!(u8, DayDuration);
impl_from!(u16, DayDuration);
impl_from!(u32, DayDuration);
impl_from!(u64, DayDuration);

impl_into!(u64, DayDuration);

/// Hours (0-60)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hour(u8);

impl Hour {
    pub fn new(hour: u64) -> Result<Hour, Error> {
        if hour > 24 {
            return Err(Error::RangeError);
        }
        Ok(Hour(hour as u8))
    }
}

impl fmt::Display for Hour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}", self.0)
    }
}

macro_rules! impl_try_from {
    ($primitive:ty, $structtype:ident) => {
        impl TryFrom<$primitive> for $structtype {
            type Error = Error;
            #[allow(unused_comparisons)]
            fn try_from(value: $primitive) -> Result<Self, Self::Error> {
                if value < 0 {
                    return Err(Error::RangeError);
                }
                $structtype::new(value as u64)
            }
        }
    };
}

impl_try_from!(u8, Hour);
impl_try_from!(u16, Hour);
impl_try_from!(u32, Hour);
impl_try_from!(u64, Hour);
impl_try_from!(i8, Hour);
impl_try_from!(i16, Hour);
impl_try_from!(i32, Hour);
impl_try_from!(i64, Hour);

impl_into!(u8, Hour);
impl_into!(u16, Hour);
impl_into!(u32, Hour);
impl_into!(u64, Hour);
impl_into!(i16, Hour);
impl_into!(i32, Hour);
impl_into!(i64, Hour);

/// An amount of hours
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HourDuration(u64);

impl HourDuration {
    pub fn new(hour: u64) -> Self {
        Self(hour)
    }
}

impl fmt::Display for HourDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}H", self.0)
    }
}

impl str::FromStr for HourDuration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.strip_suffix("H")
            .ok_or_else(|| Error::ParseError)
            .and_then(|s| s.parse().map_err(|e| Error::ParseIntError(e)))
            .map(|x| Self::new(x))
    }
}

impl Into<std::time::Duration> for HourDuration {
    fn into(self) -> std::time::Duration {
        std::time::Duration::from_secs(self.0 * 60 * 60)
    }
}

impl_from!(u8, HourDuration);
impl_from!(u16, HourDuration);
impl_from!(u32, HourDuration);
impl_from!(u64, HourDuration);

impl_into!(u64, HourDuration);

/// Minutes (0-60)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Minute(u8);

impl Minute {
    pub fn new(minute: u64) -> Result<Minute, Error> {
        if minute > 60 {
            return Err(Error::RangeError);
        }
        Ok(Minute(minute as u8))
    }
}

impl fmt::Display for Minute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}", self.0)
    }
}

impl_try_from!(u8, Minute);
impl_try_from!(u16, Minute);
impl_try_from!(u32, Minute);
impl_try_from!(u64, Minute);
impl_try_from!(i8, Minute);
impl_try_from!(i16, Minute);
impl_try_from!(i32, Minute);
impl_try_from!(i64, Minute);

impl_into!(u8, Minute);
impl_into!(u16, Minute);
impl_into!(u32, Minute);
impl_into!(u64, Minute);
impl_into!(i16, Minute);
impl_into!(i32, Minute);
impl_into!(i64, Minute);

/// An amount of minutes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MinuteDuration(u64);

impl MinuteDuration {
    pub fn new(hour: u64) -> Self {
        Self(hour)
    }
}

impl fmt::Display for MinuteDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}M", self.0)
    }
}

impl str::FromStr for MinuteDuration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.strip_suffix("M")
            .ok_or_else(|| Error::ParseError)
            .and_then(|s| s.parse().map_err(|e| Error::ParseIntError(e)))
            .map(|x| Self::new(x))
    }
}

impl Into<std::time::Duration> for MinuteDuration {
    fn into(self) -> std::time::Duration {
        std::time::Duration::from_secs(self.0 * 60)
    }
}

impl_from!(u8, MinuteDuration);
impl_from!(u16, MinuteDuration);
impl_from!(u32, MinuteDuration);
impl_from!(u64, MinuteDuration);

impl_into!(u64, MinuteDuration);

/// Seconds (0-61)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Second(u8);

impl Second {
    pub fn new(second: u64) -> Result<Second, Error> {
        if second > 61 {
            return Err(Error::RangeError);
        }
        Ok(Second(second as u8))
    }
}

impl fmt::Display for Second {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}", self.0)
    }
}

impl_try_from!(u8, Second);
impl_try_from!(u16, Second);
impl_try_from!(u32, Second);
impl_try_from!(u64, Second);
impl_try_from!(i8, Second);
impl_try_from!(i16, Second);
impl_try_from!(i32, Second);
impl_try_from!(i64, Second);

impl_into!(u8, Second);
impl_into!(u16, Second);
impl_into!(u32, Second);
impl_into!(u64, Second);
impl_into!(i16, Second);
impl_into!(i32, Second);
impl_into!(i64, Second);

/// An amount of seconds
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecondDuration(u64);

impl SecondDuration {
    pub fn new(hour: u64) -> Self {
        Self(hour)
    }
}

impl fmt::Display for SecondDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}M", self.0)
    }
}

impl str::FromStr for SecondDuration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.strip_suffix("M")
            .ok_or_else(|| Error::ParseError)
            .and_then(|s| s.parse().map_err(|e| Error::ParseIntError(e)))
            .map(|x| Self::new(x))
    }
}

impl Into<std::time::Duration> for SecondDuration {
    fn into(self) -> std::time::Duration {
        std::time::Duration::from_secs(self.0)
    }
}

impl_from!(u8, SecondDuration);
impl_from!(u16, SecondDuration);
impl_from!(u32, SecondDuration);
impl_from!(u64, SecondDuration);

impl_into!(u64, SecondDuration);

/// Used in combination with [`Second`] to signify subsecond fractions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Nanosecond(u32);

impl Nanosecond {
    pub fn new(nanoseconds: u64) -> Result<Self, Error> {
        if nanoseconds >= 1_000_000_000 {
            return Err(Error::RangeError);
        }
        Ok(Self(nanoseconds as u32))
    }
}

impl fmt::Display for Nanosecond {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad_integral(true, "", &self.0.to_string())
    }
}

macro_rules! impl_try_from {
    ($primitive:ty, $structtype:ident) => {
        impl TryFrom<$primitive> for $structtype {
            type Error = Error;
            #[allow(unused_comparisons)]
            fn try_from(value: $primitive) -> Result<Self, Self::Error> {
                if value < 0 {
                    return Err(Error::RangeError);
                }
                $structtype::new(value as u64)
            }
        }
    };
}

impl_try_from!(u8, Nanosecond);
impl_try_from!(u16, Nanosecond);
impl_try_from!(u32, Nanosecond);
impl_try_from!(i8, Nanosecond);
impl_try_from!(i16, Nanosecond);
impl_try_from!(i32, Nanosecond);

impl_into!(u32, Nanosecond);
impl_into!(u64, Nanosecond);
impl_into!(i32, Nanosecond);
impl_into!(i64, Nanosecond);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Timeshift {
    UTC,
    Offset {
        non_negative: bool,
        hours: Hour,
        minutes: Minute,
    },
}

impl Timeshift {
    pub fn utc() -> Self {
        Self::UTC
    }
    pub fn offset(non_negative: bool, hours: Hour, minutes: Minute) -> Self {
        Self::Offset {
            non_negative,
            hours,
            minutes,
        }
    }
    pub fn positive_offset(hours: Hour, minutes: Minute) -> Self {
        Self::Offset {
            non_negative: true,
            hours,
            minutes,
        }
    }
    pub fn negative_offset(hours: Hour, minutes: Minute) -> Self {
        Self::Offset {
            non_negative: false,
            hours,
            minutes,
        }
    }

    pub(crate) fn seconds_from_east(&self) -> i32 {
        match self {
            Timeshift::UTC => 0,
            Timeshift::Offset {
                non_negative,
                hours,
                minutes,
            } => {
                let hours: i32 = (*hours).into();
                let minutes: i32 = (*minutes).into();
                let sign = if *non_negative { 1 } else { -1 };

                sign * hours * 3600 + minutes * 60
            }
        }
    }
}

impl fmt::Display for Timeshift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UTC => write!(f, "Z"),
            Self::Offset {
                non_negative,
                hours,
                minutes,
            } if *non_negative => write!(f, "+{}:{}", hours, minutes),
            Self::Offset {
                non_negative: _,
                hours,
                minutes,
            } => write!(f, "-{}:{}", hours, minutes),
        }
    }
}

impl TryFrom<(i32, i32)> for Timeshift {
    type Error = Error;

    fn try_from((h, m): (i32, i32)) -> Result<Self, Self::Error> {
        if m < 0 {
            return Err(Error::RangeError);
        }
        if h < 0 {
            Ok(Timeshift::Offset {
                non_negative: false,
                hours: h.abs().try_into()?,
                minutes: m.try_into()?,
            })
        } else {
            Ok(Timeshift::Offset {
                non_negative: true,
                hours: h.abs().try_into()?,
                minutes: m.try_into()?,
            })
        }
    }
}
