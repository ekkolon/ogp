//! Metadata utility for the Open Graph `video` meta tag.

use serde::{Deserialize, Serialize};

use crate::validator::{DimensionsValidator, Validator};
use crate::Result;

/// `Image` contains Open Graph metadata for the `video` metatag(s).
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Video {
  /// The URL of the video that appears when someone shares the content.
  /// Equivalent to `og:video` | "og:video:url".
  #[serde(rename = "og:video", alias = "og:video:url")]
  pub url: Option<String>,

  /// https:// URL for the video.
  #[serde(rename = "og:video:secure_url")]
  pub secure_url: Option<String>,

  /// Equivalent to `og:video`.
  #[serde(rename = "og:video:type")]
  pub mimetype: Option<String>,

  /// Equivalent to `og:video`.
  #[serde(rename = "og:video:alt")]
  pub alt: Option<String>,

  /// Equivalent to `og:video`.
  #[serde(rename = "og:video:width")]
  pub width: Option<u32>,

  /// Equivalent to `og:video`.
  #[serde(rename = "og:video:height")]
  pub height: Option<u32>,
}

impl Validator for Video {
  fn validate(&self) -> Result<()> {
    self.validate_dimensions().unwrap();

    Ok(())
  }
}

impl DimensionsValidator for Video {
  fn width(&self) -> Option<u32> {
    self.width
  }

  fn height(&self) -> Option<u32> {
    self.height
  }
}
