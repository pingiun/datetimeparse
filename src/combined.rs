use core::fmt;

#[cfg(feature = "chrono")]
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Utc};

use crate::{components::{Day, Error, Hour, Minute, Month, Nanosecond, Second, Timeshift, SimpleYear}, Year};

/// Date without time shift information
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LocalDate<Y = SimpleYear> {
    pub year: Year<Y>,
    pub month: Month,
    pub day: Day,
}

impl<Y> LocalDate<Y> {
    pub fn new(year: Year<Y>, month: Month, day: Day) -> Self {
        Self { year, month, day }
    }
}

impl fmt::Display for LocalDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}

impl<Y, M, D> TryFrom<(Y, M, D)> for LocalDate
where
    Y: TryInto<Year, Error = Error>,
    M: TryInto<Month, Error = Error>,
    D: TryInto<Day, Error = Error>,
{
    type Error = Error;
    fn try_from((year, month, day): (Y, M, D)) -> Result<Self, Self::Error> {
        Ok(Self {
            year: year.try_into()?,
            month: month.try_into()?,
            day: day.try_into()?,
        })
    }
}

#[cfg(feature = "chrono")]
impl From<LocalDate> for NaiveDate {
    fn from(val: LocalDate) -> Self {
        NaiveDate::from_ymd_opt(val.year.into(), val.month.into(), val.day.into())
            .expect("internal values are already range checked")
    }
}

/// Time without time shift information
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LocalTime {
    pub hour: Hour,
    pub minute: Minute,
    pub second: Second,
}

impl LocalTime {
    pub fn new(hour: Hour, minute: Minute, second: Second) -> Self {
        Self {
            hour,
            minute,
            second,
        }
    }
}

impl fmt::Display for LocalTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.hour, self.minute, self.second)
    }
}

impl<H, M, S> TryFrom<(H, M, S)> for LocalTime
where
    H: TryInto<Hour, Error = Error>,
    M: TryInto<Minute, Error = Error>,
    S: TryInto<Second, Error = Error>,
{
    type Error = Error;

    fn try_from((hour, minute, second): (H, M, S)) -> Result<Self, Self::Error> {
        Ok(Self {
            hour: hour.try_into()?,
            minute: minute.try_into()?,
            second: second.try_into()?,
        })
    }
}

#[cfg(feature = "chrono")]
impl From<LocalTime> for NaiveTime {
    fn from(val: LocalTime) -> Self {
        NaiveTime::from_hms_opt(val.hour.into(), val.minute.into(), val.second.into())
            .expect("internal values are already range checked")
    }
}

/// Time without time shift information, with nanosecond precision
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PreciseLocalTime {
    pub hour: Hour,
    pub minute: Minute,
    pub second: Second,
    pub nanosecond: Nanosecond,
}

impl PreciseLocalTime {
    pub fn new(hour: Hour, minute: Minute, second: Second, nanosecond: Nanosecond) -> Self {
        Self {
            hour,
            minute,
            second,
            nanosecond,
        }
    }
}

impl fmt::Display for PreciseLocalTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ns_string = format!("{:0>9}", self.nanosecond);
        let ns = if self.nanosecond == Nanosecond::new(0).unwrap() {
            "0"
        } else {
            ns_string.trim_end_matches('0')
        };
        write!(f, "{}:{}:{}.{}", self.hour, self.minute, self.second, ns)
    }
}

impl<H, M, S, N> TryFrom<(H, M, S, N)> for PreciseLocalTime
where
    H: TryInto<Hour, Error = Error>,
    M: TryInto<Minute, Error = Error>,
    S: TryInto<Second, Error = Error>,
    N: TryInto<Nanosecond, Error = Error>,
{
    type Error = Error;

    fn try_from((hour, minute, second, nanosecond): (H, M, S, N)) -> Result<Self, Self::Error> {
        Ok(Self {
            hour: hour.try_into()?,
            minute: minute.try_into()?,
            second: second.try_into()?,
            nanosecond: nanosecond.try_into()?,
        })
    }
}

