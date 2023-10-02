use amethyst::{
  config::{Config, ConfigError},
  core::{
    ecs::{DispatcherBuilder, Resources, SystemBundle, World},
    EventChannel,
  },
  error::Error,
  input::InputEvent,
};

use crate::{
  config::AppConfig,
  system::{MouseControllerSystem, TileRotateSystem, VelocitySystem},
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
    resources.insert(EventChannel::<InputEvent>::new());

    let mouse_events_reader = resources
      .get_mut::<EventChannel<InputEvent>>()
      .unwrap()
      .register_reader();

    builder
      .add_system(VelocitySystem)
      .add_system(TileRotateSystem)
      .add_system(MouseControllerSystem::new(mouse_events_reader));
    Ok(())
  }
}
