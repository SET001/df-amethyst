use amethyst::{
  assets::{AssetStorage, Handle, Loader, ProgressCounter},
  prelude::*,
  renderer::{formats::texture::ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::menu::MenuState;

pub struct LoadingState {
  //   /// Tracks loaded assets.
  progress_counter: ProgressCounter,
  //   /// Handle to the player texture.
  pub background_texture_handle: Option<Handle<SpriteSheet>>,
}

impl LoadingState {
  pub fn new() -> LoadingState {
    LoadingState {
      progress_counter: ProgressCounter::new(),
      background_texture_handle: None,
    }
  }
}

impl SimpleState for LoadingState {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    println!("loading assets...");
    let loader = &data.world.read_resource::<Loader>();
    let texture_handle = loader.load(
      "splash_concept_icy.jpg",
      ImageFormat::default(),
      &mut self.progress_counter,
      &data.world.read_resource::<AssetStorage<Texture>>(),
    );

    let background_texture_handle = loader.load(
      "logo.ron",
      SpriteSheetFormat(texture_handle),
      &mut self.progress_counter,
      &data.world.read_resource::<AssetStorage<SpriteSheet>>(),
    );

    self.background_texture_handle = Some(background_texture_handle);
  }

  fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    if self.progress_counter.is_complete() {
      let background_texture_handle = self
        .background_texture_handle
        .take()
        .expect("Expected `background_texture_handle` to be loaded.");
      Trans::Switch(Box::new(MenuState {
        background_texture_handle,
      }))
    } else {
      Trans::None
    }
  }
}
