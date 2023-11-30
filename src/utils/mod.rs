//! Utility functions for handling media file extensions.
//!
//! This module provides a function for checking whether a given filename has a valid image extension.
//! It also includes tests to ensure the correctness of the `is_valid_image_ext` function.
//!
//! # Examples
//!
//! ```rust
//! use crate::utils::is_valid_image_ext;
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
//! ```rust
//! use crate::utils::is_valid_image_ext;
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

/// A constant array containing the allowed image file extensions.
pub const ALLOWED_MEDIA_FILE_EXT: [&str; 5] = ["png", "jpg", "jpeg", "gif", "webp"];

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
}
