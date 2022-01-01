use hfst_ol::TransitionWIndex;

#[derive(Debug)]
pub struct PmatchTransducer {
    index_table: Vec<TransitionWIndex>,
}


impl PmatchTransducer {
    pub fn new(
        index_table: Vec<TransitionWIndex>,
    ) -> Self {
        log::trace!("called PmatchTransducer::new()");
        log::trace!("  index_table={:?}", &index_table[0..10]);
        log::trace!("called PmatchTransducer::new()");
        Self {
            index_table,
        }
    }
}
