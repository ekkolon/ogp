// TODO: Add docs

use crate::{builder::MetadataBuilder, object_type::ObjectType, Result};
use serde::{de::IntoDeserializer, Deserialize, Serialize};

pub trait Website {
  fn website(&self) -> MetadataBuilder;
}

impl Website for MetadataBuilder {
  fn website(&self) -> Self {
    self.clone()
  }
}
