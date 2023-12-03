//! Utility functions for handling media file extensions.
//!
//! This module provides a function for checking whether a given filename has a valid image extension.
//! It also includes tests to ensure the correctness of the `is_valid_image_ext` function.
//!
//! # Examples
//!
//! ```ignore
//! use ogp::utils::is_valid_image_ext;
//!
//! fn main() {
//!     let filename = "image.jpg";
//!     if is_valid_image_ext(filename) {
//!         println!("Valid image extension");
//!     } else {
//!         println!("Invalid image extension");
//!     }
//! }
//! ```
//!
//! # Functions
//!
//! - `is_valid_image_ext`: Checks if a given filename has a valid image extension based on a predefined set of allowed extensions.
//!
//! # Constants
//!
//! - `ALLOWED_MEDIA_FILE_EXT`: A constant array containing the allowed image file extensions.
//!
//! # Tests
//!
//! The module includes tests for the `is_valid_image_ext` function to ensure its correctness.
//! - `valid_image_extension`: Tests if a valid image extension ("jpg") returns true.
//! - `valid_image_extension_lowercase`: Tests if a valid image extension in lowercase ("jpeg") returns true.
//! - `invalid_image_extension`: Tests if an invalid image extension ("pdf") returns false.
//! - `no_extension`: Tests if a filename without an extension returns false.
//! - `case_sensitive_extension`: Tests if the function is case-sensitive and returns true for a file with an uppercase extension ("JPG").
//!
//! # Usage
//!
//! ```ignore
//! use ogp::utils::is_valid_image_ext;
//!
//! fn main() {
//!     let filename = "image.jpg";
//!     if is_valid_image_ext(filename) {
//!         println!("Valid image extension");
//!     } else {
//!         println!("Invalid image extension");
//!     }
//! }
//! ```
//!
//! Adjust the `ALLOWED_MEDIA_FILE_EXT` array to modify the set of allowed image extensions.
//!

#![allow(dead_code)]

use isocountry::CountryCode as Country;
use isolang::Language;
use regex::Regex;
use crate::{error::Error, Result};

/// A constant array containing the allowed image file extensions.
pub const ALLOWED_MEDIA_FILE_EXT: [&str; 5] =
  ["png", "jpg", "jpeg", "gif", "webp"];

/// Checks if a given filename has a valid image extension based on a predefined set of allowed extensions.
///
/// # Arguments
///
/// * `filename` - A string representing the filename to be checked.
///
/// # Returns
///
/// Returns `true` if the filename has a valid image extension, otherwise `false`.
pub fn is_valid_image_ext(filename: &str) -> bool {
  let parts: Vec<&str> = filename.split('.').collect();
  let last_part = parts.last();
  if last_part.is_none() {
    return false;
  }

  let ext = last_part.unwrap().to_lowercase();
  ALLOWED_MEDIA_FILE_EXT.contains(&ext.as_str())
}

/// Check if a given string value represents a valid locale
/// in the format `language_TERRITORY` (e.g "en_US").
///
/// # Arguments
///
/// * `locale` - A string representing the locale to be checked.
///
/// # Returns
///
/// Returns `true` if the locale is in the correct format, otherwise `false`.
pub fn validate_locale(locale: &str) -> Result<()> {
  if locale.is_empty() {
    return Err(Error::EmptyLocale);
  }

  let chars = locale.chars();
  let num_chars = chars.count();
  if num_chars != 5 {
    return Err(Error::InvalidLocaleLength(num_chars.to_string()));
  }

  if !is_valid_locale_format(locale) {
    return Err(Error::InvalidLocaleFormat(locale.into()));
  }

  let parts: Vec<&str> = locale.split('_').collect();
  let lang = parts[0];
  let country = parts[1];

  let Some(_) = Language::from_639_1(lang) else {
    return Err(Error::InvalidLocaleLanguageCode(lang.into()));
  };

  let Some(_) = Country::for_alpha2(lang).err() else {
    return Err(Error::InvalidLocaleCountryCode(lang.into()));
  };

  Ok(())
}

fn is_valid_locale_format(locale: &str) -> bool {
  let regex_pattern = r"^[a-zA-Z]{2}_[a-zA-Z]{2}$";
  let regex = Regex::new(regex_pattern).unwrap();
  regex.is_match(locale)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn valid_image_extension() {
    let filename = "image.jpg";
    assert!(is_valid_image_ext(filename));
  }

  #[test]
  fn valid_image_extension_lowercase() {
    let filename = "image.jpeg";
    assert!(is_valid_image_ext(filename));
  }

  #[test]
  fn invalid_image_extension() {
    let filename = "document.pdf";
    assert!(!is_valid_image_ext(filename));
  }

  #[test]
  fn no_extension() {
    let filename = "file_without_extension";
    assert!(!is_valid_image_ext(filename));
  }

  #[test]
  fn case_sensitive_extension() {
    let filename = "image.JPG";
    assert!(is_valid_image_ext(filename));
  }

  // region    validate_locale
  #[test]
  fn valid_locale() {
    assert!(validate_locale("en_US").is_ok());
    assert!(validate_locale("de_DE").is_ok());
    assert!(validate_locale("fr_FR").is_ok());
  }

  #[test]
  fn empty_locale() {
    assert!(validate_locale("").is_err());
  }

  #[test]
  fn invalid_locale_length() {
    assert!(validate_locale("en_USA").is_err());
    assert!(validate_locale("enU").is_err());
  }

  #[test]
  fn invalid_locale_language_code() {
    assert!(validate_locale("eng_US").is_err());
    assert!(validate_locale("_US").is_err());
  }

  #[test]
  fn invalid_locale_country_code() {
    assert!(validate_locale("en_USA").is_err());
    assert!(validate_locale("en_").is_err());
  }
  // endregion validate_locale
}
