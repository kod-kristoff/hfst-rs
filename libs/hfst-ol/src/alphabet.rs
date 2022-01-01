pub trait TransducerAlphabet {
    fn symbol_table(&self) -> &[String];
}
