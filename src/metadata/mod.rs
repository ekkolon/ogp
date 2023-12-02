//! Models for representing Open Graph data

use crate::object_type::ObjectType;
use serde::{Deserialize, Serialize};

use self::{audio::Audio, image::Image, video::Video};

pub mod audio;
pub mod image;
pub mod video;

/// `GeneralSiteInfo` contains Open Graph metadata shared by all object types.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicMetadata {
  /// The title of your article without any branding such as your site name.
  #[serde(rename = "og:type")]
  pub object_type: Option<ObjectType>,

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
  #[serde(default)]
  pub images: Vec<image::Image>,

  /// A brief description of the content, usually between 2 and 4 sentences.
  #[serde(default)]
  pub videos: Vec<video::Video>,

  /// A brief description of the content, usually between 2 and 4 sentences.
  #[serde(default)]
  pub audios: Vec<audio::Audio>,
}

impl BasicMetadata {
  /// Returns the first `image` object if array of `images` is not empty
  pub fn image(&self) -> Option<&Image> {
    if self.images.is_empty() {
      return None;
    }

    self.images.first()
  }

  /// Returns the first `video` object if array of `videos` is not empty
  pub fn video(&self) -> Option<&Video> {
    if self.videos.is_empty() {
      return None;
    }

    self.videos.first()
  }

  /// Returns the first `audio` object if array of `audios` is not empty
  pub fn audio(&self) -> Option<&Audio> {
    if self.audios.is_empty() {
      return None;
    }

    self.audios.first()
  }
}
