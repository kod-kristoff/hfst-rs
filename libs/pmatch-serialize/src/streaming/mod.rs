
mod alphabet;
mod transducer;

pub use alphabet::{alphabet_le, AlphabetParserLe, parse_alphabet_le};
// pub use transducer::pmatch_transducer_le;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
