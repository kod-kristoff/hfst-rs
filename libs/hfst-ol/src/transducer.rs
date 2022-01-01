pub struct TransducerHeader {
    number_of_input_symbols: u16,
    number_of_symbols: u16,
    size_of_transition_index_table: u32,
    size_of_transition_target_table: u32,
    number_of_states: u32,
    number_of_transitions: u32,
    weighted: bool,
    deterministic: bool,
    input_deterministic: bool,
    minimized: bool,
    cyclic: bool,
    has_epsilon_epsilon_transitions: bool,
    has_input_epsilon_transitions: bool,
    has_input_epsilon_cycles: bool,
    has_unweighted_input_epsilon_cycles: bool,
}


impl TransducerHeader {
    pub fn new(
        number_of_input_symbols: u16,
        number_of_symbols: u16,
        size_of_transition_index_table: u32,
        size_of_transition_target_table: u32,
        number_of_states: u32,
        number_of_transitions: u32,
        weighted: bool,
        deterministic: bool,
        input_deterministic: bool,
        minimized: bool,
        cyclic: bool,
        has_epsilon_epsilon_transitions: bool,
        has_input_epsilon_transitions: bool,
        has_input_epsilon_cycles: bool,
        has_unweighted_input_epsilon_cycles: bool,
    ) -> Self {
        log::trace!("Creating TransducerHeader(");
        log::trace!("  number_of_input_symbols: {}", number_of_input_symbols);
        log::trace!("  number_of_symbols: {}", number_of_symbols);
        log::trace!("  size_of_transition_index_table: {}", size_of_transition_index_table);
        log::trace!("  size_of_transition_target_table: {}", size_of_transition_target_table);
        log::trace!("  number_of_states: {}", number_of_states);
        log::trace!("  number_of_transitions: {}", number_of_transitions);
        log::trace!("  weighted: {}", weighted);
        log::trace!("  deterministic: {}", deterministic);
        log::trace!("  input_deterministic: {}", input_deterministic);
        log::trace!("  minimized: {}", minimized);
        log::trace!("  cyclic: {}", cyclic);
        log::trace!("  has_epsilon_epsilon_transitions: {}", has_epsilon_epsilon_transitions);
        log::trace!("  has_input_epsilon_transitions: {}", has_input_epsilon_transitions);
        log::trace!("  has_input_epsilon_cycles: {}", has_input_epsilon_cycles);
        log::trace!("  has_unweighted_input_epsilon_cycles: {}", has_unweighted_input_epsilon_cycles);
        log::trace!(")");
        Self {
            number_of_input_symbols,
            number_of_symbols,
            size_of_transition_index_table,
            size_of_transition_target_table,
            number_of_states,
            number_of_transitions,
            weighted,
            deterministic,
            input_deterministic,
            minimized,
            cyclic,
            has_epsilon_epsilon_transitions,
            has_input_epsilon_transitions,
            has_input_epsilon_cycles,
            has_unweighted_input_epsilon_cycles,
        }
    }

    pub fn symbol_count(&self) -> u16 {
        self.number_of_symbols
    }
    
    pub fn index_table_size(&self) -> u32 {
        self.size_of_transition_index_table
    }
    
    pub fn target_table_size(&self) -> u32 {
        self.size_of_transition_target_table
    }
}
