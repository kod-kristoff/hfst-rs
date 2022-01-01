use std::{fs, io};

use nom::IResult;
use nom_bufreader::bufreader::BufReader as NomBufReader;

use hfst_fs::streaming::nullended_string;
use pmatch::PmatchAlphabet;

use crate::PmatchHfst3Error;

pub fn alphabet_le(
    file: &mut fs::File,
    symbol_count: u16,
) -> Result<PmatchAlphabet, PmatchHfst3Error> {
    use io::Read;

    log::trace!("called alphabet_le(symbol_count={})", symbol_count);
    let mut symbol_table: Vec<String> = Vec::with_capacity(symbol_count as usize);
    let num_symbol_bytes = symbol_count as usize * std::mem::size_of::<u16>();
    let mut buffer = vec![0u8; num_symbol_bytes];
    let _ = file.take(num_symbol_bytes as u64).read(&mut buffer)?;

    Ok(PmatchAlphabet::new(symbol_table))
}

pub fn parse_alphabet_le<R>(
    reader: &mut NomBufReader<R>,
    symbol_count: u16,
) -> Result<PmatchAlphabet, PmatchHfst3Error>
where
    R: io::Read,
{
    use nom_bufreader::Parse;

    log::trace!("called parse_alphabet_le(symbol_count={})", symbol_count);
    let mut symbol_table: Vec<String> = Vec::with_capacity(symbol_count as usize);

    for _ in 0..symbol_count {
        let symbol = reader.parse(nullended_string)?;
        symbol_table.push(String::from(symbol));
    }

    Ok(PmatchAlphabet::new(symbol_table))
}

pub struct AlphabetParserLe {
    symbol_count: u16, 
}

impl AlphabetParserLe {
    pub fn new(symbol_count: u16) -> Self {
        log::trace!("called AlphabetParserLe::new(symbol_count={})", symbol_count);
        Self { symbol_count }
    }
}

