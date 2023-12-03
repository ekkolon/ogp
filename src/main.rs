// NOTE: This main entrypointer exists for dev purposes only
// and will be removed with the first release.

use ogp::{builder::MetadataBuilder, object_type::website::Website};

fn main() {
  let builder = MetadataBuilder::default();

  let mut website_builder = builder.website();

  website_builder
    .set_title("Open Graph Protocol coming to Rust")
    .set_url("https://github.com/ekkolon/ogp")
    .set_site_name("OGP")
    .set_locale("en_US")
    .set_description(
      "The current Open Graph Protocol crate is very outdated. \
      That's why we're implementing this one.",
    );

  println!(
    "{}",
    serde_json::to_string_pretty(&website_builder).unwrap()
  )
}
