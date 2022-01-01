use std::{fs, io};

use nom::{
    number::streaming::{le_u16, le_u32},
    IResult,
};

use hfst_ol::TransducerHeader;
use crate::ParseError;

const HEADER_SIZE: usize = 56;


pub fn parse_transducer_header(
    is: &mut fs::File
) -> Result<TransducerHeader, ParseError> {
    log::trace!("Called parse_transducer_header ...");
    unimplemented!()
}

pub fn parse_transducer_header_le(
    is: &mut fs::File
) -> Result<TransducerHeader, ParseError> {
    use io::Read;

    log::trace!("Called parse_transducer_header_le ...");

    let mut buffer = [0u8; HEADER_SIZE];
    let _ = is.take(HEADER_SIZE as u64).read(&mut buffer);
    match transducer_header_le(&buffer) {
        Ok((_, transducer)) => Ok(transducer),
        Err(error) => {
            panic!("Error: {}", error);
        }
    }
}

pub fn transducer_header(input: &[u8]) -> IResult<&[u8], TransducerHeader, ()> {
    let (input, number_of_input_symbols) = le_u16(input)?;
    let (input, number_of_symbols) = le_u16(input)?;
    let (input, size_of_transition_index_table) = le_u32(input)?;
    let (input, size_of_transition_target_table) = le_u32(input)?;
    let (input, number_of_states) = le_u32(input)?;
    let (input, number_of_transitions) = le_u32(input)?;
    let (input, weighted) = bool_as_le_u32(input)?;
    let (input, deterministic) = bool_as_le_u32(input)?;
    let (input, input_deterministic) = bool_as_le_u32(input)?;
    let (input, minimized) = bool_as_le_u32(input)?;
    let (input, cyclic) = bool_as_le_u32(input)?;
    let (input, has_epsilon_epsilon_transitions) = bool_as_le_u32(input)?;
    let (input, has_input_epsilon_transitions) = bool_as_le_u32(input)?;
    let (input, has_input_epsilon_cycles) = bool_as_le_u32(input)?;
    let (input, has_unweighted_input_epsilon_cycles) = bool_as_le_u32(input)?;
    Ok((input, TransducerHeader::new(
        number_of_input_symbols,
        number_of_symbols,
        size_of_transition_index_table,
        size_of_transition_target_table,
        number_of_states,
        number_of_transitions,
        weighted,
        deterministic,
        input_deterministic,
        minimized,
        cyclic,
        has_epsilon_epsilon_transitions,
        has_input_epsilon_transitions,
        has_input_epsilon_cycles,
        has_unweighted_input_epsilon_cycles,
    )))
}

// pub fn transducer_header_le<E: nom::error::ParseError<&[u8]>>(input: &[u8]) -> IResult<&[u8], TransducerHeader, E>
//     where nom::Err<E>: From<nom::Err<nom::error::Error<&[u8]>>>
// {
pub fn transducer_header_le(input: &[u8]) -> IResult<&[u8], TransducerHeader, ()> {
    let (input, number_of_input_symbols) = le_u16(input)?;
    let (input, number_of_symbols) = le_u16(input)?;
    let (input, size_of_transition_index_table) = le_u32(input)?;
    let (input, size_of_transition_target_table) = le_u32(input)?;
    let (input, number_of_states) = le_u32(input)?;
    let (input, number_of_transitions) = le_u32(input)?;
    let (input, weighted) = bool_as_le_u32(input)?;
    let (input, deterministic) = bool_as_le_u32(input)?;
    let (input, input_deterministic) = bool_as_le_u32(input)?;
    let (input, minimized) = bool_as_le_u32(input)?;
    let (input, cyclic) = bool_as_le_u32(input)?;
    let (input, has_epsilon_epsilon_transitions) = bool_as_le_u32(input)?;
    let (input, has_input_epsilon_transitions) = bool_as_le_u32(input)?;
    let (input, has_input_epsilon_cycles) = bool_as_le_u32(input)?;
    let (input, has_unweighted_input_epsilon_cycles) = bool_as_le_u32(input)?;
    Ok((input, TransducerHeader::new(
        number_of_input_symbols,
        number_of_symbols,
        size_of_transition_index_table,
        size_of_transition_target_table,
        number_of_states,
        number_of_transitions,
        weighted,
        deterministic,
        input_deterministic,
        minimized,
        cyclic,
        has_epsilon_epsilon_transitions,
        has_input_epsilon_transitions,
        has_input_epsilon_cycles,
        has_unweighted_input_epsilon_cycles,
    )))
}

pub fn bool_as_le_u32(input: &[u8]) -> IResult<&[u8], bool, ()> {
    let (input, prop) = le_u32(input)?;
    Ok((input, prop != 0u32))
}