#[cfg(feature = "chrono")]
impl From<PreciseLocalTime> for NaiveTime {
    fn from(val: PreciseLocalTime) -> Self {
        NaiveTime::from_hms_nano_opt(
            val.hour.into(),
            val.minute.into(),
            val.second.into(),
            val.nanosecond.into(),
        )
        .expect("internal values are already range checked")
    }
}

/// Date and time without time shift information
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LocalDateTime<Y = SimpleYear> {
    pub year: Year<Y>,
    pub month: Month,
    pub day: Day,
    pub hour: Hour,
    pub minute: Minute,
    pub second: Second,
}

impl<Y> LocalDateTime<Y> {
    pub fn new(
        year: Year<Y>,
        month: Month,
        day: Day,
        hour: Hour,
        minute: Minute,
        second: Second,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }
}

impl fmt::Display for LocalDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}T{}:{}:{}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

impl<Y, Mo, D, H, Mi, S> TryFrom<(Y, Mo, D, H, Mi, S)> for LocalDateTime
where
    Y: TryInto<Year, Error = Error>,
    Mo: TryInto<Month, Error = Error>,
    D: TryInto<Day, Error = Error>,
    H: TryInto<Hour, Error = Error>,
    Mi: TryInto<Minute, Error = Error>,
    S: TryInto<Second, Error = Error>,
{
    type Error = Error;

    fn try_from(
        (year, month, day, hour, minute, second): (Y, Mo, D, H, Mi, S),
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            year: year.try_into()?,
            month: month.try_into()?,
            day: day.try_into()?,
            hour: hour.try_into()?,
            minute: minute.try_into()?,
            second: second.try_into()?,
        })
    }
}

#[cfg(feature = "chrono")]
impl From<LocalDateTime> for NaiveDateTime {
    fn from(val: LocalDateTime) -> Self {
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(val.year.into(), val.month.into(), val.day.into())
                .expect("internal values are already range checked"),
            NaiveTime::from_hms_opt(val.hour.into(), val.minute.into(), val.second.into())
                .expect("internal values are already range checked"),
        )
    }
}

/// Date and time without time shift information, with nanosecond precision
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PreciseLocalDateTime<Y = SimpleYear> {
    pub year: Year<Y>,
    pub month: Month,
    pub day: Day,
    pub hour: Hour,
    pub minute: Minute,
    pub second: Second,
    pub nanosecond: Nanosecond,
}

impl<Y> PreciseLocalDateTime<Y> {
    pub fn new(
        year: Year<Y>,
        month: Month,
        day: Day,
        hour: Hour,
        minute: Minute,
        second: Second,
        nanosecond: Nanosecond,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            nanosecond,
        }
    }
}

impl fmt::Display for PreciseLocalDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ns_string = format!("{:0>9}", self.nanosecond);
        let ns = if self.nanosecond == Nanosecond::new(0).unwrap() {
            "0"
        } else {
            ns_string.trim_end_matches('0')
        };
        write!(
            f,
            "{}-{}-{}T{}:{}:{}.{}",
            self.year, self.month, self.day, self.hour, self.minute, self.second, ns
        )
    }
}

impl<Y, Mo, D, H, Mi, S, N> TryFrom<(Y, Mo, D, H, Mi, S, N)> for PreciseLocalDateTime
where
    Y: TryInto<Year, Error = Error>,
    Mo: TryInto<Month, Error = Error>,
    D: TryInto<Day, Error = Error>,
    H: TryInto<Hour, Error = Error>,
    Mi: TryInto<Minute, Error = Error>,
    S: TryInto<Second, Error = Error>,
    N: TryInto<Nanosecond, Error = Error>,
{
    type Error = Error;

    fn try_from(
        (year, month, day, hour, minute, second, nanosecond): (Y, Mo, D, H, Mi, S, N),
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            year: year.try_into()?,
            month: month.try_into()?,
            day: day.try_into()?,
            hour: hour.try_into()?,
            minute: minute.try_into()?,
            second: second.try_into()?,
            nanosecond: nanosecond.try_into()?,
        })
    }
}

