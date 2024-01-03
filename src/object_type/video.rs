//! Metadata utility for the Open Graph `music` meta tag.

use crate::metadata::{OgMetadata, OgMetadataBuilder};
use crate::{object_type::ObjectType, Result};
use serde::{Deserialize, Serialize};

// TODO: Add missing props
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct VideoMovie {
  #[serde(flatten)]
  metadata: OgMetadataBuilder,
}

impl VideoMovie {
  pub fn new() -> Self {
    VideoMovie {
      metadata: OgMetadataBuilder::with_type(ObjectType::VideoMovie),
    }
  }
}

// TODO: Add missing props
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct VideoEpisode {
  #[serde(flatten)]
  metadata: OgMetadataBuilder,
}

impl VideoEpisode {
  pub fn new() -> Self {
    VideoEpisode {
      metadata: OgMetadataBuilder::with_type(ObjectType::VideoEpisode),
    }
  }
}

// TODO: Add missing props
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct VideoTvShow {
  #[serde(flatten)]
  metadata: OgMetadataBuilder,
}

impl VideoTvShow {
  pub fn new() -> Self {
    VideoTvShow {
      metadata: OgMetadataBuilder::with_type(ObjectType::VideoTvShow),
    }
  }
}

// TODO: Add missing props
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct VideoOther {
  #[serde(flatten)]
  metadata: OgMetadataBuilder,
}

// TODO: Add missing props
impl VideoOther {
  pub fn new() -> Self {
    VideoOther {
      metadata: OgMetadataBuilder::with_type(ObjectType::VideoOther),
    }
  }
}
