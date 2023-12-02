use crate::{
  error::Error,
  metadata::{audio, image, video},
  object_type::ObjectType,
  Result,
};
use serde::{de::IntoDeserializer, Deserialize, Serialize};

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
  #[serde(rename = "og:determinator")]
  pub determinator: Option<String>,

  /// The title of your article without any branding such as your site name.
  #[serde(rename = "og:locale")]
  pub locale: Option<String>,

  /// The title of your article without any branding such as your site name.
  #[serde(rename = "og:locale")]
  pub locale_alternate: Option<Vec<String>>,

  /// A brief description of the content, usually between 2 and 4 sentences.
  #[serde(default)]
  pub image: Option<image::Image>,

  /// A brief description of the content, usually between 2 and 4 sentences.
  #[serde(default)]
  pub video: Option<video::Video>,

  /// A brief description of the content, usually between 2 and 4 sentences.
  #[serde(default)]
  pub audio: Option<audio::Audio>,

  /// A brief description of the content, usually between 2 and 4 sentences.
  #[serde(default)]
  pub images: Vec<image::Image>,

  /// A brief description of the content, usually between 2 and 4 sentences.
  #[serde(default)]
  pub videos: Vec<video::Video>,

  /// A brief description of the content, usually between 2 and 4 sentences.
  #[serde(default)]
  pub audios: Vec<audio::Audio>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MetadataBuilder {
  #[serde(flatten)]
  pub metadata: Metadata,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ObjectMetadata {
  #[serde(rename = "og:type")]
  pub object_type: ObjectType,

  #[serde(flatten)]
  pub metadata: Metadata,
}

impl MetadataBuilder {
  pub fn new() -> Self {
    MetadataBuilder::default()
  }

  pub fn with_type(object_type: ObjectType) -> ObjectMetadata {
    ObjectMetadata {
      object_type,
      ..Default::default()
    }
  }

  // ==============================
  // region:    Required properties
  pub fn set_title(&mut self, title: impl Into<String>) -> &mut Self {
    self.metadata.title = Some(title.into());
    self
  }

  pub fn set_description(
    &mut self,
    description: impl Into<String>,
  ) -> &mut Self {
    self.metadata.description = Some(description.into());
    self
  }

  pub fn set_url(&mut self, url: impl Into<String>) -> &mut Self {
    self.metadata.url = Some(url.into());
    self
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

  pub fn add_image() {}
  pub fn add_video() {}
  pub fn add_audio() {}
  // endregion: Required properties
  // ==============================

  // Optional properties
  pub fn set_site_name(&mut self, site_name: impl Into<String>) -> &mut Self {
    self.metadata.site_name = Some(site_name.into());
    self
  }

  pub fn set_determinator(
    &mut self,
    determinator: impl Into<String>,
  ) -> &mut Self {
    self.metadata.determinator = Some(determinator.into());
    self
  }

  pub fn set_locale(&mut self, locale: impl Into<String>) -> &mut Self {
    self.metadata.locale = Some(locale.into());
    self
  }

  pub fn add_locale_alternate(
    &mut self,
    locale: impl Into<String>,
  ) -> &mut Self {
    self
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

    let Some(image) = self.image.as_ref() else {
      return Err(Error::MissingRequiredProperty("image".into()));
    };

    Ok(())
  }
}
