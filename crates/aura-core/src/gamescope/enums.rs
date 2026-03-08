use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum RenderingBackend {
    #[default]
    Auto,
    Drm,
    Sdl,
    OpenVR,
    Headless,
    Wayland,
}
impl fmt::Display for RenderingBackend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Auto => "auto",
            Self::Drm => "drm",
            Self::Sdl => "sdl",
            Self::OpenVR => "openvr",
            Self::Headless => "headless",
            Self::Wayland => "wayland",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DisplayMode {
    Borderless,
    Fullscreen,
}

impl fmt::Display for DisplayMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Borderless => "-b",
            Self::Fullscreen => "-f",
        };
        write!(f, "{}", s)
    }
}
