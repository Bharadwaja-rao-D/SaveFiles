use std::{fs::File, path::PathBuf, process::exit};

use serde::Deserialize;

use crate::error::SaveError;

use self::{
    opts::Args,
    procedure::{download, list, upload, Procedure},
};

pub mod opts;
pub mod procedure;

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub password: String,
}

//TODO: Should make this more safer
pub fn authenticate(config: PathBuf) -> bool {
    let file = File::open(config).unwrap();
    let _user: Vec<UserInfo> = serde_json::from_reader(file).unwrap();

    //send a post request to the server containing name and password and verify it

    return true;
}

//checks authentication and calls the appropriate function
pub fn handler(args: Args) -> Result<(), SaveError> {
    if !authenticate(args.config) {
        println!("Authentication failed");
        exit(1);
    }

    let local = args.local;
    let remote = args.remote;

    match args.procedure {
        Procedure::Upload => upload(&remote, &local),
        Procedure::Download => download(&remote, &local),
        Procedure::List => list(&remote),
        Procedure::Other => {
            //TODO: Handle it smoothly
            panic!("Invalid procedure given")
        }
    }

    Ok(())
}
