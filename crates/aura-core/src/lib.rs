pub mod game_profile;
pub mod gamescope;

pub use game_profile::Builder as ProfileBuilder;
pub use game_profile::GameProfile;

#[cfg(test)]
mod tests {
    use super::GameProfile;
    use super::game_profile;
    use super::gamescope;
    use std::fs;

    #[test]
    fn test_serialization() {
        let profile = game_profile::Builder::new(1245620, "Elden_ring".to_string())
            .params(gamescope::Params::default())
            .gamemode(true)
            .build();
        let result = ron::ser::to_string(&profile);
        assert!(result.is_ok(), "RON serialization failed");

        println!("Generated RON stream: {}", result.unwrap());
    }

    #[test]
    fn test_resolve_screen() {
        let profile = game_profile::Builder::new(0, "Test".to_string())
            .with_params(|p| {
                p.with_output_res((1920, 1080)).with_upscaler(|u| {
                    u.with_settings(gamescope::upscaler::Settings {
                        scale: Some(0.5),
                        sharpness: None,
                    })
                })
            })
            .build();
        let result = profile.params.resolve_screen();
        assert_eq!(
            result.1,
            (960_u32, 540_u32),
            "Upscaler scale calculation failed"
        );
    }

    #[test]
    fn test_config_dir() {
        let path_result = GameProfile::get_config_path();
        assert!(path_result.is_ok(), "Cannot get config path");

        let config_path = path_result.unwrap();
        assert!(
            config_path.exists(),
            "Directory {:?} does not exists",
            config_path
        );
    }

    #[test]
    fn read_and_load_profiles() -> anyhow::Result<()> {
        let profile = GameProfile {
            app_id: 0,
            name: "Test".to_string(),
            params: gamescope::Params::default(),
            gamemode: true,
        };

        let app_id = profile.app_id;
        profile.save_to_disk()?;
        let loaded = GameProfile::load_from_disk(app_id)?;

        assert_eq!(
            loaded.name, profile.name,
            "Loaded profile is different from the saved one",
        );

        // remove the file to clean
        fs::remove_file(GameProfile::get_config_path()?.join(format!("{}.ron", app_id)))?;

        Ok(())
    }

    #[test]
    fn test_load_nonexistent_profile() {
        assert!(GameProfile::load_from_disk(999999).is_err());
    }

    #[test]
    fn test_gamemode_true_by_default_with_new() {
        let profile = GameProfile::new(0, "Test".to_string());
        assert!(profile.gamemode, "Gamemode should be true by default");
    }

    #[test]
    fn test_gamemode_true_by_default_with_builder() {
        let profile = game_profile::Builder::new(0, "Test".to_string()).build();
        assert!(profile.gamemode, "Gamemode should be true by default");
    }
}
