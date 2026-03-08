pub mod builder;
pub mod enums;

pub use builder::Builder;
pub use enums::*;

use serde::{Deserialize, Serialize};

/// Upscaler params wrapper
/// This struct holds the the fields relative to Upscaler parameters in gamescope.
/// Such as `--scaler`, `--filter` and `--sharpness`
// NOTE: I made it a container of optional fields
// In the gamescope cli, these fields can be omitted because they have default values
// So by making them optional, we can construct a default holder of those settings without needing to specify all fields
// Like we would do with the CLI
// The upscaler settings struct holds optional fields so no need to make it optional itself (i think)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Upscaler {
    pub scaler_type: Option<self::Type>, // if not specified, defaults to Auto in the cli
    pub filter: Option<self::Filter>,
    pub settings: self::Settings,
}

impl Default for Upscaler {
    fn default() -> Self {
        Self {
            scaler_type: None,
            filter: None,
            settings: Settings::default(),
        }
    }
}

pub fn is_default_upscaler(upscaler: &Upscaler) -> bool {
    upscaler.scaler_type.is_none()
        && upscaler.filter.is_none()
        && upscaler.settings == Settings::default()
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(default)]
pub struct Settings {
    pub sharpness: Option<u32>,
    pub scale: Option<f32>,
}
impl Settings {
    pub fn calculate_res(&self, (w, h): (u32, u32)) -> (u32, u32) {
        let s = self.scale.unwrap_or(1.0);
        ((w as f32 * s) as u32, (h as f32 * s) as u32)
    }
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            sharpness: None,
            scale: None,
        }
    }
}
