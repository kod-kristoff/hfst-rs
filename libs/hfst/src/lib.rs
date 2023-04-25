pub mod implementations;
mod transducer;

pub use transducer::HfstTransducer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
