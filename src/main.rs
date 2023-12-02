// NOTE: This main entrypointer exists for dev purposes only
// and will be removed with the first release.

use ogp::{
  builder::MetadataBuilder,
  object_type::{article::Article, website::Website},
};

fn main() {
  let builder = MetadataBuilder::default();

  let _w = Article::default();

  let mut builder = builder.website();
  builder
    .set_url("https://github.com/ekkolon/ogp")
    .set_locale("DE-de")
    .set_title("Open Graph Protocol coming to Rust");

  println!("{}", serde_json::to_string_pretty(&builder).unwrap())
}
