// TODO: Add docs

use crate::{
  builder::MetadataBuilder, convert::ToHTML, object_type::ObjectType, Result,
};
use serde::{de::IntoDeserializer, Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct WebsiteMetadataBuilder {
  #[serde(flatten)]
  metadata: MetadataBuilder,
}

pub trait Website {
  fn website(&self) -> WebsiteMetadataBuilder;
}

impl Website for MetadataBuilder {
  fn website(&self) -> WebsiteMetadataBuilder {
    WebsiteMetadataBuilder {
      metadata: MetadataBuilder {
        object_type: ObjectType::Website,
        metadata: self.get().clone(),
      },
      ..Default::default()
    }
  }
}

impl ToHTML for WebsiteMetadataBuilder {}
