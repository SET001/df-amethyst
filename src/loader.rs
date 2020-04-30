use amethyst::{
  assets::{AssetStorage, Handle, Loader, ProgressCounter},
  prelude::*,
  renderer::{formats::texture::ImageFormat, Texture},
};

use crate::menu::MenuState;

#[derive(Debug)]
pub struct HandleDesc {
  name: String,
  handle: Handle<Texture>,
}

pub struct LoadingState {
  progress_counter: ProgressCounter,
  pub handles: Vec<HandleDesc>,
}

impl LoadingState {
  pub fn new() -> LoadingState {
    LoadingState {
      progress_counter: ProgressCounter::new(),
      handles: Vec::new(),
    }
  }

  pub fn multi_load<T>(&mut self, paths: Vec<&str>, world: &World) -> Vec<HandleDesc> {
    let loader = world.read_resource::<Loader>();
    paths
      .iter()
      .map(|s| HandleDesc {
        name: s.to_string(),
        handle: loader.load(
          s.to_string(),
          ImageFormat::default(),
          &mut self.progress_counter,
          &world.read_resource::<AssetStorage<Texture>>(),
        ),
      })
      .collect()
  }
}

impl SimpleState for LoadingState {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    println!("loading assets...");
    let assets = vec![
      "texture/backgrounds/space6/bright/sky.png",
      "texture/backgrounds/space6/bright/earth.png",
    ];
    self.handles = self.multi_load::<Texture>(assets, &data.world);
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
