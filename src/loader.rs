use amethyst::{
  assets::{AssetStorage, Handle, Loader, ProgressCounter},
  prelude::*,
  renderer::{formats::texture::ImageFormat, Sprite, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::menu::MenuState;

pub struct LoadingState {
  //   /// Tracks loaded assets.
  progress_counter: ProgressCounter,
  //   /// Handle to the player texture.
  pub texture_handle: Option<Handle<Texture>>,
}

impl LoadingState {
  pub fn new() -> LoadingState {
    LoadingState {
      progress_counter: ProgressCounter::new(),
      texture_handle: None,
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

    self.texture_handle = Some(texture_handle);
  }

  fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    if self.progress_counter.is_complete() {
      let texture_handle = self.texture_handle.take().expect(
        "Expected `texture_handle` to exist when \
         `progress_counter` is complete.",
      );
      let loader = &data.world.read_resource::<Loader>();
      // let sprite_sheet = self.load_sprite_sheet(texture_handle);
      // println!("spritesheet {:?}", sprite_sheet);

      let background_texture_handle = loader.load(
        "logo.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &data.world.read_resource::<AssetStorage<SpriteSheet>>(),
      );
      Trans::Switch(Box::new(MenuState {
        background_texture_handle,
      }))
    } else {
      Trans::None
    }
  }
}
