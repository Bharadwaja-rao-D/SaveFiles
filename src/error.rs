use thiserror::Error;

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("input errors")]
    IOError(#[from] std::io::Error),
    #[error("attempting to get xdg_config_directory and failed.")]
    XDGError(#[from] xdg::BaseDirectoriesError),
}

