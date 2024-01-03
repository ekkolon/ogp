//! Metadata utility for the Open Graph `music` meta tag.

use crate::metadata::{OgMetadata, OgMetadataBuilder};
use crate::{object_type::ObjectType, Result};
use serde::{de::IntoDeserializer, Deserialize, Serialize};

// TODO: Add missing props
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MusicAlbum {
  #[serde(flatten)]
  metadata: OgMetadataBuilder,
}

impl MusicAlbum {
  pub fn new() -> Self {
    MusicAlbum {
      metadata: OgMetadataBuilder::with_type(ObjectType::MusicAlbum),
    }
  }
}

// TODO: Add missing props
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MusicSong {
  #[serde(flatten)]
  metadata: OgMetadataBuilder,
}

impl MusicSong {
  pub fn new() -> Self {
    MusicSong {
      metadata: OgMetadataBuilder::with_type(ObjectType::MusicSong),
    }
  }
}

// TODO: Add missing props
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MusicPlaylist {
  #[serde(flatten)]
  metadata: OgMetadataBuilder,
}

impl MusicPlaylist {
  pub fn new() -> Self {
    MusicPlaylist {
      metadata: OgMetadataBuilder::with_type(ObjectType::MusicPlaylist),
    }
  }
}

// TODO: Add missing props
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct MusicRadioStation {
  #[serde(flatten)]
  metadata: OgMetadataBuilder,
}

impl MusicRadioStation {
  pub fn new() -> Self {
    MusicRadioStation {
      metadata: OgMetadataBuilder::with_type(ObjectType::MusicRadioStation),
    }
  }
}
