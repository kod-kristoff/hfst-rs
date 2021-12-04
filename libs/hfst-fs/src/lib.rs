use std::collections::HashMap;
use std::fs;
use std::io;

use byteorder::{BigEndian, LittleEndian};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HfstHeaderError {
    #[error("Invalid header (expected: {expected:?}, got: {found:?})")]
    InvalidHeader {
        expected: String,
        found: String
    },
    #[error("Invalid header")]
    InvalidHeaderStart(#[from] nom::Err<E>),
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
            
        }
    }
    log::debug!("Header size: {}", remaining_header_len);
    if &buffer[0..header1.len()] != header1.as_bytes() {
        log::warn!("Found unknown header: '{:?}'", &buffer[0..header1.len()]);
        instream.seek(io::SeekFrom::Start(start_position))?;
        return Err(HfstHeaderError::InvalidHeader {
            expected: header1.to_owned(),
            found: String::from_utf8_lossy(&buffer[0..header1.len()]).into_owned(),
        });
    }
    
    let remaining_header_len = io::Cursor::new(&buffer[5..7]).read_u16::<BigEndian>()?; 
    log::trace!("remaining_header_len = {}", remaining_header_len);
    // let _ = instream.by_ref().take(1).read(&mut buffer)?;
    if &buffer[7..8] != b"\0" {
        return Err(HfstHeaderError::MalformedHeader(
            "Missing NULL after size".to_owned()
        )); 
    }
    // if remaining_header_len as usize > BUFFER_SIZE {
    //     unimplemented!();
    // }
    let mut buffer = vec![0; remaining_header_len as usize];
    let _ = instream.take(remaining_header_len as u64).read(&mut buffer)?;
    // log::trace!("buffer = {:?}", &buffer);
    // log::trace!("buffer = {:?}", String::from_utf8_lossy(&buffer));
    // let _ = instream.by_ref().take(remaining_header_len as u64).read(&mut buffer)?;
    if &buffer[(remaining_header_len as isize - 1) as usize..remaining_header_len as usize] != b"\0" {
        return Err(HfstHeaderError::MalformedHeader(
            "Missing NULL after header".to_owned()
        )); 
    }
    let mut properties = HashMap::new();
    Ok(properties)
}
use nom::{IResult, do_parse, tag};
use nom::number::complete::be_u16;

#[derive(Debug, Eq, PartialEq)]
pub struct Hfst;

pub fn header(input: &[u8]) -> IResult<&[u8], Hfst> {
    do_parse!(input,
        tag!("HFST")    >>
        tag!("\0")      >>
        ( Hfst )
    )
}

#[derive(Debug, Eq, PartialEq)]
pub struct HeaderSize {
    size: u16,
}

pub fn header_size(input: &[u8]) -> IResult<&[u8], HeaderSize> {
    do_parse!(input,
        size:   be_u16      >>
                tag!("\0")  >>
                ( HeaderSize { size } )
    )
}

pub fn static_header(input: &[u8]) -> IResult<&[u8], HeaderSize> {
    do_parse!(input,
                header          >>
        size:   header_size     >>
                ( size )
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::HexDisplay;

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
