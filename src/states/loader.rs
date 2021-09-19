use crate::GameState;
use amethyst::{
  assets::{DefaultLoader, Handle, Loader, ProgressCounter},
  prelude::*,
  ui::FontAsset,
};

pub struct GameAssets {
  pub square_font: Handle<FontAsset>,
}

pub struct LoadingState {
  progress_counter: ProgressCounter,
}

impl LoadingState {
  pub fn new() -> LoadingState {
    LoadingState {
      progress_counter: ProgressCounter::new(),
    }
  }

  fn load(&mut self, resources: &mut Resources) {
    let font_handle: Handle<FontAsset> = {
      let loader = resources.get::<DefaultLoader>().expect("Get Loader");
      loader.load("font/square.ttf")
    };
    resources.insert(GameAssets {
      square_font: font_handle,
    });
  }
}

impl SimpleState for LoadingState {
  fn on_start(&mut self, data: StateData<'_, GameData>) {
    println!("Starting Loader state...");
    self.load(data.resources);
  }

  fn update(&mut self, _: &mut StateData<'_, GameData>) -> SimpleTrans {
    if self.progress_counter.is_complete() {
      Trans::Switch(Box::new(GameState::default()))
    } else {
      Trans::None
    }
  }
}
