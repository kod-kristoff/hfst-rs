pub struct BufParser<R> {
    inner: R,
}

impl BufParser {
    pub fn new() -> Self {
        log::trace!("BufParser::new()");
        Self {}
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
