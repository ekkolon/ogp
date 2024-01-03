//! Metadata utility for the Open Graph `video` meta tag.

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::utils::validate_http_url;
use crate::validator::{DimensionsValidator, Validatable};
use crate::{error, Result};

/// `Image` contains Open Graph metadata for the `video` metatag(s).
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Video {
  /// The URL of the video that appears when someone shares the content.
  /// Equivalent to `og:video` | "og:video:url".
  #[serde(alias = "og:video:url")]
  pub url: Option<Url>,

  /// https:// URL for the video.  #[serde(rename = "og:video:secure_url")]
  pub secure_url: Option<Url>,

  /// Equivalent to `og:video`.
  pub mimetype: Option<String>,

  /// Equivalent to `og:video`.
  pub alt: Option<String>,

  /// Equivalent to `og:video`.
  pub width: Option<u32>,

  /// Equivalent to `og:video`.
  pub height: Option<u32>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseVideoUrlError;

impl FromStr for Video {
  type Err = error::Error;
  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    validate_http_url(s)
      .map(|url| Video {
        url: Some(url),
        ..Default::default()
      })
      .map_err(|err| error::Error::UrlParseError(s.into()))
  }
}

impl Validatable for Video {
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
