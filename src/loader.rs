use crate::menu::MenuState;
use amethyst::{
  assets::{AssetStorage, Handle, Loader, ProgressCounter},
  prelude::*,
  renderer::{formats::texture::ImageFormat, Texture},
};
use enum_map::{enum_map, Enum, EnumMap};
use serde::Deserialize;

#[derive(Debug, Deserialize, Enum)]

pub enum Assets {
  SKY,
  EARTH,
  MOON,
}

#[derive(Debug)]
pub struct HandleDesc {
  name: String,
  handle: Handle<Texture>,
}

pub type AssetsMap = EnumMap<Assets, Option<Handle<Texture>>>;

pub struct LoadingState<'a> {
  assets_config: EnumMap<Assets, &'a str>,
  progress_counter: ProgressCounter,
}

impl<'a> LoadingState<'a> {
  pub fn new() -> LoadingState<'a> {
    LoadingState {
      progress_counter: ProgressCounter::new(),
      assets_config: enum_map! {
        Assets::SKY => "texture/backgrounds/space6/bright/sky.png",
        Assets::EARTH => "texture/backgrounds/space6/bright/earth.png",
        Assets::MOON => "texture/backgrounds/space6/bright/moon.png",
      },
    }
  }

  fn load<T>(&mut self, world: &World) -> AssetsMap {
    let loader = world.read_resource::<Loader>();
    let mut handles = AssetsMap::new();
    for (k, v) in &self.assets_config {
      handles[k] = Some(loader.load(
        v.to_string(),
        ImageFormat::default(),
        &mut self.progress_counter,
        &world.read_resource::<AssetStorage<Texture>>(),
      ))
    }
    return handles;
  }
}

impl SimpleState for LoadingState<'_> {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    println!("loading assets...");
    let handles = self.load::<Texture>(&data.world);
    data.world.insert(handles);
  }

  fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    if self.progress_counter.is_complete() {
      println!("assets loading complete...");
      Trans::Switch(Box::new(MenuState::default()))
    } else {
      Trans::None
    }
  }
}
