//! Metadata utility for the Open Graph `article` meta tag.

use crate::{
  builder::{Metadata, MetadataBuilder},
  error::Error,
  object_type::ObjectType,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Article {
  /// When the article was first published.
  #[serde(rename = "article:published_time")]
  pub published_time: Option<String>,

  /// When the article was last changed.
  #[serde(rename = "article:modified_time")]
  pub modified_time: Option<String>,

  /// When the article is out of date after.
  #[serde(rename = "article:expiration_time")]
  pub expiration_time: Option<String>,

  /// Writers of the article.
  #[serde(rename = "article:author")]
  pub author: Option<String>,

  /// A high-level section name. E.g. Technology
  #[serde(rename = "article:section")]
  pub section: Option<String>,

  /// Tag words associated with this article.
  #[serde(rename = "article:tag")]
  pub tag: Option<String>,

  #[serde(flatten)]
  metadata: MetadataBuilder,
}

impl Article {
  pub fn new() -> Self {
    Article {
      metadata: MetadataBuilder::with_type(ObjectType::Article),
      ..Default::default()
    }
  }
}
