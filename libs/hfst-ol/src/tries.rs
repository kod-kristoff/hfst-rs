#[derive(Debug)]
pub struct OlLetterTrie {
    letters: Vec<Box<OlLetterTrie>>,
    symbols: Vec<u16>,
}


impl OlLetterTrie {
    pub fn new() -> Self {
        log::trace!("called OlLetterTrie::new()");
        Self {
            letters: Vec::new(),
            symbols: Vec::new(),
        }
    }
}
