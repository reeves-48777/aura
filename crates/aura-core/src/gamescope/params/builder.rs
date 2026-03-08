use crate::gamescope;

use super::{DisplayMode, Params, RenderingBackend};

pub struct Builder {
    params: Params,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            params: Params::default(),
        }
    }

    pub fn with_output_res(mut self, resolution: (u32, u32)) -> Self {
        self.params.output_res = resolution;
        self
    }

    pub fn with_nested_res(mut self, resolution: (u32, u32)) -> Self {
        self.params.nested_res = resolution;
        self
    }

    pub fn with_refresh_rate(mut self, rate: u32) -> Self {
        self.params.nested_refresh = rate;
        self
    }

    pub fn with_upscaler<F>(mut self, build_fn: F) -> Self
    where
        F: FnOnce(gamescope::upscaler::Builder) -> gamescope::upscaler::Builder,
    {
        let builder = gamescope::upscaler::Builder::new();
        self.params.upscaler = build_fn(builder).build();
        self
    }

    pub fn with_mouse_sensitivity(mut self, sens: f32) -> Self {
        self.params.mouse_sensitivity = Some(sens);
        self
    }

    pub fn with_rendering_backend(mut self, backend: RenderingBackend) -> Self {
        self.params.backend = Some(backend);
        self
    }

    pub fn mangoapp(mut self) -> Self {
        self.params.mangoapp = true;
        self
    }

    pub fn adaptive_sync(mut self) -> Self {
        self.params.adaptive_sync = true;
        self
    }

    pub fn immediate_flips(mut self) -> Self {
        self.params.immediate_flips = true;
        self
    }

    pub fn force_grab_cursor(mut self) -> Self {
        self.params.force_grab_cursor = true;
        self
    }

    pub fn with_nested_display(mut self, display: DisplayMode) -> Self {
        self.params.nested_display = Some(display);
        self
    }

    pub fn build(self) -> Params {
        self.params
    }
}
