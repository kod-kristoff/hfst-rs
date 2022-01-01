pub mod streaming;
mod transducer_header;

pub use transducer_header::{
    parse_transducer_header,
    parse_transducer_header_le,
    ParseError,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
