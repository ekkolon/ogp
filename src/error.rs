//! This module defines custom error types used throughout the application.
//!
//! The `Error` enum provides variants for different error scenarios,
//! and each variant carries relevant information about the error.
//!
//! # Usage
//!
//! This module makes use of the `thiserror` crate for convenient error handling and derivation of the `Error` trait.
//!
//! ```rust
//! use ogp::error::Error;
//! use ogp::Result;
//!
//! fn process_data() -> Result<()> {
//!     // Your code that might result in an error
//!     // ...
//!     Ok(())
//! }
//!
//! fn main() {
//!     if let Err(err) = process_data() {
//!         eprintln!("Error: {}", err);
//!     }
//! }
//! ```
//!
//! # Errors
//!
//! The `Error` enum includes the following variants:
//!
//! - `Generic`: Represents a generic error with a custom message.
//! - `IO`: Represents an I/O error, transparently wrapping a `std::io::Error`.
//! - `InvalidHttpUrlScheme`: Represents an error for an invalid URL scheme, which must be one of 'http' or 'https'.
//! - `InvalidHttpsUrlScheme`: Represents an error for an invalid URL scheme in the context of HTTPS. The URL must start with 'https://'.
//! - `InvalidImageExtension`: Represents an error for an invalid image extension. Only 'png', 'jpg', 'jpeg', 'gif', or 'webp' images are allowed.
//! - `IncompleteImageDimensions`: Represents an error for incomplete visual object dimensions. Only one dimension value is found, but values for both 'width' and 'height' are required.
//!
//! # Dependencies
//!
//! - `thiserror`: Provides convenient derivation of the `Error` trait.
//!
//! See individual error variants for more details on each error scenario.
//!

/// Custom error type for representing different error scenarios.
#[derive(thiserror::Error, Debug)]
pub enum Error {
  /// Represents a generic error with a custom message.
  #[error("Generic {0}")]
  Generic(String),

  /// Represents an I/O error, transparently wrapping a `std::io::Error`.
  #[error(transparent)]
  IO(#[from] std::io::Error),

  /// Represents an error for an invalid URL scheme. It must be one of 'http' or 'https'.
  #[error("Failed to parse URL: {0}")]
  UrlParseError(String),

  /// Represents an error for an invalid URL scheme.
  /// It must be one of 'http' or 'https'.
  #[error("Invalid URL scheme '{0}'. Must be one of 'http'|'https'")]
  InvalidHttpUrlScheme(String),

  /// Represents an error for an invalid URL scheme in the context of HTTPS.
  /// The URL must start with 'https://'.
  #[error("Invalid URL scheme '{0}'. URL must start with 'https://'")]
  InvalidHttpsUrlScheme(String),

  /// Represents an error for an invalid image extension. Only 'png', 'jpg', 'jpeg', 'gif', or 'webp' images are allowed.
  #[error(
    "Invalid image extension '{0}'. \
        Only png|jpg|jpeg|gif|webp images are allowed."
  )]
  InvalidImageExtension(String),

  /// Represents an error for incomplete visual object dimensions.
  /// Only found value for the specified dimension, but values for both 'width' and 'height' are required.
  #[error(
    "Incomplete visual object dimensions. \
        Only found value for '{0}', but values for both \
        'width' and 'height' are required"
  )]
  IncompleteImageDimensions(&'static str),

  /// Represents an error for when an object is missing a property.
  #[error("Missing required property '{0}'")]
  MissingRequiredProperty(String),

  /// Represents an error for when an object is missing a property.
  #[error(
    "Locale '{0}' is invalid. \
    Must be in format `language_TERRITORY` (e.g. 'en_US')"
  )]
  InvalidLocale(String),

  /// Represents an error for when an object is missing a property.
  #[error(
    "Invalid Locale format '{0}'. \
    Must be in format `language_TERRITORY` (e.g. 'en_US')"
  )]
  InvalidLocaleFormat(String),

  /// Represents an error for when an object is missing a property.
  #[error(
    "Expected Locale value with length of 5 but actual length is '{0}'. \
      Must be in format `language_TERRITORY` (e.g. 'en_US')"
  )]
  InvalidLocaleLength(String),

  /// Represents an error for when an object is missing a property.
  #[error(
    "Locale '{0}' contains an invalid language code. \
    Language codes must be in ISO 639-1 format."
  )]
  InvalidLocaleLanguageCode(String),

  /// Represents an error for when an object is missing a property.
  #[error(
    "Locale '{0}' contains an invalid country code. \
    Country codes must be in ISO 3166 format."
  )]
  InvalidLocaleCountryCode(String),

  /// Represents an error for when an object is missing a property.
  #[error(
    "Locale is empty. \
    Must be in format `language_TERRITORY` (e.g. 'en_US')"
  )]
  EmptyLocale,
}
