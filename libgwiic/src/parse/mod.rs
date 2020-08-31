mod base;
mod decl;
mod frag;
mod gwii;

use crate::core::Spanned;
use nom::IResult;
use nom_locate::LocatedSpan;

pub(crate) use frag::*;
pub(crate) use gwii::parse_gwii;

pub(crate) type ParserSpan<'a> = LocatedSpan<&'a str>;
pub(crate) type ParseResult<'a, T> = ParseResultNaked<'a, Spanned<T>>;
pub(crate) type ParseResultNaked<'a, T> = IResult<ParserSpan<'a>, T>;

pub(crate) mod prelude {
    pub(crate) use crate::core::*;
    pub(crate) use crate::parse::*;

    pub(crate) use crate::parse::base::*;
    pub(crate) use nom::branch::alt;
    pub(crate) use nom::bytes::complete::{take, take_until, take_while1};
    pub(crate) use nom::combinator::all_consuming;
    pub(crate) use nom::multi::{many0, separated_list};
    pub(crate) use nom::sequence::preceded;
    pub(crate) use nom::{
        error::{ErrorKind as NomErrorKind, ParseError as NomParseError},
        Err as NomErr,
    };
}
