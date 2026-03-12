use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuraError {
    #[error("Resolution {0}x{1} is not valid (too small or invalid format)")]
    InvalidResolution(u32, u32),

    #[error("Refresh rate {0}Hz is out of bounds. Aura capped it to {1}Hz to protect your GPU.")]
    RefreshRateClamped(u32, u32),

    #[error("The scale {0} is out of bounds (MIN: {2} MAX: {3}). Aura clamped it to {1}")]
    ScaleClamped(f32, f32, f32, f32),

    #[error("The sharpness {0} is out of bounds (MIN: {2} MAX: {3}). Aura clamped it to {1}")]
    SharpnessClamped(f32, f32, f32, f32),
}
