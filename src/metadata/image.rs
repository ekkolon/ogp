//! Metadata utility for the Open Graph `image` meta tag.

use serde::{Deserialize, Serialize};

use crate::validator::{DimensionsValidator, Validator};
use crate::Result;

/// `Image` contains Open Graph metadata for the `image` metatag(s).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
  /// An image URL which should represent your object within the graph.
  /// This image appears when someone shares the content.
  ///
  /// Represents both the `og:image` | "og:image:url" property.
  #[serde(rename = "og:image", alias = "og:image:url")]
  pub url: Option<String>,

  /// An alternate url to use if the webpage requires HTTPS.
  #[serde(rename = "og:image:secure_url")]
  pub secure_url: Option<String>,

  /// A MIME type for this image.
  #[serde(rename = "og:image:type")]
  pub mimetype: Option<String>,

  /// A description of what is in the image (not a caption).
  /// If the page specifies an og:image it should specify `og:image:alt`
  #[serde(rename = "og:image:alt")]
  pub alt: Option<String>,

  /// The number of pixels wide.
  #[serde(rename = "og:image:width")]
  pub width: Option<u32>,

  /// The number of pixels high.
  #[serde(rename = "og:image:height")]
  pub height: Option<u32>,
}

impl Validator for Image {
  fn validate(&self) -> Result<()> {
    self.validate_dimensions().unwrap();

    Ok(())
  }
}

impl DimensionsValidator for Image {
  fn width(&self) -> Option<u32> {
    self.width
  }

  fn height(&self) -> Option<u32> {
    self.height
  }
}
