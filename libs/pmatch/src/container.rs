use hfst_ol::{DoubleTape, Encoder};

use crate::PmatchAlphabet;


#[derive(Debug)]
pub struct PmatchContainer {
    alphabet: PmatchAlphabet,
    encoder: Encoder,
}


impl PmatchContainer {
    pub fn new(
        alphabet: PmatchAlphabet,
        encoder: Encoder,
    ) -> Self {
        log::trace!("Called PmatchContainer::new(alphabet={:?}, encoder={:?}) ...", alphabet, encoder);
        Self { alphabet, encoder }
    }

    pub fn pmatch<'a>(&mut self, line: &'a str) -> String {
        log::trace!("Called PmatchContainer::pmatch with line='{}'", line);
        let result = self.process(line);
        self.alphabet.stringify(&result)
    }

    fn process(&mut self, line: &str) -> DoubleTape {
        log::trace!("Called PmatchContainer::process");
        let input = Input::init(line, &mut self.alphabet, &mut self.encoder);
        let result = DoubleTape::new();
        result
    }
}

#[derive(Debug)]
struct Input {
    symbols: Vec<u16>,
}

impl Input {
    pub fn init(
        input: &str,
        alphabet: &mut PmatchAlphabet,
        encoder: &mut Encoder,
    ) -> Self {
        use unicode_segmentation::UnicodeSegmentation;

        log::trace!("called Input::init(input={})", input);

        let mut symbols = Vec::new();
        for grapheme in input.graphemes(true) {
            let k = encoder.find_key(grapheme);
            let k = match k {
                None => {
                    let symbol_key = alphabet.add_symbol(grapheme);
                    encoder.read_input_symbol(grapheme, symbol_key);
                    symbol_key
                },
                Some(symbol_key) => *symbol_key
            };
            symbols.push(k);
        }
        Self { symbols }
    }
}
