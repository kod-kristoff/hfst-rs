use super::HfstState;

#[derive(Debug, Clone)]
pub struct HfstBasicTransition {
    target_state: HfstState,
}

impl HfstBasicTransition {
    pub fn new(target_state: HfstState, isymbol: String, osymbol: String, weight: f32) -> Self {
        Self { target_state }
    }

    pub fn get_target_state(&self) -> HfstState {
        self.target_state
    }
}
