use std::fs;
use std::io;

use thiserror::Error;
use nom_bufreader::bufreader::BufReader as NomBufReader;

use bufparser::BufReader;
use hfst_fs::OwnedParseError;
use hfst_ol::{Encoder, TransducerHeader};
use hfst_ol_parser::{
    parse_transducer_header_le,
    streaming::transducer_header_le,
};
use pmatch::{PmatchAlphabet, PmatchContainer, PmatchTransducer};

mod alphabet;
mod transducer;
pub mod streaming;

// pub use alphabet::parse_alphabet_le;
pub use transducer::parse_pmatch_transducer_le;
use streaming::{alphabet_le, AlphabetParserLe, parse_alphabet_le};


#[derive(Debug, Error)]
pub enum PmatchHfst3Error {
    #[error("Hfst header error")]
    HfstHeaderError(#[from] hfst_fs::HfstHeaderError),
    #[error("Transducer header error")]
    TransducerHeaderError(#[from] hfst_ol_parser::ParseError),
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("Invalid table: {0}")]
    IndexTableError(String),
    #[error("Parse error: {0}")]
    NomParseError(String),
}

impl From<nom_bufreader::Error<nom::error::Error<&[u8]>>> for PmatchHfst3Error {
    fn from(err: nom_bufreader::Error<nom::error::Error<&[u8]>>) -> Self {
        Self::NomParseError(format!("{:?}", err))
    }
}

impl From<nom_bufreader::Error<OwnedParseError>> for PmatchHfst3Error {
    fn from(err: nom_bufreader::Error<OwnedParseError>) -> Self {
        Self::NomParseError(format!("{:?}", err))
    }
}

impl From<nom_bufreader::Error<()>> for PmatchHfst3Error {
    fn from(err: nom_bufreader::Error<()>) -> Self {
        Self::NomParseError(format!("{:?}", err))
    }
}

pub struct PmatchHfst3;

impl PmatchHfst3 {
    pub fn from_file(
        mut file: fs::File
    ) -> Result<PmatchContainer, PmatchHfst3Error> {
        log::trace!("Deserializing Pmatch from HFST_OLW");
        let properties = hfst_fs::parse_hfst3_header(&mut file)?;
        let transducer_name = match properties.get("name") {
            None => {
                log::warn!("TOP not defined in archive, using first as TOP");
                "TOP"
            },
            Some(name) => {
                if name != "TOP" {
                    log::warn!("TOP not defined in archive, using first as TOP");
                }
                name
            }
        };
        log::debug!("transducer_name = {}", transducer_name);
        unimplemented!()
        // Ok(PmatchContainer::new())
    }

    pub fn from_file_le(
        mut file: fs::File
    ) -> Result<PmatchContainer, PmatchHfst3Error> {
        use nom_bufreader::Parse;

        log::trace!("Deserializing Pmatch from HFST_OLW");
        let properties = hfst_fs::parse_hfst3_header_le(&mut file)?;
        let transducer_name = match properties.get("name") {
            None => {
                log::warn!("TOP not defined in archive, using first as TOP");
                "TOP"
            },
            Some(name) => {
                if name != "TOP" {
                    log::warn!("TOP not defined in archive, using first as TOP");
                }
                name
            }
        };
        log::debug!("transducer_name = {}", transducer_name);
        match properties.get("type") {
            None => {
                log::warn!("type information missing from archive");
            },
            Some(type_) => {
                if type_ != "HFST_OLW" {
                    log::warn!("archive type isn't weighted optimized-lookup according to header");
                }
            }
        };
        let count_patterns = false;
        let delete_patterns = false;
        let extract_patterns = false;
        let locate_mode = false;
        let mark_patterns = true;
        let max_context_length = 254;
        let max_recursion = 5000;
        let need_separators = true;
        let xerox_composition = true;

        // let mut reader = NomBufReader::new(file);
        let mut reader = BufReader::new(Box::new(file));
        let header: TransducerHeader = reader.parse(transducer_header_le)?;
            //parse_transducer_header_le(&mut file)?;
        let alphabet: PmatchAlphabet = parse_alphabet_le(
            &mut reader,
            header.symbol_count(),
        )?;
        // let alphabet: PmatchAlphabet = parse_alphabet_le(
        //     &mut file,
        //     header.symbol_count(),
        // )?;
        let encoder = Encoder::from_alphabet(&alphabet);
        let toplevel: PmatchTransducer = parse_pmatch_transducer_le(
            &mut file,
            header.index_table_size(),
            header.target_table_size(),
            &alphabet,
            "TOP",
        )?;
        Ok(PmatchContainer::new(alphabet, encoder))
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
