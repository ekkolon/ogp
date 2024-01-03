// TODO: Add docs

use crate::metadata::{OgMetadata, OgMetadataBuilder};
use crate::{convert::ToHTML, object_type::ObjectType, Result};

use serde::{de::IntoDeserializer, Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct WebsiteMetadata {
  #[serde(flatten)]
  root: OgMetadata,
}

pub trait Website {
  fn website(&self) -> WebsiteMetadata;
}

impl Website for OgMetadataBuilder {
  fn website(&self) -> WebsiteMetadata {
    let root = OgMetadata {
      object_type: ObjectType::Website,
      ..self.get_metadata()
    };
    WebsiteMetadata { root }
  }
}

impl ToHTML for WebsiteMetadata {}
