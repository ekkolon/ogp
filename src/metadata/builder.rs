use std::str::FromStr;

use crate::{
  error::Error,
  metadata::{Audio, Image, OgMetadata, Video},
  object_type::{Determiner, ObjectType},
  utils::{validate_http_url, validate_locale},
  Result,
};
use serde::{de::IntoDeserializer, Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct OgMetadataBuilder {
  #[serde(flatten)]
  metadata: OgMetadata,
}

impl OgMetadataBuilder {
  pub fn new() -> Self {
    OgMetadataBuilder::default()
  }

  pub fn with_type(object_type: ObjectType) -> OgMetadataBuilder {
    OgMetadataBuilder {
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

  pub fn add_image_url(
    &mut self,
    image_url: impl Into<String>,
  ) -> Result<&mut Self> {
    match Image::from_str(&image_url.into()) {
      Err(err) => Err(err),
      Ok(img) => {
        self.metadata.images.push(img);
        Ok(self)
      }
    }
  }

  pub fn add_video(&mut self, video: Video) -> &mut Self {
    self.metadata.videos.push(video);
    self
  }

  pub fn add_video_url(
    &mut self,
    video_url: impl Into<String>,
  ) -> Result<&mut Self> {
    match Video::from_str(&video_url.into()) {
      Err(err) => Err(err),
      Ok(video) => {
        self.metadata.videos.push(video);
        Ok(self)
      }
    }
  }

  pub fn add_audio(&mut self, audio: Audio) -> &mut Self {
    self.metadata.audios.push(audio);
    self
  }

  pub fn add_audio_url(
    &mut self,
    audio_url: impl Into<String>,
  ) -> Result<&mut Self> {
    match Audio::from_str(&audio_url.into()) {
      Err(err) => Err(err),
      Ok(audio) => {
        self.metadata.audios.push(audio);
        Ok(self)
      }
    }
  }
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
    if let Some(ref mut locale_alternate) = self.metadata.locale_alternate {
      locale_alternate.push(locale.into());
    } else {
      self.metadata.locale_alternate.replace(vec![locale.into()]);
    }

    self
  }

  pub fn get_metadata(&self) -> OgMetadata {
    self.metadata.clone()
  }
}

pub trait Build<T> {
  fn build(&self) -> Result<T>;
}
