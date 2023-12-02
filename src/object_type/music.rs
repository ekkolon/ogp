//! Metadata utility for the Open Graph `music` meta tag.

use crate::{
  builder::{MetadataBuilder, ObjectMetadata},
  object_type::ObjectType,
  Result,
};
use serde::{de::IntoDeserializer, Deserialize, Serialize};

// TODO: Add missing props
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MusicAlbum {
  #[serde(flatten)]
  metadata: ObjectMetadata,
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
  metadata: ObjectMetadata,
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
  metadata: ObjectMetadata,
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
  metadata: ObjectMetadata,
}

impl MusicRadioStation {
  pub fn new() -> Self {
    MusicRadioStation {
      metadata: MetadataBuilder::with_type(ObjectType::MusicRadioStation),
    }
  }
}
