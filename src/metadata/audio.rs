//! Metadata utility for the Open Graph `audio` meta tag.

use serde::{Deserialize, Serialize};

use crate::validator::Validator;
use crate::Result;

/// `Image` contains Open Graph metadata for the `audio` metatag(s).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Audio {
    /// The URL of the audio that appears when someone shares the content.
    /// Equivalent to `og:audio` | "og:audio:url".
    #[serde(rename = "og:audio", alias = "og:audio:url")]
    pub url: Option<String>,

    /// https:// URL for the audio.
    #[serde(rename = "og:audio:secure_url")]
    pub secure_url: Option<String>,

    /// Equivalent to `og:audio`.
    #[serde(rename = "og:audio:type")]
    pub mimetype: Option<String>,
}

impl Validator for Audio {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
