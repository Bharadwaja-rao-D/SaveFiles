use crate::error::SaveError;
use std::{path::PathBuf, str::FromStr};
#[derive(Debug)]
pub enum Procedure {
    Download,
    Upload,
    List,
    Other,
}

impl FromStr for Procedure {
    type Err = SaveError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(match s {
            "download" => Procedure::Download,
            "upload" => Procedure::Upload,
            "list" => Procedure::List,
            //TODO: Should return an error
            _ => Procedure::Other,
        });
    }
}

pub fn upload(remote: &String, local: &PathBuf) {
}

pub fn download(remote: &String, local: &PathBuf) {}

pub fn list(remote: &String) {}
