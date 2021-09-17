use crate::component::Velocity;
use crate::tilemap::ExampleTile;

use amethyst::{
  assets::{DefaultLoader, Handle, Loader, ProcessingQueue},
  core::{
    math::Vector3,
    transform::{Parent, Transform},
    Named,
  },
  input::{is_close_requested, is_key_down},
  prelude::*,
  renderer::{
    camera::{ActiveCamera, Camera},
    sprite::{SpriteSheet, Sprites},
    Texture,
  },
  tiles::{MortonEncoder, TileMap},
  window::ScreenDimensions,
  winit,
};
#[derive(Default)]
pub struct GameState {}

const TILEMAP_WIDTH: u32 = 50;
const TILEMAP_HEIGHT: u32 = 8;

impl SimpleState for GameState {
  fn on_start(&mut self, data: StateData<'_, GameData>) {
    let mut world = data.world;
    println!("GameState.on_start");

    let (width, height) = {
      let dim = data
        .resources
        .get::<ScreenDimensions>()
        .expect("Read ScreenDimensions");
      (dim.width(), dim.height())
    };
    println!("{}, {}", width, height);
    let camera = world.push((
      Named("camera".into()),
      // Parent(player),
      Transform::from(Vector3::new(0.0, 0.0, 1.1)),
      Camera::standard_2d(width, height),
    ));
    data.resources.insert(ActiveCamera {
      entity: Some(camera),
    });
    let map_sprite_sheet_handle =
      load_sprite_sheet(data.resources, "texture/icy.png", "texture/icy.ron");
    println!("{:?}", map_sprite_sheet_handle);
    let tilemap = TileMap::<ExampleTile, MortonEncoder>::new(
      Vector3::new(TILEMAP_WIDTH, TILEMAP_HEIGHT, 1),
      Vector3::new(128, 128, 1),
      Some(map_sprite_sheet_handle),
    );
    world.push((
      tilemap,
      Transform::default(),
      Velocity {
        x: -1.0,
        y: 0.0,
        z: 0.0,
      },
    ));
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
