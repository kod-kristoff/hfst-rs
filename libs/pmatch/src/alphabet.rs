use hfst_ol::{DoubleTape, TransducerAlphabet};


#[derive(Debug)]
pub struct PmatchAlphabet {
    symbol_table: Vec<String>,
}


impl PmatchAlphabet {
    pub fn new(symbol_table: Vec<String>) -> Self {
        log::trace!("Called PmatchAlphabet::new(symbol_table={:?})", &symbol_table[0..10]);
        Self { symbol_table }
    }

    pub fn empty() -> Self {
        log::trace!("Called PmatchAlphabet::empty()");
        let symbol_table = Vec::new();
        Self { symbol_table }
    }

    pub fn stringify(&self, tape: &DoubleTape) -> String {
        log::trace!("Called PmatchAlphabet::stringify(tape={:?})", tape);
        String::new()
    }

    pub fn add_symbol(&mut self, symbol: &str) -> u16 {
        log::trace!("Called PmatchAlphabet::add_symbol(symbol={})", symbol);
        let symbol_key = self.symbol_table.len();
        self.symbol_table.push(symbol.to_string());
        symbol_key as u16
    }
}

impl TransducerAlphabet for PmatchAlphabet {
    fn symbol_table(&self) -> &[String] {
        log::trace!("Called PmatchAlphabet::symbol_table()");
        self.symbol_table.as_slice()
    }
}
