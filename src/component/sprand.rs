use amethyst::{
  assets::PrefabData,
  derive::PrefabData,
  ecs::Entity,
  ecs::{storage::VecStorage, Component, WriteStorage},
  Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Component, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
#[storage(VecStorage)]
pub struct RandomizeSpawnPoint {
  pub randomized: bool,
}
