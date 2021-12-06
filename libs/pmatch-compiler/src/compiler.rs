use std::collections::HashMap;

use hfst::HfstTransducer;


pub struct PmatchCompiler {
    verbose: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum CompileError {
    #[error("?")]
    UnknownError,
}

impl PmatchCompiler {
    pub fn new() -> Self {
        log::trace!("Creating PmatchCompiler ...");
        Self { verbose: false }
    }

    pub fn compile(
        &self,
        program: &str
    ) -> Result<HashMap<String, Box<dyn HfstTransducer>>, CompileError> {
        log::trace!("Compiling: '{}'", program);
        let retval = HashMap::new();
        Ok(retval)
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        log::trace!("Setting verbose={}", verbose);
        self.verbose = verbose;
    }
}
