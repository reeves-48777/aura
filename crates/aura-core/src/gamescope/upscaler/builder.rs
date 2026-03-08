use super::{Filter, Settings, Type, Upscaler};

pub struct Builder {
    upscaler: Upscaler,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            upscaler: Upscaler::default(),
        }
    }

    pub fn with_type(mut self, upscaler_type: Type) -> Self {
        self.upscaler.scaler_type = Some(upscaler_type);
        self
    }

    pub fn with_filter(mut self, filter: Filter) -> Self {
        self.upscaler.filter = Some(filter);
        self
    }

    pub fn with_settings(mut self, settings: Settings) -> Self {
        self.upscaler.settings = settings;
        self
    }

    pub fn build(self) -> Upscaler {
        self.upscaler
    }
}
