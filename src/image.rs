//! # Image

use exif::Reader;
use serde_json::Value;
use std::{fs::File, io::BufReader};

use crate::{
    error::MetadataExtractionError,
    metadata::{self, Metadata, MetadataValues},
};

pub struct Image {
    pub path: String,
    pub metadata: metadata::Metadata,
}

impl Image {
    pub fn new(path: &str, metadata: Metadata) -> Self {
        Self {
            path: path.to_string(),
            metadata,
        }
    }

    pub fn from_file(path: &str) -> Result<Self, MetadataExtractionError> {
        let file = File::open(path)?;
        let mut bufreader = BufReader::new(&file);
        let exifreader = Reader::new();
        let extracted = exifreader.read_from_container(&mut bufreader)?;
        let parsed: MetadataValues = extracted.try_into()?;
        let meta = Metadata::new(parsed);
        Ok(Self::new(path, meta))
    }
}
