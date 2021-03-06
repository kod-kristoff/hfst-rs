mod alphabet;
mod container;
mod transducer;

pub use alphabet::PmatchAlphabet;
pub use container::PmatchContainer;
pub use transducer::PmatchTransducer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
