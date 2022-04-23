use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

use crate::{error::SaveError, procedure::Procedure};

///cli command:  Upload remote local --config
#[derive(Debug, StructOpt)]
pub struct Opts {
    #[structopt()]
    procedure: Procedure,

    #[structopt()]
    remote: String,

    //TODO: In case of download it should be the directory path
    //In case of upload it should be the file
    #[structopt(parse(from_os_str))]
    local: PathBuf,

    //used to verify the user and his password
    #[structopt(long, short)]
    config: Option<String>,
}

//use this struct for all other stuff
#[derive(Debug)]
pub struct Args {
    pub procedure: Procedure,
    pub remote: String,
    pub local: PathBuf,
    pub config: PathBuf,
}

pub fn extract_config_file(str: &Option<String>) -> Result<PathBuf, SaveError> {
    match str {
        //check if the file exists there or not
        Some(s) => {
            let file_path = PathBuf::from(s);
            match File::open(&file_path) {
                Ok(_) => return Ok(file_path),
                Err(_) => {
                    File::create(&file_path).expect("File creation failed");
                    return Ok(file_path);
                }
            }
        }
        None => {
            let xdg_dirs = xdg::BaseDirectories::with_prefix("save_file")?;
            let config_path = xdg_dirs.place_config_file("save_file.json")?;
            File::create(&config_path).expect("File creation failed");
            return Ok(config_path);
        }
    }
}

#[test]
fn test_extract_config_file() {
    let default_path = "/home/bharadwaja/.config/save_file/save_file.json";
    assert_eq!(
        extract_config_file(&None).unwrap().to_str().unwrap(),
        default_path
    );
    assert_eq!(
        extract_config_file(&Some(".".to_string()))
            .unwrap()
            .to_str()
            .unwrap(),
        "."
    );
}

impl TryFrom<Opts> for Args {
    type Error = std::io::Error;
    fn try_from(value: Opts) -> Result<Self, Self::Error> {
        return Ok(Args {
            procedure: value.procedure,
            remote: value.remote,
            local: value.local,
            config: extract_config_file(&value.config).unwrap(),
        });
    }
}
