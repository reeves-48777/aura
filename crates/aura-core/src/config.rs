use directories::ProjectDirs;
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
}
