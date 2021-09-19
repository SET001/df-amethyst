use crate::GameState;
use amethyst::{
  assets::{DefaultLoader, Handle, Loader, ProcessingQueue, ProgressCounter},
  prelude::*,
  renderer::{
    sprite::{SpriteSheet, Sprites},
    Texture,
  },
  ui::FontAsset,
};

pub struct GameAssets {
  pub square_font: Handle<FontAsset>,
  pub tile_map: Handle<SpriteSheet>,
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
    let tile_map_handle =
      load_sprite_sheet(resources, "texture/tilemap.png", "texture/tilemap.ron");
    resources.insert(GameAssets {
      square_font: font_handle,
      tile_map: tile_map_handle,
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
      println!("Loading complete...");
      Trans::Switch(Box::new(GameState::default()))
    } else {
      println!("Loading...");
      Trans::None
    }
  }
}

fn load_sprite_sheet(
  resources: &mut Resources,
  png_path: &str,
  ron_path: &str,
) -> Handle<SpriteSheet> {
  let loader = resources.get::<DefaultLoader>().expect("Get Loader");
  let texture: Handle<Texture> = loader.load(png_path);
  let sprites: Handle<Sprites> = loader.load(ron_path);
  println!("{:?}, {:?}", texture, sprites);
  let sprite_sheet_store = resources.get::<ProcessingQueue<SpriteSheet>>().unwrap();
  loader.load_from_data(SpriteSheet { texture, sprites }, (), &sprite_sheet_store)
}
