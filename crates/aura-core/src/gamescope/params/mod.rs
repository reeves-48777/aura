pub mod builder;

pub use builder::Builder;

use super::{
    DisplayMode, RenderingBackend,
    upscaler::{Upscaler, is_default_upscaler},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Params {
    // monitor settings
    pub output_res: (u32, u32),
    pub nested_res: (u32, u32),
    pub nested_refresh: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_scale: Option<f32>,

    #[serde(default, skip_serializing_if = "is_default_upscaler")]
    pub upscaler: Upscaler,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mouse_sensitivity: Option<f32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend: Option<RenderingBackend>,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub mangoapp: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub adaptive_sync: bool,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub immediate_flips: bool,

    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub force_grab_cursor: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested_display: Option<DisplayMode>,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            output_res: (0, 0),
            nested_res: (0, 0),
            nested_refresh: 0,
            max_scale: None,
            upscaler: Upscaler::default(),
            mouse_sensitivity: None,
            backend: None,
            mangoapp: true,
            adaptive_sync: true,
            immediate_flips: false,
            force_grab_cursor: true,
            nested_display: Some(DisplayMode::Fullscreen),
        }
    }
}

use display_info::DisplayInfo;
impl Params {
    pub fn resolve_screen(&self) -> ((u32, u32), (u32, u32), u32) {
        let displays = DisplayInfo::all().unwrap_or_default();

        let display_info = displays
            .iter()
            .find(|d| d.is_primary)
            .or_else(|| displays.first());

        let output = if self.output_res != (0, 0) {
            self.output_res
        } else {
            display_info
                .as_ref()
                .map(|d| (d.width, d.height))
                .unwrap_or((1920, 1080))
        };

        let nested = if self.nested_res != (0, 0) {
            self.nested_res
        } else {
            self.upscaler
                .settings
                .scale
                .as_ref()
                .map(|s| ((output.0 as f32 * s) as u32, (output.1 as f32 * s) as u32))
                .unwrap_or(output)
        };

        let refresh = if self.nested_refresh != 0 {
            self.nested_refresh
        } else {
            display_info
                .as_ref()
                .map(|d| d.frequency as u32)
                .unwrap_or(60)
        };

        (output, nested, refresh)
    }
}
