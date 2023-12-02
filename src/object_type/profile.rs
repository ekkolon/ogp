//! Metadata utility for the Open Graph `profile` meta tag.

use crate::{
  builder::{Metadata, MetadataBuilder, ObjectMetadata},
  error::Error,
  metadata::{audio, image, video},
  object_type::ObjectType,
  Result,
};
use serde::{de::IntoDeserializer, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Gender {
  #[serde(rename = "male")]
  Male,

  #[serde(rename = "female")]
  Female,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Profile {
  /// A name normally given to an individual by a parent or self-chosen.
  #[serde(rename = "profile:first_name")]
  pub first_name: Option<String>,

  /// A name inherited from a family or marriage and by which the
  /// individual is commonly known.
  #[serde(rename = "profile:last_name")]
  pub last_name: Option<String>,

  /// A short unique string to identify them.
  #[serde(rename = "profile:username")]
  pub username: Option<String>,

  /// Their gender.
  #[serde(rename = "profile:gender")]
  pub gender: Option<Gender>,

  #[serde(flatten)]
  metadata: ObjectMetadata,
}

impl Profile {
  pub fn new() -> Self {
    Profile {
      metadata: MetadataBuilder::with_type(ObjectType::Profile),
      ..Default::default()
    }
  }
}
