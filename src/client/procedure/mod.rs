use std::str::FromStr;

use crate::error::SaveError;
#[derive(Debug)]
pub enum Procedure {
    Download,
    Upload,
    Other,
}

impl FromStr for Procedure {
    type Err = SaveError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(match s {
            "download" => Procedure::Download,
            "upload" => Procedure::Upload,
            //TODO: Should return an error
            _ => Procedure::Other,
        });
    }
}
