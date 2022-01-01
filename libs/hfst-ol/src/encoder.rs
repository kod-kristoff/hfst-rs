use std::collections::HashMap;

use crate::TransducerAlphabet;

#[derive(Debug)]
pub struct Encoder {
    letters: HashMap<String, u16>,
}


impl Encoder {
    pub fn new() -> Self {
        log::trace!("called Encoder::new()");
        let letters = HashMap::new();
        Self { letters }
    }

    pub fn from_alphabet(alphabet: &dyn TransducerAlphabet) -> Self {
        log::trace!("called Encoder::from_alphabet()");
        let letters: HashMap<String, u16> = alphabet
            .symbol_table()
            .iter()
            .enumerate()
            .map(|(i, s)| (s.to_string(), i as u16))
            .collect();
        Self { letters }
    }

    pub fn find_key(&self, key: &str) -> Option<&u16> {
        self.letters.get(key)
    }

    pub fn read_input_symbol(&mut self, key: &str, symbol_key: u16) {
        log::trace!("called Encoder::read_input_symbol(key={}, symbol_key={})", key, symbol_key);
        self.letters.insert(key.to_string(), symbol_key);
    }
}
