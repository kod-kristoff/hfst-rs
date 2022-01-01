use std::{fs, io, mem};

use nom::{
    multi::many1,
    number::complete::{be_u16, be_u32, le_u16, le_u32},
    IResult,
};

use hfst_ol::TransitionWIndex;
use pmatch::{PmatchAlphabet, PmatchTransducer};

use crate::PmatchHfst3Error;


pub fn parse_pmatch_transducer_le(
    file: &mut fs::File,
    index_table_size: u32,
    target_table_size: u32,
    alphabet: &PmatchAlphabet,
    name: &str,
) -> Result<PmatchTransducer, PmatchHfst3Error> {
    use io::Read;

    log::trace!("called parse_pmatch_transducer_le(");
    log::trace!("  index_table_size={},", index_table_size);
    log::trace!("  target_table_size={},", target_table_size);
    log::trace!("  alphabet={:?}", alphabet);
    log::trace!("  name={}", name);
    log::trace!(")");
    let num_index_table_bytes = index_table_size as usize * (mem::size_of::<u16>() + mem::size_of::<u32>()); 
    let mut buffer = vec![0u8; num_index_table_bytes]; 
    let _ = file.take(num_index_table_bytes as u64).read(&mut buffer)?;
    let index_table: Vec<TransitionWIndex> = parse_index_table_le(&buffer, index_table_size)?;
    Ok(PmatchTransducer::new(
        index_table,
    ))
}

pub fn parse_index_table_le(
    input: &[u8],
    table_size: u32,
) -> Result<Vec<TransitionWIndex>, PmatchHfst3Error> {
    log::trace!("called parse_index_table(table_size={})", table_size);
    let res = transition_w_index_vec_le(input);
    match res {
        Ok((remaining, vec)) => {
            if remaining != b"" {
                return Err(
                    PmatchHfst3Error::IndexTableError(
                        format!(
                            "Non-empty remaining: '{}'",
                            String::from_utf8_lossy(remaining)
                        )
                    )
                )
            }
            Ok(vec)
        },
        _ => {
            panic!("")
        }
    }
} 

pub fn transition_w_index_vec_le(input: &[u8]) -> IResult<&[u8], Vec<TransitionWIndex>> {
    many1(transition_w_index_le)(input)
}

pub fn transition_w_index_le(
    input: &[u8]
) -> IResult<&[u8], TransitionWIndex> {
    let (input, input_symbol) = be_u16(input)?;
    let (input, first_transition_index) = be_u32(input)?;
    Ok((input, TransitionWIndex::new(input_symbol, first_transition_index)))
} 
