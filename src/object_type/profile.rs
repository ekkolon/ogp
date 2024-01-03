//! Metadata utility for the Open Graph `profile` meta tag.

use crate::{
  builder::MetadataBuilder, convert::ToHTML, object_type::ObjectType, Result,
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
pub struct ProfileMetadataBuilder {
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
  metadata: MetadataBuilder,
}

impl ProfileMetadataBuilder {}

pub trait Profile {
  fn profile(&self) -> ProfileMetadataBuilder;
}

impl Profile for MetadataBuilder {
  fn profile(&self) -> ProfileMetadataBuilder {
    ProfileMetadataBuilder {
      metadata: MetadataBuilder {
        object_type: ObjectType::Profile,
        metadata: self.get().clone(),
      },
      ..Default::default()
    }
  }
}

impl ToHTML for ProfileMetadataBuilder {}
