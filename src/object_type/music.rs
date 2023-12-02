//! Metadata utility for the Open Graph `music` meta tag.

use crate::{builder::MetadataBuilder, object_type::ObjectType, Result};
use serde::{de::IntoDeserializer, Deserialize, Serialize};

// TODO: Add missing props
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MusicAlbum {
  #[serde(flatten)]
  metadata: MetadataBuilder,
}

impl MusicAlbum {
  pub fn new() -> Self {
    MusicAlbum {
      metadata: MetadataBuilder::with_type(ObjectType::MusicAlbum),
    }
  }
}

// TODO: Add missing props
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MusicSong {
  #[serde(flatten)]
  metadata: MetadataBuilder,
}

impl MusicSong {
  pub fn new() -> Self {
    MusicSong {
      metadata: MetadataBuilder::with_type(ObjectType::MusicSong),
    }
  }
}

// TODO: Add missing props
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MusicPlaylist {
  #[serde(flatten)]
  metadata: MetadataBuilder,
}

impl MusicPlaylist {
  pub fn new() -> Self {
    MusicPlaylist {
      metadata: MetadataBuilder::with_type(ObjectType::MusicPlaylist),
    }
  }
}

// TODO: Add missing props
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MusicRadioStation {
  #[serde(flatten)]
  metadata: MetadataBuilder,
}

impl MusicRadioStation {
  pub fn new() -> Self {
    MusicRadioStation {
      metadata: MetadataBuilder::with_type(ObjectType::MusicRadioStation),
    }
  }
}
