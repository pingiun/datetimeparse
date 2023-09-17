use std::{collections::VecDeque, str};

use crate::{
    combined::{
        LocalDate, LocalDateTime, LocalTime, PreciseLocalDateTime, PreciseLocalTime,
        PreciseShiftedDateTime, ShiftedDateTime,
    },
    components::{Day, Hour, Minute, Month, Nanosecond, Second, StandardYear, Timeshift, Year},
    parse_utils::{is_digit, parse_n_digits, tag, take_while, ParseError, any_of},
};

#[derive(Debug)]
pub enum Element<Y = Year> {
    Year(Y),
    Month(Month),
    Day(Day),
    Hour(Hour),
    Minute(Minute),
    Second(Second),
    Nanosecond(Nanosecond),
    Timeshift(Timeshift),
}

#[derive(Debug)]
pub enum ElementTag {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Nanosecond,
    Timeshift,
}

pub struct Parser {
    elements: VecDeque<Element>,
    context: ParseContext,
}

#[derive(Debug)]
pub enum BuildError {
    NotEnoughElements,
    Unexpected { got: Element, expected: ElementTag },
}

pub struct ParseContext {
    space_as_date_time_separator: bool,
    empty_date_separator: bool,
    empty_time_separator: bool,
    negative_zero: bool,
}

impl ParseContext {
    pub fn new_rfc3339() -> Self {
        Self {
            space_as_date_time_separator: true,
            empty_date_separator: false,
            empty_time_separator: false,
            negative_zero: true,
        }
    }

    pub fn new_strict_rfc3339() -> Self {
        Self {
            space_as_date_time_separator: false,
            empty_date_separator: false,
            empty_time_separator: false,
            negative_zero: true,
        }
    }

    pub fn new_iso8601() -> Self {
        Self {
            space_as_date_time_separator: false,
            empty_date_separator: true,
            empty_time_separator: true,
            negative_zero: false,
        }
    }

    pub fn into_parser(self) -> Parser {
        Parser {
            elements: VecDeque::new(),
            context: self,
        }
    }

    fn allows_empty_date_separators(&self) -> bool {
        self.empty_date_separator
    }

    fn allows_empty_time_separators(&self) -> bool {
        self.empty_time_separator
    }

    fn allows_space_as_date_time_separator(&self) -> bool {
        self.space_as_date_time_separator
    }

    fn allows_negative_zero(&self) -> bool {
        self.negative_zero
    }
}

impl Default for ParseContext {
    fn default() -> Self {
        Self::new_rfc3339()
    }
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            elements: VecDeque::new(),
            context: ParseContext::default(),
        }
    }
}

