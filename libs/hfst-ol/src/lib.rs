mod alphabet;
mod double_tape;
mod encoder;
mod transducer;
mod transitions;
mod tries;

pub use alphabet::TransducerAlphabet;
pub use double_tape::DoubleTape;
pub use encoder::Encoder;
pub use transducer::TransducerHeader;
pub use transitions::TransitionWIndex;
pub use tries::OlLetterTrie;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
