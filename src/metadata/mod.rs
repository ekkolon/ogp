//! Models for representing Open Graph data

use crate::object_type::ObjectType;
use serde::{Deserialize, Serialize};

use self::{audio::Audio, image::Image, video::Video};

pub mod audio;
pub mod image;
pub mod video;
