use amethyst::{
  core::ecs::{DispatcherBuilder, Resources, SystemBundle, World},
  error::Error,
};

use crate::system::VelocitySystem;

pub struct DfBundle;

impl SystemBundle for DfBundle {
  fn load(
    &mut self,
    _world: &mut World,
    _resources: &mut Resources,
    builder: &mut DispatcherBuilder,
  ) -> Result<(), Error> {
    builder.add_system(VelocitySystem);
    Ok(())
  }
}
