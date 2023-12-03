//! Metadata utility for the Open Graph `music` meta tag.

use crate::{builder::MetadataBuilder, object_type::ObjectType, Result};
use serde::{Deserialize, Serialize};

// TODO: Add missing props
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct VideoMovie {
  #[serde(flatten)]
  metadata: MetadataBuilder,
}

impl VideoMovie {
  pub fn new() -> Self {
    VideoMovie {
      metadata: MetadataBuilder::with_type(ObjectType::VideoMovie),
    }
  }
}

// TODO: Add missing props
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct VideoEpisode {
  #[serde(flatten)]
  metadata: MetadataBuilder,
}

impl VideoEpisode {
  pub fn new() -> Self {
    VideoEpisode {
      metadata: MetadataBuilder::with_type(ObjectType::VideoEpisode),
    }
  }
}

// TODO: Add missing props
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct VideoTvShow {
  #[serde(flatten)]
  metadata: MetadataBuilder,
}

impl VideoTvShow {
  pub fn new() -> Self {
    VideoTvShow {
      metadata: MetadataBuilder::with_type(ObjectType::VideoTvShow),
    }
  }
}

// TODO: Add missing props
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct VideoOther {
  #[serde(flatten)]
  metadata: MetadataBuilder,
}

// TODO: Add missing props
impl VideoOther {
  pub fn new() -> Self {
    VideoOther {
      metadata: MetadataBuilder::with_type(ObjectType::VideoOther),
    }
  }
}
