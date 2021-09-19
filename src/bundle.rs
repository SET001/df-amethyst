use crate::config::AppConfig;
use crate::system::VelocitySystem;
use amethyst::{
  config::{Config, ConfigError},
  core::ecs::{DispatcherBuilder, Resources, SystemBundle, World},
  error::Error,
};

pub struct DfBundle {
  config: AppConfig,
}

impl DfBundle {
  pub fn from_config(config: AppConfig) -> Self {
    DfBundle { config }
  }
  pub fn from_config_path(path: impl AsRef<std::path::Path>) -> Result<Self, ConfigError> {
    Ok(DfBundle::from_config(AppConfig::load(path.as_ref())?))
  }
}

impl SystemBundle for DfBundle {
  fn load(
    &mut self,
    _world: &mut World,
    resources: &mut Resources,
    builder: &mut DispatcherBuilder,
  ) -> Result<(), Error> {
    resources.insert(self.config.clone());
    builder.add_system(VelocitySystem);
    Ok(())
  }
}