#[cfg(feature = "chrono")]
impl From<PreciseLocalDateTime> for NaiveDateTime {
    fn from(val: PreciseLocalDateTime) -> Self {
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(val.year.into(), val.month.into(), val.day.into())
                .expect("internal values are already range checked"),
            NaiveTime::from_hms_nano_opt(
                val.hour.into(),
                val.minute.into(),
                val.second.into(),
                val.nanosecond.into(),
            )
            .expect("internal values are already range checked"),
        )
    }
}

/// Date and time with time shift information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftedDateTime<Y = SimpleYear> {
    pub year: Year<Y>,
    pub month: Month,
    pub day: Day,
    pub hour: Hour,
    pub minute: Minute,
    pub second: Second,
    pub timeshift: Timeshift,
}

impl<Y> ShiftedDateTime<Y> {
    pub fn new(
        year: Year<Y>,
        month: Month,
        day: Day,
        hour: Hour,
        minute: Minute,
        second: Second,
        timeshift: Timeshift,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            timeshift,
        }
    }
}

impl fmt::Display for ShiftedDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}T{}:{}:{}{}",
            self.year, self.month, self.day, self.hour, self.minute, self.second, self.timeshift
        )
    }
}

impl<Y, Mo, D, H, Mi, S, T> TryFrom<(Y, Mo, D, H, Mi, S, T)> for ShiftedDateTime
where
    Y: TryInto<Year, Error = Error>,
    Mo: TryInto<Month, Error = Error>,
    D: TryInto<Day, Error = Error>,
    H: TryInto<Hour, Error = Error>,
    Mi: TryInto<Minute, Error = Error>,
    S: TryInto<Second, Error = Error>,
    T: TryInto<Timeshift, Error = Error>,
{
    type Error = Error;

    fn try_from(
        (year, month, day, hour, minute, second, timeshift): (Y, Mo, D, H, Mi, S, T),
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            year: year.try_into()?,
            month: month.try_into()?,
            day: day.try_into()?,
            hour: hour.try_into()?,
            minute: minute.try_into()?,
            second: second.try_into()?,
            timeshift: timeshift.try_into()?,
        })
    }
}

#[cfg(feature = "chrono")]
impl From<ShiftedDateTime> for DateTime<FixedOffset> {
    fn from(val: ShiftedDateTime) -> Self {
        DateTime::from_local(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(val.year.into(), val.month.into(), val.day.into())
                    .expect("internal values are already range checked"),
                NaiveTime::from_hms_opt(val.hour.into(), val.minute.into(), val.second.into())
                    .expect("internal values are already range checked"),
            ),
            FixedOffset::east_opt(val.timeshift.seconds_from_east())
                .expect("internal values are already range checked"),
        )
    }
}

#[cfg(feature = "chrono")]
impl TryInto<DateTime<Utc>> for ShiftedDateTime {
    type Error = ();

    fn try_into(self) -> Result<DateTime<Utc>, Self::Error> {
        match self.timeshift {
            Timeshift::Utc => Ok(DateTime::<Utc>::from_local(
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(self.year.into(), self.month.into(), self.day.into())
                        .expect("internal values are already range checked"),
                    NaiveTime::from_hms_opt(
                        self.hour.into(),
                        self.minute.into(),
                        self.second.into(),
                    )
                    .expect("internal values are already range checked"),
                ),
                Utc,
            )),
            Timeshift::Offset {
                non_negative: _,
                hours: _,
                minutes: _,
            } => Err(()),
        }
    }
}

/// Date and with time shift information, with nanosecond precision
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreciseShiftedDateTime<Y = SimpleYear> {
    pub year: Year<Y>,
    pub month: Month,
    pub day: Day,
    pub hour: Hour,
    pub minute: Minute,
    pub second: Second,
    pub nanosecond: Nanosecond,
    pub timeshift: Timeshift,
}

impl<Y> PreciseShiftedDateTime<Y> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        year: Year<Y>,
        month: Month,
        day: Day,
        hour: Hour,
        minute: Minute,
        second: Second,
        nanosecond: Nanosecond,
        timeshift: Timeshift,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            nanosecond,
            timeshift,
        }
    }
}

