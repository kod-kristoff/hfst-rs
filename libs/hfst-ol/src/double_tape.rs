#[derive(Debug)]
pub struct DoubleTape;

impl DoubleTape {
    pub fn new() -> Self {
        log::trace!("called DoubleTape::new()");
        Self {}
    }
}