impl Parser {
    pub fn parse_year<'a>(&mut self, data: &'a [u8]) -> Result<&'a [u8], ParseError<'a>> {
        let (year, rest) = parse_n_digits(4, data)?;
        let year = year.try_into().map_err(|_| ParseError::RangeError)?;
        self.elements
            .push_back(Element::Year(StandardYear::new(year)?));
        Ok(rest)
    }

    pub fn parse_month<'a>(&mut self, data: &'a [u8]) -> Result<&'a [u8], ParseError<'a>> {
        let (month, rest) = parse_n_digits(2, data)?;
        self.elements.push_back(Element::Month(Month::new(month)?));
        Ok(rest)
    }

    pub fn parse_day<'a>(&mut self, data: &'a [u8]) -> Result<&'a [u8], ParseError<'a>> {
        let (day, rest) = parse_n_digits(2, data)?;
        self.elements.push_back(Element::Day(Day::new(day)?));
        Ok(rest)
    }

    pub fn parse_date_separator<'a>(&mut self, data: &'a [u8]) -> Result<&'a [u8], ParseError<'a>> {
        let rest = match tag(b"-")(data) {
            Ok((_, rest)) => rest,
            Err(ParseError::Fail(x)) => {
                if self.context.allows_empty_date_separators() {
                    data
                } else {
                    return Err(ParseError::Fail(x));
                }
            }
            Err(e) => return Err(e),
        };
        Ok(rest)
    }

    pub fn parse_date<'a>(&mut self, data: &'a [u8]) -> Result<&'a [u8], ParseError<'a>> {
        let rest = self.parse_year(data)?;
        let rest = self.parse_date_separator(rest)?;
        let rest = self.parse_month(rest)?;
        let rest = self.parse_date_separator(rest)?;
        let rest = self.parse_day(rest)?;
        Ok(rest)
    }

    pub fn parse_hour<'a>(&mut self, data: &'a [u8]) -> Result<&'a [u8], ParseError<'a>> {
        let (hour, rest) = parse_n_digits(2, data)?;
        self.elements.push_back(Element::Hour(Hour::new(hour)?));
        Ok(rest)
    }

    pub fn parse_minute<'a>(&mut self, data: &'a [u8]) -> Result<&'a [u8], ParseError<'a>> {
        let (minute, rest) = parse_n_digits(2, data)?;
        self.elements
            .push_back(Element::Minute(Minute::new(minute)?));
        Ok(rest)
    }

    pub fn parse_second<'a>(&mut self, data: &'a [u8]) -> Result<&'a [u8], ParseError<'a>> {
        let (second, rest) = parse_n_digits(2, data)?;
        self.elements
            .push_back(Element::Second(Second::new(second)?));
        Ok(rest)
    }

    pub fn parse_time_separator<'a>(&mut self, data: &'a [u8]) -> Result<&'a [u8], ParseError<'a>> {
        let rest = match tag(b":")(data) {
            Ok((_, rest)) => rest,
            Err(ParseError::Fail(x)) => {
                if self.context.allows_empty_time_separators() {
                    data
                } else {
                    return Err(ParseError::Fail(x));
                }
            }
            Err(e) => return Err(e),
        };
        Ok(rest)
    }

    pub fn parse_time<'a>(&mut self, data: &'a [u8]) -> Result<&'a [u8], ParseError<'a>> {
        let rest = self.parse_hour(data)?;
        let rest = self.parse_time_separator(rest)?;
        let rest = self.parse_minute(rest)?;
        let rest = self.parse_time_separator(rest)?;
        let rest = self.parse_second(rest)?;
        Ok(rest)
    }

    pub fn parse_date_time_separator<'a>(
        &mut self,
        data: &'a [u8],
    ) -> Result<&'a [u8], ParseError<'a>> {
        let rest = match any_of(&[b"T", b"t"])(data) {
            Ok((_, rest)) => rest,
            Err(ParseError::Fail(x)) => {
                if self.context.allows_space_as_date_time_separator() {
                    let (_, rest) = tag(b" ")(data)?;
                    rest
                } else {
                    return Err(ParseError::Fail(x));
                }
            }
            Err(e) => return Err(e),
        };
        Ok(rest)
    }

    pub fn parse_fractional_separator<'a>(
        &mut self,
        data: &'a [u8],
    ) -> Result<&'a [u8], ParseError<'a>> {
        let (_, rest) = tag(b".")(data)?;
        Ok(rest)
    }

    pub fn parse_fractional_seconds<'a>(
        &mut self,
        data: &'a [u8],
    ) -> Result<&'a [u8], ParseError<'a>> {
        let (digits, rest) = take_while(is_digit)(data)?;
        if digits.len() > 9 {
            return Err(ParseError::RangeError);
        }
        let number: u64 = str::from_utf8(digits)?.parse()?;
        let factor = 10u64.pow((9 - digits.len()) as u32);
        self.elements
            .push_back(Element::Nanosecond(Nanosecond::new(number * factor)?));
        Ok(rest)
    }

    pub fn parse_timezone_offset<'a>(
        &mut self,
        data: &'a [u8],
    ) -> Result<&'a [u8], ParseError<'a>> {
        let res = any_of(&[b"Z", b"z"])(data);
        if let Ok((_, rest)) = res {
            self.elements
                .push_back(Element::Timeshift(Timeshift::utc()));
            return Ok(rest);
        }
        if data.len() < 1 {
            return Err(ParseError::UnexpectedEof { needed: 1 });
        }
        let (non_negative, rest) = match data[0] {
            b'-' => (false, &data[1..]),
            b'+' => (true, &data[1..]),
            _ => return Err(ParseError::Fail(data)),
        };
        let (hours, rest) = parse_n_digits(2, rest)?;
        let rest = self.parse_time_separator(rest)?;
        let (minutes, rest) = parse_n_digits(2, rest)?;
        if !non_negative && hours == 0 && minutes == 0 && !self.context.allows_negative_zero() {
            return Err(ParseError::NegativeZero);
        }
        let hours = Hour::new(hours)?;
        let minutes = Minute::new(minutes)?;

        self.elements
            .push_back(Element::Timeshift(Timeshift::offset(
                non_negative,
                hours,
                minutes,
            )));

        Ok(rest)
    }

    pub fn parse_local_date_time<'a>(
        &mut self,
        data: &'a [u8],
    ) -> Result<&'a [u8], ParseError<'a>> {
        let rest = self.parse_date(data)?;
        let rest = self.parse_date_time_separator(rest)?;
        let rest = self.parse_time(rest)?;
        Ok(rest)
    }

    pub fn parse_precise_local_date_time<'a>(
        &mut self,
        data: &'a [u8],
    ) -> Result<&'a [u8], ParseError<'a>> {
        let rest = self.parse_local_date_time(data)?;
        let rest = match self.parse_fractional_separator(rest) {
            Ok(rest) => self.parse_fractional_seconds(rest)?,
            Err(ParseError::Fail(_)) => {self.elements.push_back(Element::Nanosecond(Nanosecond::new(0)?)); return Ok(rest)},
            Err(e) => return Err(e),
        };
        Ok(rest)
    }

    pub fn parse_shifted_date_time<'a>(
        &mut self,
        data: &'a [u8],
    ) -> Result<&'a [u8], ParseError<'a>> {
        let rest = self.parse_local_date_time(data)?;
        let rest = self.parse_timezone_offset(rest)?;
        Ok(rest)
    }

    pub fn parse_precise_shifted_date_time<'a>(
        &mut self,
        data: &'a [u8],
    ) -> Result<&'a [u8], ParseError<'a>> {
        let rest = self.parse_precise_local_date_time(data)?;
        let rest = self.parse_timezone_offset(rest)?;
        Ok(rest)
    }

    pub fn build_date(mut self) -> Result<LocalDate, BuildError> {
        let year = match self.elements.pop_front() {
            Some(Element::Year(year)) => year,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Year,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let month = match self.elements.pop_front() {
            Some(Element::Month(month)) => month,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Month,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let day = match self.elements.pop_front() {
            Some(Element::Day(day)) => day,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Day,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        Ok(LocalDate { year, month, day })
    }

    pub fn build_time(mut self) -> Result<LocalTime, BuildError> {
        let hour = match self.elements.pop_front() {
            Some(Element::Hour(hour)) => hour,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Hour,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let minute = match self.elements.pop_front() {
            Some(Element::Minute(minute)) => minute,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Minute,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let second = match self.elements.pop_front() {
            Some(Element::Second(second)) => second,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Second,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };

        Ok(LocalTime {
            hour,
            minute,
            second,
        })
    }

    pub fn build_precise_local_time(mut self) -> Result<PreciseLocalTime, BuildError> {
        let hour = match self.elements.pop_front() {
            Some(Element::Hour(hour)) => hour,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Hour,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let minute = match self.elements.pop_front() {
            Some(Element::Minute(minute)) => minute,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Minute,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let second = match self.elements.pop_front() {
            Some(Element::Second(second)) => second,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Second,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let nanosecond = match self.elements.pop_front() {
            Some(Element::Nanosecond(nanosecond)) => nanosecond,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Nanosecond,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };

        Ok(PreciseLocalTime {
            hour,
            minute,
            second,
            nanosecond,
        })
    }

    pub fn build_local_date_time(mut self) -> Result<LocalDateTime, BuildError> {
        let year = match self.elements.pop_front() {
            Some(Element::Year(year)) => year,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Year,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let month = match self.elements.pop_front() {
            Some(Element::Month(month)) => month,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Month,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let day = match self.elements.pop_front() {
            Some(Element::Day(day)) => day,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Day,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let hour = match self.elements.pop_front() {
            Some(Element::Hour(hour)) => hour,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Hour,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let minute = match self.elements.pop_front() {
            Some(Element::Minute(minute)) => minute,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Minute,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let second = match self.elements.pop_front() {
            Some(Element::Second(second)) => second,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Second,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        Ok(LocalDateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }

    pub fn build_shifted_date_time(mut self) -> Result<ShiftedDateTime, BuildError> {
        let year = match self.elements.pop_front() {
            Some(Element::Year(year)) => year,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Year,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let month = match self.elements.pop_front() {
            Some(Element::Month(month)) => month,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Month,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let day = match self.elements.pop_front() {
            Some(Element::Day(day)) => day,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Day,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let hour = match self.elements.pop_front() {
            Some(Element::Hour(hour)) => hour,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Hour,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let minute = match self.elements.pop_front() {
            Some(Element::Minute(minute)) => minute,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Minute,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let second = match self.elements.pop_front() {
            Some(Element::Second(second)) => second,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Second,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let timeshift = match self.elements.pop_front() {
            Some(Element::Timeshift(timeshift)) => timeshift,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Timeshift,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        Ok(ShiftedDateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
            timeshift,
        })
    }

    pub fn build_precise_local_date_time(mut self) -> Result<PreciseLocalDateTime, BuildError> {
        let year = match self.elements.pop_front() {
            Some(Element::Year(year)) => year,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Year,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let month = match self.elements.pop_front() {
            Some(Element::Month(month)) => month,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Month,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let day = match self.elements.pop_front() {
            Some(Element::Day(day)) => day,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Day,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let hour = match self.elements.pop_front() {
            Some(Element::Hour(hour)) => hour,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Hour,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let minute = match self.elements.pop_front() {
            Some(Element::Minute(minute)) => minute,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Minute,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let second = match self.elements.pop_front() {
            Some(Element::Second(second)) => second,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Second,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let nanosecond = match self.elements.pop_front() {
            Some(Element::Nanosecond(nanosecond)) => nanosecond,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Nanosecond,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        Ok(PreciseLocalDateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
            nanosecond,
        })
    }

    pub fn build_precise_shifted_date_time(mut self) -> Result<PreciseShiftedDateTime, BuildError> {
        let year = match self.elements.pop_front() {
            Some(Element::Year(year)) => year,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Year,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let month = match self.elements.pop_front() {
            Some(Element::Month(month)) => month,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Month,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let day = match self.elements.pop_front() {
            Some(Element::Day(day)) => day,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Day,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let hour = match self.elements.pop_front() {
            Some(Element::Hour(hour)) => hour,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Hour,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let minute = match self.elements.pop_front() {
            Some(Element::Minute(minute)) => minute,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Minute,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let second = match self.elements.pop_front() {
            Some(Element::Second(second)) => second,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Second,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let nanosecond = match self.elements.pop_front() {
            Some(Element::Nanosecond(nanosecond)) => nanosecond,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Nanosecond,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        let timeshift = match self.elements.pop_front() {
            Some(Element::Timeshift(timeshift)) => timeshift,
            Some(e) => {
                return Err(BuildError::Unexpected {
                    got: e,
                    expected: ElementTag::Timeshift,
                })
            }
            None => return Err(BuildError::NotEnoughElements),
        };
        Ok(PreciseShiftedDateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
            nanosecond,
            timeshift,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    pub fn test_parse_time() {
        let mut parser = Parser::new();
        let rest = b"20:10:21";
        let rest = parser.parse_time(rest).unwrap();
        assert_eq!(rest, b"");
        let time = parser.build_time().unwrap();
        assert_eq!(time, (20, 10, 21).try_into().unwrap())
    }
}
