mod transducer_header;

pub use transducer_header::{
    bool_as_le_u32,
    transducer_header,
    transducer_header_le,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
