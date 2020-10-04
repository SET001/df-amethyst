use super::menu::MenuState;
use crate::component::{Dimensions, RangedScroller, Scroller};
use amethyst::core::transform::Transform;
use amethyst::input::{is_key_down, VirtualKeyCode};
use amethyst::{
  assets::{AssetStorage, Handle, Prefab, PrefabData, PrefabLoader, ProgressCounter, RonFormat},
  derive::PrefabData,
  ecs::{storage::DenseVecStorage, Component, Entity, WriteStorage},
  prelude::*,
  renderer::{
    camera::Camera,
    sprite::{prefab::SpriteScenePrefab, SpriteRender},
  },
  window::ScreenDimensions,
  Error,
};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Deserialize, Serialize, Component, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct Position(pub f32, pub f32, pub f32);

#[derive(Debug, Clone, Deserialize, Serialize, Component, PrefabData)]
#[serde(deny_unknown_fields)]
pub enum CustomPrefabData {
  Moon { sprite: SpriteScenePrefab },
}
// #[derive(Deserialize, Serialize, PrefabData)]
// #[serde(deny_unknown_fields)]
// pub struct ScrollerPrefab {
//   scroller: Scroller,
//   transform: Transform,
//   dimension: Dimensions,
// }

#[derive(new, Default)]
pub struct PrefabsTest {
  #[new(default)]
  pub progress_counter: Option<ProgressCounter>,
  #[new(default)]
  pub moon_prefab: Option<Handle<Prefab<CustomPrefabData>>>,
}

impl SimpleState for PrefabsTest {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let StateData { world, .. } = data;
    self.progress_counter = Some(Default::default());
    let moon_prefab = world.exec(|loader: PrefabLoader<'_, CustomPrefabData>| {
      loader.load(
        "prefab/moon.ron",
        RonFormat,
        self.progress_counter.as_mut().unwrap(),
      )
    });
    world.create_entity().with(moon_prefab).build();
    initialise_camera(world);
  }
}

fn initialise_camera(world: &mut World) {
  let (width, height) = {
    let dim = world.read_resource::<ScreenDimensions>();
    (dim.width(), dim.height())
  };

  let mut camera_transform = Transform::default();
  camera_transform.set_translation_z(1.0);

  world
    .create_entity()
    .with(camera_transform)
    .with(Camera::standard_2d(width, height))
    .build();
}
