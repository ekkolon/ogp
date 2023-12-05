// NOTE: This main entrypoint exists for dev purposes only
// and will be removed with the first release.

use std::str::FromStr;

use chrono::{DateTime, Utc};
use ogp::{
  builder::MetadataBuilder, convert::ToHTML, metadata::Image,
  object_type::article::Article, Result,
};
use url::Url;

fn main() -> Result<()> {
  let date: DateTime<Utc> = chrono::offset::Utc::now();
  let published_time = &date.to_string();

  let mut base_builder = MetadataBuilder::default();

  base_builder
    .set_title("Open Graph Protocol coming to Rust")
    .set_url("https://github.com/ekkolon/ogp")
    .set_site_name("OGP")
    .set_locale("en_US")
    .add_locale_alternate("de_DE")
    .set_description(
      "The current Open Graph Protocol crate is very outdated. \
      That's why we're implementing this one.",
    )
    .add_image(Image {
      url: Some(
        Url::from_str("https://images.unsplash.com/photo-1683009427540")
          .unwrap(),
      ),
      alt: Some("A beautiful image".into()),
      ..Default::default()
    })
    .add_image(Image {
      url: Some(
        Url::from_str("https://images.unsplash.com/photo-1683009427540")
          .unwrap(),
      ),
      alt: Some("Another beautiful image".into()),
      ..Default::default()
    });

  let mut article_builder = base_builder.article();

  let article = article_builder
    .set_author("Nelson Dominguez")
    .set_published_time(published_time)
    .set_section("Technology")
    .add_tags(&[
      "Cross platform",
      "Editor",
      "Web",
      "Rich-content",
      "Developers",
    ]);

  let html = &article.to_html();

  println!(
    "ARTICLE:raw -> {}",
    serde_json::to_string_pretty(&article).unwrap()
  );

  println!(
    "ARTICLE:html -> {}",
    serde_json::to_string_pretty(&html).unwrap()
  );

  Ok(())
}
