//! Models for representing Open Graph data

use crate::{object_type::ObjectType, validator::Validatable};
use serde::{Deserialize, Serialize};

pub use self::{
  audio::Audio, builder::OgMetadataBuilder, image::Image, video::Video,
};

mod audio;
mod builder;
mod image;
mod video;

use std::str::FromStr;

use crate::{
  error::Error,
  object_type::Determiner,
  utils::{validate_http_url, validate_locale},
  Result,
};
use serde::de::IntoDeserializer;
use url::Url;

/// `GeneralSiteInfo` contains Open Graph metadata shared by all object types.
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct OgMetadata {
  /// The type of your object, e.g., "video.movie".
  ///
  /// Depending on the type you specify, other properties may also be required.
  #[serde(rename = "og:type")]
  pub object_type: ObjectType,

  /// The canonical URL of your object that will be used as its permanent ID in the graph,
  /// e.g., "https://www.imdb.com/title/tt0117500/".
  ///
  /// This should be the undecorated URL, without session variables, user
  /// identifying parameters, or counters. Likes and Shares for this URL will
  /// aggregate at this URL. For example, mobile domain URLs should point to the
  /// desktop version of the URL as the canonical URL to aggregate Likes and
  /// Shares across different versions of the page.
  #[serde(rename = "og:url")]
  pub url: Option<String>,

  /// The title of your article without any branding such as your site name.
  #[serde(rename = "og:title")]
  pub title: Option<String>,

  /// A one to two sentence description of your object.
  #[serde(rename = "og:description")]
  pub description: Option<String>,

  /// If your object is part of a larger web site, the name which should be
  /// displayed for the overall site. e.g., "IMDb".
  #[serde(rename = "og:site_name")]
  pub site_name: Option<String>,

  /// The word that appears before this object's title in a sentence.
  ///
  /// An enum of (a, an, the, "", auto). If auto is chosen, the consumer
  /// of your data should chose between "a" or "an". Default is "" (blank).
  #[serde(rename = "og:determiner")]
  pub determiner: Option<Determiner>,

  /// The locale these tags are marked up in. Of the format `language_TERRITORY`.
  ///
  /// Default is `en_US`.
  #[serde(rename = "og:locale")]
  pub locale: Option<String>,

  /// An array of other locales this page is available in.
  #[serde(rename = "og:locale:alternate")]
  pub locale_alternate: Option<Vec<String>>,

  /// A brief description of the content, usually between 2 and 4 sentences.
  #[serde(rename = "og:image", default)]
  pub images: Vec<Image>,

  /// A brief description of the content, usually between 2 and 4 sentences.
  #[serde(rename = "og:video", default)]
  pub videos: Vec<Video>,

  /// A brief description of the content, usually between 2 and 4 sentences.
  #[serde(rename = "og:audio", default)]
  pub audios: Vec<Audio>,
}

impl Validatable for OgMetadata {
  fn validate(&self) -> Result<()> {
    let Some(title) = self.title.as_ref() else {
      return Err(Error::MissingRequiredProperty("title".into()));
    };

    let Some(url) = self.url.as_ref() else {
      return Err(Error::MissingRequiredProperty("url".into()));
    };

    let Some(description) = self.description.as_ref() else {
      return Err(Error::MissingRequiredProperty("description".into()));
    };

    Ok(())
  }
}
