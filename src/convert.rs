use serde::Serialize;
use serde_json::{json, to_string_pretty, Value};

use crate::object_type::website::Website;

macro_rules! meta_tag {
  ($name:expr, $content:expr) => {
    format!(r#"<meta property="{}" content="{}" />"#, $name, $content)
  };
}

pub trait ToHTML
where
  Self: Serialize,
{
  fn to_html(&self) -> Vec<String> {
    let serialized = json!(self);

    fn process_json(value: &Value, prefix: &str) -> Vec<String> {
      match value {
        Value::Object(obj) => {
          let mut result = Vec::new();
          for (key, value) in obj {
            let new_prefix = if prefix.is_empty() {
              key.to_string()
            } else {
              format!("{}:{}", prefix, key)
            };
            result.extend(process_json(value, &new_prefix));
          }
          result
        }
        Value::Array(arr) => {
          let mut result = Vec::new();
          for (index, value) in arr.iter().enumerate() {
            let new_prefix = format!("{}{}", prefix, "");
            result.extend(process_json(value, &new_prefix));
          }
          result
        }
        Value::String(s) => vec![meta_tag!(prefix, s)],
        _ => vec![], // Ignore other types for simplicity
      }
    }

    process_json(&serialized, "")
  }
}
