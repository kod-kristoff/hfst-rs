use std::fs;
use std::io;

use thiserror::Error;

use pmatch::PmatchContainer;

#[derive(Debug, Error)]
pub enum PmatchHfst3Error {
    #[error("Transducer header error")]
    TransducerHeaderError(#[from] hfst_fs::HfstHeaderError),
}

pub struct PmatchHfst3;

impl PmatchHfst3 {
    pub fn from_file(
        mut file: fs::File
    ) -> Result<PmatchContainer, PmatchHfst3Error> {
        log::trace!("Deserializing Pmatch from HFST_OLW 3");
        let properties = hfst_fs::parse_hfst3_header(&mut file)?;
        let transducer_name = match properties.get("name") {
            None => {
                log::warn!("TOP not defined in archive, using first as TOP");
                "TOP"
            },
            Some(name) => {
                if name != "TOP" {
                    log::warn!("TOP not defined in archive, using first as TOP");
                }
                name
            }
        };
        log::debug!("transducer_name = {}", transducer_name);
        Ok(PmatchContainer {})
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
