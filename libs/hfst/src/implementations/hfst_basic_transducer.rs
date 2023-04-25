use std::collections::BTreeMap;

use super::{hfst_tropical_transducer_transition_data::WeightType, HfstBasicTransition, HfstState};

pub struct HfstBasicTransducer {
    state_vector: Vec<Vec<HfstBasicTransition>>,
    final_weight_map: BTreeMap<HfstState, WeightType>,
}

impl Default for HfstBasicTransducer {
    fn default() -> Self {
        Self {
            state_vector: Vec::default(),
            final_weight_map: BTreeMap::default(),
        }
    }
}
impl HfstBasicTransducer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_state(&mut self, s: HfstState) -> HfstState {
        while self.state_vector.len() <= s {
            self.state_vector.push(Vec::default());
        }
        s
    }

    pub fn set_final_weight(&mut self, s: HfstState, weight: WeightType) {
        self.add_state(s);
        self.final_weight_map.insert(s, weight);
    }

    pub fn add_transition<T: Into<HfstBasicTransition>>(&mut self, s: HfstState, transition: T) {
        self.add_transition_symbols(s, transition.into(), true)
    }
    pub fn add_transition_symbols(
        &mut self,
        s: HfstState,
        transition: HfstBasicTransition,
        add_symbols_to_alphabet: bool,
    ) {
        self.add_state(s);
        self.add_state(transition.get_target_state());
        self.state_vector[s].push(transition);
    }
}
