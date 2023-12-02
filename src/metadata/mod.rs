//! Models for representing Open Graph data

use crate::object_type::ObjectType;
use serde::{Deserialize, Serialize};

pub use self::{audio::Audio, image::Image, video::Video};

mod audio;
mod image;
mod video;
