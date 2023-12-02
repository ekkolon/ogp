//! Metadata utility for the Open Graph `book` meta tag.

use crate::{builder::MetadataBuilder, object_type::ObjectType, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Book {
  /// Who wrote this book.
  #[serde(rename = "book:author")]
  pub author: Option<String>,

  /// The ISBN.
  #[serde(rename = "book:isbn")]
  pub isbn: Option<String>,

  /// The date the book was released.
  #[serde(rename = "book:release_date ")]
  pub release_date: Option<String>,

  /// Tag words associated with this book.
  #[serde(rename = "book:tag")]
  pub tag: Option<String>,

  #[serde(flatten)]
  metadata: MetadataBuilder,
}

impl Book {
  pub fn new() -> Self {
    Book {
      metadata: MetadataBuilder::with_type(ObjectType::Book),
      ..Default::default()
    }
  }
}
