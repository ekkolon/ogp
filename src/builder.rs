use std::str::FromStr;

use crate::{
  error::Error,
  metadata::{Audio, Image, Video},
  object_type::{Determiner, ObjectType},
  utils::{validate_http_url, validate_locale},
  Result,
};
use serde::{de::IntoDeserializer, Deserialize, Serialize};
use url::Url;

/// `GeneralSiteInfo` contains Open Graph metadata shared by all object types.
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Metadata {
  /// The canonical URL for your page.
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

  /// A brief description of the content, usually between 2 and 4 sentences.
  #[serde(rename = "og:description")]
  pub description: Option<String>,

  /// A brief description of the content, usually between 2 and 4 sentences.
  #[serde(rename = "og:site_name")]
  pub site_name: Option<String>,

  /// The title of your article without any branding such as your site name.
  #[serde(rename = "og:determiner")]
  pub determiner: Option<Determiner>,

  /// The title of your article without any branding such as your site name.
  #[serde(rename = "og:locale")]
  pub locale: Option<String>,

  /// The title of your article without any branding such as your site name.
  #[serde(rename = "og:locale:alternate")]
  pub locale_alternate: Option<Vec<String>>,

  /// A brief description of the content, usually between 2 and 4 sentences.
  #[serde(default)]
  pub images: Vec<Image>,

  /// A brief description of the content, usually between 2 and 4 sentences.
  #[serde(default)]
  pub videos: Vec<Video>,

  /// A brief description of the content, usually between 2 and 4 sentences.
  #[serde(default)]
  pub audios: Vec<Audio>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MetadataBuilder {
  #[serde(rename = "og:type")]
  pub object_type: ObjectType,

  #[serde(flatten)]
  pub metadata: Metadata,
}

impl MetadataBuilder {
  pub fn new() -> Self {
    MetadataBuilder::default()
  }

  pub fn with_type(object_type: ObjectType) -> MetadataBuilder {
    MetadataBuilder {
      object_type,
      ..Default::default()
    }
  }

  // ==============================
  // region:    Required properties
  pub fn set_title(&mut self, title: impl Into<String>) -> &mut Self {
    self.metadata.title.insert(title.into());
    self
  }

  pub fn set_description(
    &mut self,
    description: impl Into<String>,
  ) -> &mut Self {
    self.metadata.description.insert(description.into());
    self
  }

  pub fn set_url(&mut self, url: impl Into<String>) -> &mut Self {
    match validate_http_url(&url.into()) {
      Err(err) => panic!("error: {}", err),
      Ok(url) => {
        self.metadata.url.insert(url.into());
        self
      }
    }
  }

  pub fn set_image(&mut self, image: impl Into<String>) -> &mut Self {
    // self.image = Some(image.into());
    self
  }

  pub fn set_video(&mut self, video: impl Into<String>) -> &mut Self {
    // self.video = Some(video.into());
    self
  }

  pub fn set_audio(&mut self, audio: impl Into<String>) -> &mut Self {
    // self.audio = Some(audio.into());
    self
  }

  pub fn add_image(&mut self, img: Image) -> &mut Self {
    self.metadata.images.push(img);
    self
  }

  pub fn add_image_url(&mut self, image_url: impl Into<String>) -> &mut Self {
    match Image::from_str(&image_url.into()) {
      Err(err) => panic!("error: {:?}", err),
      Ok(img) => self.metadata.images.push(img),
    }

    self
  }

  pub fn add_video() {}
  pub fn add_audio() {}
  // endregion: Required properties
  // ==============================

  // Optional properties
  pub fn set_site_name(&mut self, site_name: impl Into<String>) -> &mut Self {
    self.metadata.site_name.insert(site_name.into());
    self
  }

  pub fn set_determiner(
    &mut self,
    determiner: Option<Determiner>,
  ) -> &mut Self {
    self.metadata.determiner = determiner;
    self
  }

  pub fn set_locale(&mut self, locale: impl Into<String>) -> &mut Self {
    let val: String = locale.into();
    match validate_locale(&val) {
      Err(err) => panic!("error: {}", err),
      Ok(_) => {
        self.metadata.locale.insert(val);
        self
      }
    }
  }

  pub fn add_locale_alternate(
    &mut self,
    locale: impl Into<String>,
  ) -> &mut Self {
    self
  }

  pub fn get(&self) -> &Metadata {
    &self.metadata
  }
}

pub trait Build<T> {
  fn build(&self) -> Result<T>;
}

pub trait Validate {
  fn validate(&self) -> Result<()>;
}

impl Validate for Metadata {
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