impl fmt::Display for PreciseShiftedDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ns_string = format!("{:0>9}", self.nanosecond);
        let ns = if self.nanosecond == Nanosecond::new(0).unwrap() {
            "0"
        } else {
            ns_string.trim_end_matches('0')
        };
        write!(
            f,
            "{}-{}-{}T{}:{}:{}.{}{}",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
            ns,
            self.timeshift
        )
    }
}

impl<Y, Mo, D, H, Mi, S, N, T> TryFrom<(Y, Mo, D, H, Mi, S, N, T)> for PreciseShiftedDateTime
where
    Y: TryInto<Year, Error = Error>,
    Mo: TryInto<Month, Error = Error>,
    D: TryInto<Day, Error = Error>,
    H: TryInto<Hour, Error = Error>,
    Mi: TryInto<Minute, Error = Error>,
    S: TryInto<Second, Error = Error>,
    N: TryInto<Nanosecond, Error = Error>,
    T: TryInto<Timeshift, Error = Error>,
{
    type Error = Error;

    fn try_from(
        (year, month, day, hour, minute, second, nanosecond, timeshift): (Y, Mo, D, H, Mi, S, N, T),
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            year: year.try_into()?,
            month: month.try_into()?,
            day: day.try_into()?,
            hour: hour.try_into()?,
            minute: minute.try_into()?,
            second: second.try_into()?,
            nanosecond: nanosecond.try_into()?,
            timeshift: timeshift.try_into()?,
        })
    }
}

#[cfg(feature = "chrono")]
impl From<PreciseShiftedDateTime> for DateTime<FixedOffset> {
    fn from(val: PreciseShiftedDateTime) -> Self {
        DateTime::from_local(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(val.year.into(), val.month.into(), val.day.into())
                    .expect("internal values are already range checked"),
                NaiveTime::from_hms_nano_opt(
                    val.hour.into(),
                    val.minute.into(),
                    val.second.into(),
                    val.nanosecond.into(),
                )
                .expect("internal values are already range checked"),
            ),
            FixedOffset::east_opt(val.timeshift.seconds_from_east())
                .expect("internal values are already range checked"),
        )
    }
}

#[cfg(feature = "chrono")]
impl TryInto<DateTime<Utc>> for PreciseShiftedDateTime {
    type Error = ();

    fn try_into(self) -> Result<DateTime<Utc>, Self::Error> {
        match self.timeshift {
            Timeshift::Utc => Ok(DateTime::<Utc>::from_local(
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(self.year.into(), self.month.into(), self.day.into())
                        .expect("internal values are already range checked"),
                    NaiveTime::from_hms_nano_opt(
                        self.hour.into(),
                        self.minute.into(),
                        self.second.into(),
                        self.nanosecond.into(),
                    )
                    .expect("internal values are already range checked"),
                ),
                Utc,
            )),
            Timeshift::Offset {
                non_negative: _,
                hours: _,
                minutes: _,
            } => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LocalDate, PreciseLocalTime, PreciseShiftedDateTime};

    #[test]
    fn test_try_from_tuple() {
        let cd: LocalDate = LocalDate::try_from((2022, 1, 2)).unwrap();
        assert_eq!(format!("{}", cd), "2022-01-02")
    }

    #[test]
    fn test_precise_time() {
        let pt: PreciseLocalTime = PreciseLocalTime::try_from((20, 12, 0, 0)).unwrap();
        assert_eq!(format!("{}", pt), "20:12:00.0");

        let pt: PreciseLocalTime = PreciseLocalTime::try_from((20, 12, 0, 123_400_000)).unwrap();
        assert_eq!(format!("{}", pt), "20:12:00.1234");
    }

    #[test]
    fn test_format_full_datetime() {
        let dt = PreciseShiftedDateTime::try_from((2023, 4, 9, 21, 22, 2, 123_400_000, (12, 2)))
            .unwrap();
        assert_eq!(format!("{}", dt), "2023-04-09T21:22:02.1234+12:02");
        let dt = PreciseShiftedDateTime::try_from((2023, 4, 9, 21, 22, 2, 123_400_000, (-12, 2)))
            .unwrap();
        assert_eq!(format!("{}", dt), "2023-04-09T21:22:02.1234-12:02")
    }
}
