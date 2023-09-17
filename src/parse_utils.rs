use core::fmt;
use std::{
    error::Error,
    num::ParseIntError,
    str::{self, Utf8Error},
};

use crate::components;

pub type ParseResult<'a, T> = Result<(T, &'a [u8]), ParseError<'a>>;

#[derive(Debug)]
pub enum ParseError<'a> {
    UnexpectedEof { needed: usize },
    Utf8Error,
    InvalidNumber,
    RangeError,
    NegativeZero,
    Fail(&'a [u8]),
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl<'a> Error for ParseError<'a> {}

impl<'a> From<Utf8Error> for ParseError<'a> {
    fn from(value: Utf8Error) -> Self {
        ParseError::Utf8Error
    }
}

impl<'a> From<ParseIntError> for ParseError<'a> {
    fn from(value: ParseIntError) -> Self {
        ParseError::InvalidNumber
    }
}

impl<'a> From<components::Error> for ParseError<'a> {
    fn from(value: components::Error) -> Self {
        match value {
            components::Error::RangeError => ParseError::RangeError,
            components::Error::ParseIntError(_) => ParseError::InvalidNumber,
            components::Error::ParseError => ParseError::Fail(b""),
        }
    }
}

pub(crate) fn take_n<'a>(n: usize) -> impl Fn(&'a [u8]) -> ParseResult<'a, &'a [u8]> {
    move |i: &'a [u8]| {
        if i.len() < n {
            return Err(ParseError::UnexpectedEof { needed: n });
        }
        Ok((&i[..n], &i[n..]))
    }
}

pub(crate) fn tag<'a>(tag: &'a [u8]) -> impl Fn(&'a [u8]) -> ParseResult<'a, ()> {
    move |i: &'a [u8]| {
        if i.len() < tag.len() {
            return Err(ParseError::UnexpectedEof { needed: tag.len() });
        }
        if &i[..tag.len()] != tag {
            return Err(ParseError::Fail(i));
        }
        Ok(((), &i[tag.len()..]))
    }
}

pub(crate) fn any_of<'a>(tags: &'a [&'a [u8]]) -> impl Fn(&'a [u8]) -> ParseResult<'a, usize> {
    move |i: &'a [u8]| {
        for (idx, tag) in tags.iter().enumerate() {
            if i.len() < tag.len() {
                continue;
            }
            if &i[..tag.len()] == *tag {
                return Ok((idx, &i[tag.len()..]));
            }
        }
        Err(ParseError::Fail(i))
    }
}

pub(crate) fn is_digit(n: u8) -> bool {
    b"0123456789".contains(&n)
}

pub(crate) fn take_while<'a>(
    cond: impl Fn(u8) -> bool,
) -> impl Fn(&'a [u8]) -> ParseResult<&'a [u8]> {
    move |i: &'a [u8]| {
        if i.is_empty() {
            return Err(ParseError::UnexpectedEof { needed: 1 });
        }
        let mut idx = 0;
        while cond(i[idx]) {
            if idx + 1 >= i.len() {
                idx += 1;
                break;
            }
            idx += 1;
        }
        Ok((&i[..idx], &i[idx..]))
    }
}

pub(crate) fn take_until<'a>(
    cond: impl Fn(u8) -> bool,
) -> impl Fn(&'a [u8]) -> ParseResult<&'a [u8]> {
    take_while(move |x| !cond(x))
}

pub(crate) fn parse_n_digits<'a>(n: usize, input: &'a [u8]) -> ParseResult<'a, u64> {
    let (digits, rest) = take_n(n)(input)?;
    let number: u64 = str::from_utf8(digits)?.parse()?;
    Ok((number, rest))
}
