//! # Metadata

use std::{
    collections::HashMap,
    error::Error,
    fmt,
    fmt::{Display, Formatter},
};

use serde_json::Value;
use sqlx::types::Uuid;

use crate::error::MetadataExtractionError;

#[derive(Debug)]
pub struct Metadata {
    uuid: Uuid,
    entry: Uuid,
    data: MetadataValues,
}

impl Metadata {
    pub fn new(data: MetadataValues) -> Self {
        Self {
            uuid: Uuid::from_bytes([0; 16]),
            entry: Uuid::from_bytes([0; 16]),
            data,
        }
    }
}

#[derive(Debug)]
pub struct MetadataValues(Value);

impl MetadataValues {
    fn new(v: Value) -> Self {
        Self(v)
    }
}

impl Error for MetadataValues {}

impl Display for MetadataValues {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Unable to parse metadata.")
    }
}

impl TryFrom<exif::Exif> for MetadataValues {
    type Error = MetadataExtractionError;

    fn try_from(e: exif::Exif) -> Result<Self, Self::Error> {
        let mut unparsed = HashMap::new();
        for f in e.fields() {
            let key = format!("{}", f.tag);
            let value = format!("{}", f.display_value().with_unit(&e));
            unparsed.insert(key, value);
        }
        let parsed = serde_json::to_value(unparsed)?;
        Ok(MetadataValues(parsed))
    }
}
