use core::{fmt, num};

#[derive(Debug)]
pub enum Error {
    Range,
    ParseInt(num::ParseIntError),
    Parse,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimpleYear;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtendedYear<const N: usize>;

pub trait YearDigits {
    fn digits() -> usize;
    fn from_digits(digits: i32) -> Result<Year<Self>, Error> where Self: Sized;
}

impl YearDigits for SimpleYear {
    fn digits() -> usize {
        4
    }
    fn from_digits(digits: i32) -> Result<Year<Self>, Error> {
        Year::new(digits)
    }
}

impl<const N: usize> YearDigits for ExtendedYear<N> {
    fn digits() -> usize {
        N
    }
    fn from_digits(digits: i32) -> Result<Year<Self>, Error> {
        Year::new_extended(digits)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Year<Y = SimpleYear>(i32, Y);

impl<const N: usize> Year<ExtendedYear<N>> {
    pub fn new_extended(year: i32) -> Result<Self, Error> {
        if year == 0 {
            return Ok(Self(year, ExtendedYear));
        }
        let year = match year.checked_abs().and_then(|x| x.checked_ilog10()) {
            Some(num) if (num as usize) < N => year,
            Some(_) => return Err(Error::Range),
            None => return Err(Error::Range),
        };
        Ok(Self(year, ExtendedYear))
    }
}

impl Year<SimpleYear> {
    pub fn new(year: i32) -> Result<Self, Error> {
        if year < 0 || year > 9999 {
            return Err(Error::Range);
        }

        Ok(Self(year, SimpleYear))
    }
}

#[cfg(test)]
mod year_test {
    use super::{ExtendedYear, Year};

    #[test]
    fn test_year_4_digits() {
        assert!(Year::new(0).is_ok());
        assert!(Year::new(1).is_ok());
        assert!(Year::new(9999).is_ok());
        assert!(Year::new(10000).is_err());
        assert_eq!(format!("{}", Year::new(1).unwrap()), "0001");
    }

    #[test]
    fn test_negative_years() {
        assert!(Year::new(-1).is_err());
        assert!(Year::<ExtendedYear<6>>::new_extended(-1).is_ok());
        assert_eq!(
            format!("{}", Year::<ExtendedYear<6>>::new_extended(-1).unwrap()),
            "-000001"
        );
    }

    #[test]
    fn test_big_years() {
        assert!(Year::<ExtendedYear<6>>::new_extended(100000).is_ok());
        assert_eq!(
            format!("{}", Year::<ExtendedYear<6>>::new_extended(1).unwrap()),
            "+000001"
        );
    }
}

impl fmt::Display for Year<SimpleYear> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>width$}", self.0, width = 4)
    }
}

impl<const N: usize> fmt::Display for Year<ExtendedYear<N>> {
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
                    return Err(Error::Range);
                }
                $structtype::new(value as i32)
            }
        }
    };
}

impl_try_from!(u8, Year);
impl_try_from!(u16, Year);
impl_try_from!(u32, Year);
impl_try_from!(u64, Year);
impl_try_from!(i8, Year);
impl_try_from!(i16, Year);
impl_try_from!(i32, Year);
impl_try_from!(i64, Year);

macro_rules! impl_into {
    ($primitive:ty, $structtype:ident) => {
        impl From<$structtype> for $primitive {
            fn from(value: $structtype) -> $primitive {
                value.0 as $primitive
            }
        }
    };
}

impl_into!(u8, Year);
impl_into!(u16, Year);
impl_into!(u32, Year);
impl_into!(u64, Year);
impl_into!(i16, Year);
impl_into!(i32, Year);
impl_into!(i64, Year);

/// Month of the year (1-12)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Month(u8);

impl Month {
    pub fn new(month: u64) -> Result<Self, Error> {
        if month == 0 {
            return Err(Error::Range);
        }
        if month > 12 {
            return Err(Error::Range);
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
                    return Err(Error::Range);
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

/// Week of the year (1-53)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Week(u8);

impl Week {
    pub fn new(week: u64) -> Result<Self, Error> {
        if week > 53 {
            return Err(Error::Range);
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

/// Day of the month (1-31)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Day(u8);

impl Day {
    pub fn new(day: u64) -> Result<Self, Error> {
        if day == 0 {
            return Err(Error::Range);
        }
        if day > 31 {
            return Err(Error::Range);
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
                    return Err(Error::Range);
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

/// Hours (0-60)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hour(u8);

impl Hour {
    pub fn new(hour: u64) -> Result<Hour, Error> {
        if hour > 24 {
            return Err(Error::Range);
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
                    return Err(Error::Range);
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

/// Minutes (0-60)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Minute(u8);

impl Minute {
    pub fn new(minute: u64) -> Result<Minute, Error> {
        if minute > 60 {
            return Err(Error::Range);
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

/// Seconds (0-61)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Second(u8);

impl Second {
    pub fn new(second: u64) -> Result<Second, Error> {
        if second > 61 {
            return Err(Error::Range);
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

/// Used in combination with [`Second`] to signify subsecond fractions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Nanosecond(u32);

impl Nanosecond {
    pub fn new(nanoseconds: u64) -> Result<Self, Error> {
        if nanoseconds >= 1_000_000_000 {
            return Err(Error::Range);
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
                    return Err(Error::Range);
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
    Utc,
    Offset {
        non_negative: bool,
        hours: Hour,
        minutes: Minute,
    },
}

impl Timeshift {
    pub fn utc() -> Self {
        Self::Utc
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
            Timeshift::Utc => 0,
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
            Self::Utc => write!(f, "Z"),
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
            return Err(Error::Range);
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
