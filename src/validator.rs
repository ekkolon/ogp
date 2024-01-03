//! Validators for different types of data.
//!
//! This module defines traits for validation purposes. Each trait represents a specific type of validation.
//!
//! # Examples
//!
//! ```rust
//! use ogp::validator::{Validatable, DimensionsValidator, SecureURLValidator};
//! use ogp::Result;
//!
//! struct MyData {
//!     // ... fields for your data
//! }
//!
//! impl Validatable for MyData {
//!     fn validate(&self) -> Result<()> {
//!         // Your validation logic for MyData
//!         // ...
//!         Ok(())
//!     }
//! }
//!
//! impl DimensionsValidator for MyData {
//!     fn width(&self) -> Option<u32> {
//!         // Return the width of your data
//!         Some(300)
//!     }
//!
//!     fn height(&self) -> Option<u32> {
//!         // Return the height of your data
//!         Some(300)
//!     }
//! }
//!
//! impl SecureURLValidator for MyData {
//!     fn secure_url(&self) -> Option<url::Url> {
//!         // Return the secure URL of your data
//!         None
//!     }
//! }
//! ```
//!
//! # Traits
//!
//! - `Validatable`: A trait for general validation.
//! - `DimensionsValidator`: A trait for validating dimensions, including width and height.
//! - `SecureURLValidator`: A trait for validating secure URLs.
//!
//! ## Validatable
//!
//! The `Validatable` trait provides a `validate` method that returns a `Result<()>` indicating whether the validation succeeded or failed.
//!
//! ## DimensionsValidator
//!
//! The `DimensionsValidator` trait provides methods for retrieving width and height, and a `validate_dimensions` method to ensure both dimensions are present when needed.
//!
//! ## SecureURLValidator
//!
//! The `SecureURLValidator` trait provides methods for retrieving a secure URL and a `validate_secure_url` method to ensure the URL uses the "https" scheme.
//!
//! # Usage
//!
//! ```rust
//! use ogp::validator::{DimensionsValidator, SecureURLValidator};
//!
//! struct MyData {
//!     // ... fields for your data
//! }
//!
//! impl DimensionsValidator for MyData {
//!     fn width(&self) -> Option<u32> {
//!         // Return the width of your data
//!         // ...
//!         Some(200)
//!     }
//!
//!     fn height(&self) -> Option<u32> {
//!         // Return the height of your data
//!         // ...
//!         Some(200)
//!     }
//! }
//!
//! impl SecureURLValidator for MyData {
//!     fn secure_url(&self) -> Option<url::Url> {
//!         // Return the secure URL of your data
//!         // ...
//!         None
//!     }
//! }
//! ```
//!

use crate::{error::Error, Result};

/// A trait for general validation.
pub trait Validatable {
  /// Validates the data and returns a `Result<()>` indicating success or failure.
  fn validate(&self) -> Result<()>;
}

/// A trait for validating dimensions, including width and height.
pub trait DimensionsValidator {
  /// Retrieves the width of the data.
  fn width(&self) -> Option<u32>;

  /// Retrieves the height of the data.
  fn height(&self) -> Option<u32>;

  /// Validates that both width and height are present when needed.
  fn validate_dimensions(&self) -> Result<()> {
    if self.height().is_some() && self.width().is_none() {
      return Err(Error::IncompleteImageDimensions("width"));
    } else if self.width().is_some() && self.height().is_none() {
      return Err(Error::IncompleteImageDimensions("height"));
    }

    Ok(())
  }
}

/// A trait for validating secure URLs.
pub trait SecureURLValidator {
  /// Retrieves the secure URL of the data.
  fn secure_url(&self) -> Option<url::Url>;

  /// Validates that the URL uses the "https" scheme.
  fn validate_secure_url(&self) -> Result<()> {
    match self.secure_url() {
      None => Ok(()),
      Some(url) => {
        let scheme = url.scheme();
        if scheme == "https" {
          return Ok(());
        }

        let err = Error::InvalidHttpsUrlScheme(scheme.into());
        Err(err)
      }
    }
  }
}
