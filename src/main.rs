// NOTE: This main entrypoint exists for dev purposes only
// and will be removed with the first release.

use chrono::{DateTime, Utc};
use ogp::{builder::MetadataBuilder, object_type::article::Article};

fn main() {
  let date: DateTime<Utc> = chrono::offset::Utc::now();
  let published_time = &date.to_string();

  let mut base_builder = MetadataBuilder::default();

  base_builder
    .set_title("Open Graph Protocol coming to Rust")
    .set_url("https://github.com/ekkolon/ogp")
    .set_site_name("OGP")
    .set_locale("en_US")
    .set_description(
      "The current Open Graph Protocol crate is very outdated. \
      That's why we're implementing this one.",
    )
    .add_image_url("https://images.unsplash.com/photo-1683009427540-c5bd6a32abf6?q=80&w=2070&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDF8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D");

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

  println!("{}", serde_json::to_string_pretty(&article).unwrap())
}
