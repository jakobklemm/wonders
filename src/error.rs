//! # Global error types

use std::{
    error::Error,
    fmt,
    fmt::{Display, Formatter},
    io,
};

#[derive(Debug)]
pub enum MetadataExtractionError {
    DirectoryError,

    FileNotFound(String),

    IoError(String),

    UnknownError,
}

impl Error for MetadataExtractionError {}

impl From<io::Error> for MetadataExtractionError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            _ => Self::IoError(format!(
                "Internal IO error: {}, {}",
                err.kind().to_string(),
                err
            )),
        }
    }
}

impl From<()> for MetadataExtractionError {
    fn from(_e: ()) -> Self {
        Self::UnknownError
    }
}

impl Display for MetadataExtractionError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::DirectoryError => write!(f, "Error: path points to a directory, not a file!"),
            Self::FileNotFound(s) => write!(f, "Error: could not find file {}.", s),
            Self::IoError(s) => write!(f, "Generic IO error: {}", s),
            Self::UnknownError => write!(f, "Unknown system error!",),
        }
    }
}
