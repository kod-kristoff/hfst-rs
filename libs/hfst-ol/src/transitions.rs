
#[derive(Copy, Clone, Debug)]
pub struct TransitionWIndex {
    input_symbol: u16,
    first_transition_index: u32,
}

impl TransitionWIndex {
    pub fn new(
        input_symbol: u16,
        first_transition_index: u32
    ) -> Self {
        Self { input_symbol, first_transition_index }
    }
}
