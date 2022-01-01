use nom::{
    error::{ContextError, Error as NomError, ErrorKind, FromExternalError, ParseError},
    multi::many1,
    number::streaming::{be_u16, le_u16},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct OwnedParseError(NomError<Vec<u8>>);

impl ParseError<&'_ [u8]> for OwnedParseError {
    fn from_error_kind(input: &'_ [u8], kind: ErrorKind) -> Self {
        Self(NomError::from_error_kind(input.to_owned(), kind))
    }

    fn append(_: &'_ [u8], _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl ContextError<&'_ [u8]> for OwnedParseError {}

impl<E> FromExternalError<&'_ [u8], E> for OwnedParseError {
    fn from_external_error(input: &'_ [u8], kind: ErrorKind, _e: E) -> Self {
        Self(NomError::from_external_error(input.to_owned(), kind, _e))
    }
}
