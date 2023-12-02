use ogp::{
  builder::Metadata,
  object_type::{
    article::Article,
  },
};


fn main() {
  let _basic = Metadata::default();

  let w = Article::default();

  println!("{}", serde_json::to_string_pretty(&w).unwrap())
}
