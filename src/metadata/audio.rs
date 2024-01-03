//! Metadata utility for the Open Graph `audio` meta tag.

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::utils::validate_http_url;
use crate::validator::Validatable;
use crate::{error, Result};

/// `Image` contains Open Graph metadata for the `audio` metatag(s).
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Audio {
  /// The URL of the audio that appears when someone shares the content.
  /// Equivalent to `og:audio` | "og:audio:url".
  #[serde(alias = "og:audio:url")]
  pub url: Option<Url>,

  /// https:// URL for the audio.
  pub secure_url: Option<Url>,

  /// Equivalent to `og:audio`.
  pub mimetype: Option<String>,
}

impl FromStr for Audio {
  type Err = error::Error;
  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    validate_http_url(s)
      .map(|url| Audio {
        url: Some(url),
        ..Default::default()
      })
      .map_err(|err| error::Error::UrlParseError(s.into()))
  }
}

impl Validatable for Audio {
  fn validate(&self) -> Result<()> {
    Ok(())
  }
}
