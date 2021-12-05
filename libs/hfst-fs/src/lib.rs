use std::collections::HashMap;
use std::fs;
use std::io;
use std::borrow::Cow;

use byteorder::{BigEndian, LittleEndian};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HfstHeaderError {
    #[error("Invalid header (expected: {expected:?}, got: {found:?})")]
    InvalidHeader {
        expected: String,
        found: String
    },
    #[error("Malformed header: {0}")]
    MalformedHeader(String),
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
}

pub trait ReadSeek: io::Read + io::Seek {}

impl<T> ReadSeek for T
    where T: io::Read + io::Seek
{}

pub fn parse_hfst3_header(
    // instream: &mut fs::File,
    instream: &mut dyn ReadSeek,
) -> Result<HashMap<String, String>, HfstHeaderError> {
    use io::{Read, Seek};
    use byteorder::ReadBytesExt;

    const BUFFER_SIZE: usize = 256;

    log::trace!("Called parse_hfst3_header");
    let start_position = instream.stream_position()?;

    let header1 = "HFST";
    let mut buffer = [0u8; BUFFER_SIZE];
    let _ = instream.take(8).read(&mut buffer)?;
    log::trace!("buffer = {:?}", &buffer[0..10]);
    log::trace!("buffer = {:?}", String::from_utf8_lossy(&buffer[0..10]));
    
    let remaining_header_len = match static_header(&buffer[0..8]) {
        Ok((_, header_size)) => header_size.size,
        _ => {
            log::warn!("Found unknown header: '{:?}'", &buffer[0..header1.len()]);
            instream.seek(io::SeekFrom::Start(start_position))?;
            return Err(HfstHeaderError::InvalidHeader {
                expected: header1.to_owned(),
                found: String::from_utf8_lossy(&buffer[0..header1.len()]).into_owned(),
            });
        }
    };
    log::debug!("Header size: {}", remaining_header_len);
    let mut buffer = vec![0u8; remaining_header_len as usize];
    let _ = instream.take(remaining_header_len as u64).read(&mut buffer)?;
    let properties = match hash(&buffer) {
        Ok((remaining, properties)) => {
            log::debug!("remaining: {:?}", remaining);
            properties
        },
        Err(error) => {
            log::error!("Header error:\n{:?}", error);
            println!("Header error:\n{:?}", error);
            panic!("Error: {:?}", error);
        }
    };
    let properties = properties.into_iter().collect();
    Ok(properties)
}
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    combinator::{cut, map},
    error::context,
    multi::many1,
    number::complete::be_u16,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
pub struct Hfst;

pub fn header(i: &[u8]) -> IResult<&[u8], Hfst> {
    map(terminated(
        tag(b"HFST"),
        tag(b"\0")
    ), |_| Hfst {})(i)
}

#[derive(Debug, Eq, PartialEq)]
pub struct HeaderSize {
    size: u16,
}

pub fn header_size(i: &[u8]) -> IResult<&[u8], HeaderSize> {
    map(terminated(
        be_u16,
        tag(b"\0")
    ), |size| HeaderSize { size })(i)
}

pub fn static_header(i: &[u8]) -> IResult<&[u8], HeaderSize> {
    preceded(
        header,
        header_size
    )(i)
}

pub fn hash(i: &[u8]) -> IResult<&[u8], Vec<(String, String)>> {
    context(
        "map",
        cut(
            map(
                many1(key_value),
                |tuple_vec| {
                    tuple_vec.into_iter().map(|(k, v)| (String::from(k), String::from(v))).collect()
                }
        ))
    )(i)
}

#[derive(Debug, Eq, PartialEq)]
pub struct Nil;

pub fn nil(i: &[u8]) -> IResult<&[u8], Nil> {
    map(tag(b"\0"), |_| Nil)(i)
}

pub fn parse_zb_str<'a>(i: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    take_until1(b"\0" as &[u8])(i)
}

pub fn zb_string(i: &[u8]) -> IResult<&[u8], &[u8]> {
    terminated(
        parse_zb_str,
        tag(b"\0")
    )(i)
}

pub fn nullended_string(i: &[u8]) -> IResult<&[u8], Cow<str>> {
    map(zb_string, |s| String::from_utf8_lossy(s).to_owned())(i)
}

pub fn nullended_string0(i: &[u8]) -> IResult<&[u8], Cow<str>> {
    alt((
        nullended_string,
        map(nil, |_| Cow::from(""))
    ))(i)
}

pub fn key_value(i: &[u8]) -> IResult<&[u8], (Cow<str>, Cow<str>)> {
    separated_pair(nullended_string, tag(b""), nullended_string0)(i)
}



#[cfg(test)]
mod tests {
    use super::*;
    use nom::HexDisplay;

    mod parsers {
        use super::*;

        #[test]
        fn test_null() {
            let res = nil(b"\0");
            match res {
                Ok(_) => {},
                _ => {
                    panic!("")
                }
            }
        }

        #[test]
        fn valid_nonempty_zerobased_string() {
            assert_eq!(zb_string(b"a\0"), Ok((&[] as &[u8], b"a" as &[u8])));
        }

        #[test]
        fn valid_nonempty_nullended_string() {
            assert_eq!(nullended_string(b"a\0"), Ok((&[] as &[u8], Cow::from("a"))));
        }

        #[test]
        fn valid_nullended_string() {
            assert_eq!(nullended_string0(b"a\0"), Ok((&[] as &[u8], Cow::from("a"))));
            assert_eq!(nullended_string0(b"\0"), Ok((&[] as &[u8], Cow::from(""))));
        }

        #[test]
        fn valid_parse_zb_str() {
            assert_eq!(parse_zb_str(b"a\0"), Ok((b"\0" as &[u8], b"a" as &[u8])));
        }

        #[test]
        fn valid_key_value() {
            assert_eq!(
                key_value(b"a\0b\0"),
                Ok((
                    &[] as &[u8],
                    (Cow::from("a"), Cow::from("b"))
            )));
            assert_eq!(
                key_value(b"ab\0\0"),
                Ok((
                    &[] as &[u8],
                    (Cow::from("ab"), Cow::from(""))
            )));
        }
    }
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    mod foma {
        use super::*;

        fn get_header() -> &'static [u8] {
            let data = include_bytes!("../../../tests/data/foma_header.bin");
            data
        }

        #[test]
        fn parse_static_header() {
            let data = get_header();

            println!("bytes:\n{}", &data.to_hex(8));
            let res = static_header(data);
    
            match res {
                Ok((i, o)) => {
                    println!("remaining:\n{}", &i.to_hex(8));
                    println!("parsed: {:?}", o);
                    assert_eq!(o.size, 28);
                },
                _ => {
                    println!("error or incomplete!");
                    panic!("cannot parse static header");
                }
            }
        }

        #[test]
        fn parse_whole_header() -> Result<(), HfstHeaderError> {

            let properties = parse_hfst3_header(
                &mut io::Cursor::new(get_header())
            )?;
            print!("properties={:?}", properties);
            assert_eq!(properties.get("type"), Some(&"FOMA".to_owned()));
            Ok(())
        }
    }
}
