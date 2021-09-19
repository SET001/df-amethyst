use crate::component::Velocity;
use crate::hud::{DebugHud, Hud};
use crate::tilemap::ExampleTile;

use amethyst::{
  assets::{DefaultLoader, Handle, Loader, ProcessingQueue},
  core::{math::Vector3, transform::Transform, Named},
  input::{is_close_requested, is_key_down},
  prelude::*,
  renderer::{
    camera::{ActiveCamera, Camera},
    sprite::{SpriteSheet, Sprites},
    Texture,
  },
  tiles::{MortonEncoder, TileMap},
  ui::FontAsset,
  window::{ScreenDimensions, Window},
  winit,
};

#[derive(Default)]
pub struct GameState {
  debug_hud: Option<DebugHud>,
}

const TILEMAP_WIDTH: u32 = 10;
const TILEMAP_HEIGHT: u32 = 8;

impl SimpleState for GameState {
  fn on_start(&mut self, data: StateData<'_, GameData>) {
    let world = data.world;
    let resources = data.resources;

    let font_handle: Handle<FontAsset> = {
      let loader = resources.get::<DefaultLoader>().expect("Get Loader");
      loader.load("font/square.ttf")
    };

    let mut hud = DebugHud::new(font_handle);
    hud.enable(world, resources);

    self.debug_hud = Some(hud);

    let (width, height) = {
      let dim = resources
        .get::<ScreenDimensions>()
        .expect("Read ScreenDimensions");
      let window = resources.get::<Window>().expect("Read Window");
      println!("dimensions: {:?}", dim);
      println!("window: {:?}", window);
      (dim.width(), dim.height())
    };
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 10.0);

    let camera = world.push((
      Named("camera".into()),
      transform,
      Camera::standard_2d(width, height),
    ));
    resources.insert(ActiveCamera {
      entity: Some(camera),
    });

    let map_sprite_sheet_handle =
      load_sprite_sheet(resources, "texture/icy.png", "texture/icy.ron");
    let tilemap = TileMap::<ExampleTile, MortonEncoder>::new(
      Vector3::new(TILEMAP_WIDTH, TILEMAP_HEIGHT, 2),
      Vector3::new(128, 128, 1),
      Some(map_sprite_sheet_handle),
    );
    world.push((
      tilemap,
      Transform::from(Vector3::new(0.0, 0.0, 0.0)),
      Velocity {
        x: -1.0,
        y: 0.0,
        z: 0.0,
      },
    ));
  }

  fn on_stop(&mut self, data: StateData<GameData>) {
    data.world.clear();
  }

  fn handle_event(&mut self, data: StateData<'_, GameData>, event: StateEvent) -> SimpleTrans {
    let StateData { .. } = data;
    if let StateEvent::Window(event) = &event {
      if is_close_requested(&event) || is_key_down(&event, winit::event::VirtualKeyCode::Escape) {
        Trans::Quit
      } else {
        Trans::None
      }
    } else {
      Trans::None
    }
  }

  fn update(&mut self, data: &mut StateData<'_, GameData>) -> SimpleTrans {
    let hud = self.debug_hud.as_ref().unwrap();
    hud.on_update(data.world, data.resources);

    Trans::None
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
