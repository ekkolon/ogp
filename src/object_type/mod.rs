//! Object types for representing entities within a graph.
//!
//! # Enums
//!
//! - `ObjectType`: The main enum representing various object types in the graph.
//!
//! ## ObjectType Variants
//!
//! - `MusicSong`: Represents a song in the music category.
//! - `MusicAlbum`: Represents a music album.
//! - `MusicPlaylist`: Represents a music playlist.
//! - `MusicRadioStation`: Represents a radio station in the music category.
//! - `VideoMovie`: Represents a movie in the video category.
//! - `VideoEpisode`: Represents an episode of a TV show in the video category.
//! - `VideoTvShow`: Represents a TV show in the video category.
//! - `VideoOther`: Represents miscellaneous video content.
//! - `Article`: Represents an article.
//! - `Book`: Represents a book.
//! - `Profile`: Represents a user profile.
//! - `Website`: Represents a website.
//!
//! # Usage
//!
//! ```rust
//! use ogp::object_type::ObjectType;
//!
//! let music_song = ObjectType::MusicSong;
//! let video_movie = ObjectType::VideoMovie;
//! let article = ObjectType::Article;
//!
//! ```

use serde::{Deserialize, Serialize};

pub mod article;
pub mod book;
pub mod music;
pub mod profile;
pub mod video;
pub mod website;

/// The type of object in the graph this refers to.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub enum ObjectType {
  /// Represents a song in the music category.
  #[serde(rename = "music.song")]
  MusicSong,

  /// Represents a music album.
  #[serde(rename = "music.album")]
  MusicAlbum,

  /// Represents a music playlist.
  #[serde(rename = "music.playlist")]
  MusicPlaylist,

  /// Represents a radio station in the music category.
  #[serde(rename = "music.radio_station")]
  MusicRadioStation,

  /// Represents a movie in the video category.
  #[serde(rename = "video.movie")]
  VideoMovie,

  /// Represents an episode of a TV show in the video category.
  #[serde(rename = "video.episode")]
  VideoEpisode,

  /// Represents a TV show in the video category.
  #[serde(rename = "video.tv_show")]
  VideoTvShow,

  /// Represents miscellaneous video content.
  #[serde(rename = "video.other")]
  VideoOther,

  /// Represents an article.
  #[serde(rename = "article")]
  Article,

  /// Represents a book.
  #[serde(rename = "book")]
  Book,

  /// Represents a user profile.
  #[serde(rename = "profile")]
  Profile,

  /// Represents a website.
  #[default]
  #[serde(rename = "website")]
  Website,
}

impl ObjectType {
  /// Performs a value conversion from any value that implements the `Into` trait
  /// to an Open Graph object type.
  ///
  /// ## Returns
  ///
  /// The transformed string value as `ObjectType`.
  ///
  /// Falls back to `ObjectType::Website`, if provided value does not match any valid string.
  pub fn from_string(value: impl Into<String>) -> ObjectType {
    let obj_type: &str = &value.into();

    match obj_type {
      "article" => ObjectType::Article,
      "book" => ObjectType::Book,
      "profile" => ObjectType::Profile,
      "website" => ObjectType::Website,
      "music.song" => ObjectType::MusicSong,
      "music.album" => ObjectType::MusicAlbum,
      "music.playlist" => ObjectType::MusicPlaylist,
      "music.radio_station" => ObjectType::MusicRadioStation,
      "video.movie" => ObjectType::VideoMovie,
      "video.episode" => ObjectType::VideoEpisode,
      "video.tv_show" => ObjectType::VideoTvShow,
      "video.other" => ObjectType::VideoOther,
      _ => ObjectType::Website,
    }
  }
}

/// Enum representing the word that appears before an Open Graph object's title in a sentence.
/// If auto is chosen, the consumer of your data should chose between "a" or "an".
///
/// Default is "" (blank).
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub enum Determiner {
  #[serde(rename = "a")]
  A,

  #[serde(rename = "an")]
  An,

  #[serde(rename = "the")]
  The,

  #[default]
  #[serde(rename = "")]
  Blank,

  #[serde(rename = "auto")]
  Auto,
}

impl Determiner {
  pub fn from_string(value: impl Into<String>) -> Determiner {
    let determ: &str = &value.into();
    match determ {
      "a" => Determiner::A,
      "an" => Determiner::An,
      "the" => Determiner::The,
      "auto" => Determiner::Auto,
      _ => Determiner::Blank,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // region    ObjectType
  // ========================================
  #[test]
  fn object_type_website_conversion() {
    // Should fall back to ObjectType::Website when invalid value is provided
    assert_eq!(ObjectType::from_string("podcast"), ObjectType::Website);

    assert_eq!(ObjectType::from_string("website"), ObjectType::Website);
  }

  #[test]
  fn object_type_article_conversion() {
    assert_eq!(ObjectType::from_string("article"), ObjectType::Article)
  }

  #[test]
  fn object_type_music_conversion() {
    assert_eq!(
      ObjectType::from_string("music.album"),
      ObjectType::MusicAlbum
    );
    assert_eq!(
      ObjectType::from_string("music.playlist"),
      ObjectType::MusicPlaylist
    );
    assert_eq!(
      ObjectType::from_string("music.radio_station"),
      ObjectType::MusicRadioStation
    );
    assert_eq!(ObjectType::from_string("music.song"), ObjectType::MusicSong);
  }

  #[test]
  fn object_type_video_conversion() {
    assert_eq!(
      ObjectType::from_string("video.episode"),
      ObjectType::VideoEpisode
    );
    assert_eq!(
      ObjectType::from_string("video.movie"),
      ObjectType::VideoMovie
    );
    assert_eq!(
      ObjectType::from_string("video.other"),
      ObjectType::VideoOther
    );
    assert_eq!(
      ObjectType::from_string("video.tv_show"),
      ObjectType::VideoTvShow
    );
  }
  // ========================================
  // endregion ObjectType

  // region    Determiner
  #[test]
  fn blank_determiner_if_nomatch() {
    assert_eq!(Determiner::from_string("that"), Determiner::Blank)
  }

  #[test]
  fn literal_the_determiner() {
    assert_eq!(Determiner::from_string("the"), Determiner::The)
  }

  #[test]
  fn literal_an_determiner() {
    assert_eq!(Determiner::from_string("an"), Determiner::An)
  }

  #[test]
  fn literal_a_determiner() {
    assert_eq!(Determiner::from_string("a"), Determiner::A)
  }

  #[test]
  fn auto_determiner() {
    assert_eq!(Determiner::from_string("auto"), Determiner::Auto)
  }
  // endregion Determiner
}
