use std::io;

pub struct BufReader {
    inner: Box<dyn io::Read>,
}

impl BufReader {
    pub fn new(inner: Box<dyn io::Read>) -> Self {
        log::trace!("BufReader::new()");
        Self { inner }
    }
}

impl BufReader {
    pub fn parse<F>(&mut self, f: F) -> Result<F::Output, Box<dyn std::error::Error>>
    where
        F: Fn()
    {
        log::trace!("BufReader::parse()");
        todo!()
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
