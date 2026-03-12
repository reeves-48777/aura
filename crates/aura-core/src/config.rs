use directories::ProjectDirs;
use display_info::DisplayInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GlobalConfig {
    pub known_displays: HashMap<String, DisplaySettings>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DisplaySettings {
    pub name: String,
    pub resolution: (u32, u32),
    pub refresh_rate: u32,
    pub hdr: bool,
}

impl DisplaySettings {
    pub const DEFAULT_WIDTH: u32 = 1920;
    pub const DEFAULT_HEIGHT: u32 = 1080;
    pub const DEFAULT_FPS: u32 = 60;
}

impl GlobalConfig {
    pub fn get_config_path() -> anyhow::Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "aura", "aura")
            .ok_or_else(|| anyhow::anyhow!("Cannot find config folder"))?;

        let path = proj_dirs.config_dir().to_path_buf();

        if !path.exists() {
            fs::create_dir_all(&path)?;
        }

        Ok(path)
    }

    pub fn load() -> anyhow::Result<Self> {
        let mut path = Self::get_config_path()?;
        path.push("config.ron");

        if !path.exists() {
            let default_conf = Self::default();
            default_conf.save()?;
            return Ok(default_conf);
        }

        let content = fs::read_to_string(path)?;
        Ok(ron::de::from_str(&content)?)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let mut path = Self::get_config_path()?;
        path.push("config.ron");
        let content = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn detect_display(&self) -> DisplaySettings {
        println!("[Aura]  Searching for optimal display...");
        let displays = DisplayInfo::all().unwrap_or_default();

        let best = displays
            .iter()
            .filter(|d| d.width > d.height)
            .max_by(|a, b| {
                let res_a = a.width * a.height;
                let res_b = b.width * b.height;

                res_a
                    .cmp(&res_b)
                    .then_with(|| a.frequency.total_cmp(&b.frequency))
            })
            .or_else(|| displays.iter().find(|d| d.is_primary))
            .or_else(|| displays.first())
            .cloned();

        if let Some(infos) = best {
            DisplaySettings {
                name: infos.name,
                resolution: (infos.width, infos.height),
                refresh_rate: infos.frequency.round() as u32,
                hdr: false,
            }
        } else {
            println!("Aura did not find suitable display...\nUsing fallback instead");
            DisplaySettings {
                name: "Aura fallback display".to_string(),
                resolution: (
                    DisplaySettings::DEFAULT_WIDTH,
                    DisplaySettings::DEFAULT_HEIGHT,
                ),
                refresh_rate: DisplaySettings::DEFAULT_FPS,
                hdr: false,
            }
        }
    }
}
