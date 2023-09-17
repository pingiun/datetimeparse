//! Durations from ISO 8601, chapter 4.3

use core::{fmt, str};

use crate::components::Error;

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
        s.strip_suffix('Y')
            .ok_or(Error::Parse)
            .and_then(|s| s.parse().map_err(Error::ParseInt))
            .map(Self::new)
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

macro_rules! impl_into {
    ($primitive:ty, $structtype:ident) => {
        impl From<$structtype> for $primitive {
            fn from(value: $structtype) -> $primitive {
                value.0 as $primitive
            }
        }
    };
}

impl_into!(u64, YearDuration);

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
        s.strip_suffix('M')
            .ok_or(Error::Parse)
            .and_then(|s| s.parse().map_err(Error::ParseInt))
            .map(Self::new)
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
        s.strip_suffix('W')
            .ok_or(Error::Parse)
            .and_then(|s| s.parse().map_err(Error::ParseInt))
            .map(Self::new)
    }
}

impl From<WeekDuration> for std::time::Duration {
    fn from(val: WeekDuration) -> Self {
        std::time::Duration::from_secs(val.0 * 60 * 60 * 24 * 7)
    }
}

impl_from!(u8, WeekDuration);
impl_from!(u16, WeekDuration);
impl_from!(u32, WeekDuration);
impl_from!(u64, WeekDuration);

impl_into!(u64, WeekDuration);

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
        s.strip_suffix('D')
            .ok_or(Error::Parse)
            .and_then(|s| s.parse().map_err(Error::ParseInt))
            .map(Self::new)
    }
}

impl From<DayDuration> for std::time::Duration {
    fn from(val: DayDuration) -> Self {
        std::time::Duration::from_secs(val.0 * 60 * 60 * 24)
    }
}

impl_from!(u8, DayDuration);
impl_from!(u16, DayDuration);
impl_from!(u32, DayDuration);
impl_from!(u64, DayDuration);

impl_into!(u64, DayDuration);

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
        s.strip_suffix('H')
            .ok_or(Error::Parse)
            .and_then(|s| s.parse().map_err(Error::ParseInt))
            .map(Self::new)
    }
}

impl From<HourDuration> for std::time::Duration {
    fn from(val: HourDuration) -> Self {
        std::time::Duration::from_secs(val.0 * 60 * 60)
    }
}

impl_from!(u8, HourDuration);
impl_from!(u16, HourDuration);
impl_from!(u32, HourDuration);
impl_from!(u64, HourDuration);

impl_into!(u64, HourDuration);

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
        s.strip_suffix('M')
            .ok_or(Error::Parse)
            .and_then(|s| s.parse().map_err(Error::ParseInt))
            .map(Self::new)
    }
}

impl From<MinuteDuration> for std::time::Duration {
    fn from(val: MinuteDuration) -> Self {
        std::time::Duration::from_secs(val.0 * 60)
    }
}

impl_from!(u8, MinuteDuration);
impl_from!(u16, MinuteDuration);
impl_from!(u32, MinuteDuration);
impl_from!(u64, MinuteDuration);

impl_into!(u64, MinuteDuration);

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
        s.strip_suffix('M')
            .ok_or(Error::Parse)
            .and_then(|s| s.parse().map_err(Error::ParseInt))
            .map(Self::new)
    }
}

impl From<SecondDuration> for std::time::Duration {
    fn from(val: SecondDuration) -> Self {
        std::time::Duration::from_secs(val.0)
    }
}

impl_from!(u8, SecondDuration);
impl_from!(u16, SecondDuration);
impl_from!(u32, SecondDuration);
impl_from!(u64, SecondDuration);

impl_into!(u64, SecondDuration);
