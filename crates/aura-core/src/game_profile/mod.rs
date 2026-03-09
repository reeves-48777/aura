pub mod builder;

pub use builder::Builder;

use super::gamescope;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameProfile {
    pub app_id: u32,
    pub name: String,
    pub params: gamescope::Params,
    pub gamemode: bool,
}

use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

impl GameProfile {
    pub fn new(app_id: u32, name: String) -> Self {
        Self {
            app_id,
            name,
            params: gamescope::Params::default(),
            gamemode: true,
        }
    }

    pub fn get_profiles_path() -> anyhow::Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "aura", "aura")
            .ok_or_else(|| anyhow::anyhow!("Cannot find project dirs for aura"))?;
        let path = proj_dirs.data_dir().join("profiles");

        if !path.exists() {
            fs::create_dir_all(&path)?;
        }

        Ok(path)
    }

    pub fn save_to_disk(&self) -> anyhow::Result<()> {
        let mut path = Self::get_profiles_path()?;
        path.push(format!("{}.ron", self.app_id));

        let content = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?;
        fs::write(path, content)?;

        Ok(())
    }

    pub fn load_from_disk(app_id: u32) -> anyhow::Result<Self> {
        let mut path = Self::get_profiles_path()?;
        path.push(format!("{}.ron", app_id));

        let content = fs::read_to_string(path)?;

        let mut profile: GameProfile = ron::de::from_str(&content)?;
        // SHIELD
        profile.params.upscaler.settings.sanitize();

        Ok(profile)
    }

    pub fn launch(&self, game_args: Vec<String>) -> anyhow::Result<()> {
        let (nested, output, refresh) = self.params.resolve_screen();
        let mut args = vec![
            "-w".to_string(),
            nested.0.to_string(),
            "-h".to_string(),
            nested.1.to_string(),
            "-W".to_string(),
            output.0.to_string(),
            "-H".to_string(),
            output.1.to_string(),
            "-r".to_string(),
            refresh.to_string(),
        ];

        if let Some(max_scale) = self.params.max_scale {
            args.push("--max-scale".to_string());
            args.push(max_scale.to_string());
        }

        if let Some(scaler_type) = &self.params.upscaler.scaler_type {
            args.push("-S".to_string());
            args.push(scaler_type.to_string());
        }

        if let Some(scaler_filter) = &self.params.upscaler.filter {
            args.push("-F".to_string());
            args.push(scaler_filter.to_string());
        }

        if let Some(upscaler_sharpness) = self.params.upscaler.settings.sharpness() {
            args.push("--sharpness".to_string());
            args.push(upscaler_sharpness.to_string());
        }

        if self.params.mangoapp {
            args.push("--mangoapp".to_string());
        }

        if let Some(sensitivity) = self.params.mouse_sensitivity {
            args.push("-s".to_string());
            args.push(sensitivity.to_string());
        }

        if self.params.adaptive_sync {
            args.push("--adaptive-sync".to_string());
        }

        if self.params.force_grab_cursor {
            args.push("--force-grab-cursor".to_string());
        }

        if self.params.immediate_flips {
            args.push("--immediate-flips".to_string());
        }

        if let Some(display_mode) = &self.params.nested_display {
            args.push(display_mode.to_string())
        }

        args.push("--".to_string());

        args.extend(game_args);

        let mut final_cmd = if self.gamemode {
            let mut c = Command::new("gamemoderun");
            c.arg("gamescope").args(args);
            c
        } else {
            let mut c = Command::new("gamescope");
            c.args(args);
            c
        };

        println!("Aura started cmd: {:?}", final_cmd);

        let status = final_cmd.status()?;

        if !status.success() {
            return Err(anyhow::anyhow!("gamescope exited with non-zero status"));
        }

        Ok(())
    }
}
