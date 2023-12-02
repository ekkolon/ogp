// TODO: Add docs

use crate::{
  builder::{MetadataBuilder, ObjectMetadata},
  object_type::ObjectType,
  Result,
};
use serde::{de::IntoDeserializer, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Website {
  #[serde(flatten)]
  metadata: ObjectMetadata,
}

impl Website {
  pub fn new() -> Self {
    Website {
      metadata: MetadataBuilder::with_type(ObjectType::Website),
    }
  }
}
