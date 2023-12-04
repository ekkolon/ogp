//! Metadata utility for the Open Graph `article` meta tag.

use std::{ops::Add, str::FromStr};

use crate::{
  builder::{Metadata, MetadataBuilder},
  error::Error,
  object_type::ObjectType,
};

use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ArticleMetadataBuilder {
  /// When the article was first published.
  #[serde(rename = "article:published_time", with = "ts_seconds_option")]
  published_time: Option<DateTime<Utc>>,

  /// When the article was last changed.
  #[serde(rename = "article:modified_time", with = "ts_seconds_option")]
  modified_time: Option<DateTime<Utc>>,

  /// When the article is out of date after.
  #[serde(rename = "article:expiration_time", with = "ts_seconds_option")]
  expiration_time: Option<DateTime<Utc>>,

  /// Writers of the article.
  #[serde(rename = "article:author")]
  author: Option<String>,

  /// A high-level section name. E.g. Technology
  #[serde(rename = "article:section")]
  section: Option<String>,

  /// Tag words associated with this article.
  tags: Vec<String>,

  #[serde(flatten)]
  metadata: MetadataBuilder,
}

impl ArticleMetadataBuilder {
  pub fn set_published_time(
    &mut self,
    published_time: impl Into<String>,
  ) -> &mut Self {
    match DateTime::<Utc>::from_str(&published_time.into()) {
      Err(err) => panic!("error: {}", err),
      Ok(date) => {
        println!("{}", date);
        self.published_time.insert(date);
      }
    };

    self
  }

  pub fn set_modified_time(
    &mut self,
    modified_time: impl Into<String>,
  ) -> &mut Self {
    self.set_date_from_str(modified_time, &self.modified_time);
    self
  }

  pub fn set_expiration_time(
    &mut self,
    expiration_time: impl Into<String>,
  ) -> &mut Self {
    self.set_date_from_str(expiration_time, &self.expiration_time);
    self
  }

  pub fn set_author(&mut self, author: impl Into<String>) -> &mut Self {
    self.author.insert(author.into());
    self
  }

  pub fn set_section(&mut self, section: impl Into<String>) -> &mut Self {
    self.section.insert(section.into());
    self
  }

  pub fn add_tags(&mut self, new_tags: &[&str]) -> &mut Self {
    for tag in new_tags {
      let t: String = tag.to_owned().into();
      self.tags.push(t);
    }

    self
  }

  pub fn add_tag(&mut self, tag: impl Into<String>) -> &mut Self {
    self.tags.push(tag.into());
    self
  }

  fn set_date_from_str(
    &self,
    date: impl Into<String>,
    mut prop: &Option<DateTime<Utc>>,
  ) {
    match DateTime::<Utc>::from_str(&date.into()) {
      Err(err) => panic!("error: {}", err),
      Ok(date) => {
        println!("{}", date);
        prop.as_ref().insert(&date);
      }
    }
  }
}

pub trait Article {
  fn article(&self) -> ArticleMetadataBuilder;
}

impl Article for MetadataBuilder {
  fn article(&self) -> ArticleMetadataBuilder {
    ArticleMetadataBuilder {
      metadata: MetadataBuilder {
        object_type: ObjectType::Article,
        metadata: self.get().clone(),
      },
      ..Default::default()
    }
  }
}
