#[derive(Debug)]
pub struct PmatchContainer;


impl PmatchContainer {
    pub fn from_name(name: &str) -> Self {
        log::trace!("Creating PmatchContainer from '{}'", name);
        Self {}
    }

    pub fn pmatch<'a>(&self, line: &'a str) -> &'a str {
        log::trace!("Called PmatchContainer::pmatch with line='{}'", line);
        line
    }
}
