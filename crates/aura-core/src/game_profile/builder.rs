use super::GameProfile;
use super::gamescope;

pub struct Builder {
    profile: GameProfile,
}

impl Builder {
    pub fn new(app_id: u32, name: String) -> Self {
        let profile = GameProfile::new(app_id, name);
        Self { profile: profile }
    }

    pub fn params(mut self, params: gamescope::Params) -> Self {
        self.profile.params = params;
        self
    }

    pub fn with_params<F>(mut self, build_fn: F) -> Self
    where
        F: FnOnce(gamescope::ParamsBuilder) -> gamescope::ParamsBuilder,
    {
        let builder = gamescope::ParamsBuilder::new();
        let params = build_fn(builder).build();
        self.profile.params = params;
        self
    }

    pub fn gamemode(mut self, gamemode: bool) -> Self {
        self.profile.gamemode = gamemode;
        self
    }

    pub fn build(self) -> GameProfile {
        self.profile
    }
}
