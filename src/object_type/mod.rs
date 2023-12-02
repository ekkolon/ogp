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

/// The type of object in the graph this refers to.
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
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
