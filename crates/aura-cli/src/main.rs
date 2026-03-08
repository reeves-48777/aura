use aura_core::*;

fn main() -> anyhow::Result<()> {
    let profile = ProfileBuilder::new(0, "Test Gears".to_string())
        .with_params(|p| {
            p.with_output_res((3440, 1440))
                .with_nested_res((3440, 1440))
                .with_refresh_rate(165)
                .with_upscaler(|u| {
                    u.with_filter(gamescope::upscaler::Filter::Fsr)
                        .with_settings(gamescope::upscaler::Settings {
                            scale: Some(0.5),
                            sharpness: None,
                        })
                })
                .immediate_flips()
        })
        .gamemode(true)
        .build();

    let cmd = vec!["glxgears".to_string()];

    println!("Aura started...");

    profile.launch(cmd)?;

    Ok(())
}
