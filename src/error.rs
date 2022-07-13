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

    MetadataNotFound(String),

    Invalid(String),

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

impl From<exif::Error> for MetadataExtractionError {
    fn from(err: exif::Error) -> Self {
        match err {
            exif::Error::Io(e) => Self::IoError(format!("exif IO error: {}", e)),
            exif::Error::NotFound(e) => {
                Self::MetadataNotFound(format!("Unable to locate exif metadata in file: {}", e))
            }
            _ => Self::UnknownError,
        }
    }
}

impl From<serde_json::Error> for MetadataExtractionError {
    fn from(err: serde_json::Error) -> Self {
        Self::Invalid(format!("Unable to parse metadata to json: {}", err))
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
            Self::MetadataNotFound(s) => write!(f, "Missing metadata in file: {}", s),
            Self::UnknownError => write!(f, "Unknown system error!",),
            Self::Invalid(s) => write!(f, "Unable to process metadata: {}", s),
        }
    }
}

#[derive(Debug)]
pub enum SystemError {
    DatabaseConnection(String),
    Unknown(String),
}

impl Error for SystemError {}

impl Display for SystemError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::DatabaseConnection(s) => write!(f, "Unable to connect to database: {}", s),
            Self::Unknown(s) => write!(f, "Error from unknown context: {}", s),
        }
    }
}

impl From<MetadataExtractionError> for SystemError {
    fn from(e: MetadataExtractionError) -> Self {
        Self::Unknown(format!("Unable to extract metadata: {}", e))
    }
}

impl From<sqlx::Error> for SystemError {
    fn from(e: sqlx::Error) -> Self {
        Self::DatabaseConnection(format!("Database connection error: {}", e))
    }
}
