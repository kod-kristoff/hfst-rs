use std::{fs, io};

use pmatch::PmatchAlphabet;

use crate::PmatchHfst3Error;

pub fn parse_alphabet_le(
    file: &mut fs::File,
    symbol_count: u16,
) -> Result<PmatchAlphabet, PmatchHfst3Error> {
    use io::Read;

    log::trace!("called parse_alphabet_le(symbol_count={})", symbol_count);
    let mut symbol_table: Vec<String> = Vec::with_capacity(symbol_count as usize);
    let num_symbol_bytes = symbol_count as usize * std::mem::size_of::<u16>();
    let mut buffer = vec![0u8; num_symbol_bytes];
    let _ = file.take(num_symbol_bytes as u64).read(&mut buffer)?;

    Ok(PmatchAlphabet::new(symbol_table))
}
