use amethyst::{
  assets::{AssetStorage, Loader},
  core::{
    math::{Point3, Vector3},
    Time, Transform,
  },
  ecs::prelude::Entity,
  prelude::*,
  renderer::{
    formats::texture::ImageFormat,
    sprite::{SpriteSheet, SpriteSheetFormat, SpriteSheetHandle},
    Texture,
  },
  tiles::{MortonEncoder, RenderTiles2D, Tile, TileMap},
  ui::{UiCreator, UiFinder, UiText},
  utils::fps_counter::FpsCounter,
};

use crate::menu::MenuState;
use amethyst::input::{is_key_down, VirtualKeyCode};

pub const TILEMAP_HEIGHT: u32 = 8;
pub const TILEMAP_WIDTH: u32 = 1000;

#[derive(Default, Clone)]
pub struct ExampleTile;
impl Tile for ExampleTile {
  fn sprite(&self, pos: Point3<u32>, _: &World) -> Option<usize> {
    if pos[2] == 1 {
      let x = pos[1];
      const LAST_LINE: u32 = TILEMAP_HEIGHT - 1;
      const PRE_LAST_LINE: u32 = TILEMAP_HEIGHT - 2;
      match x {
        0 => return Some(2),
        1 => return Some(0),
        2 => return Some(1),
        LAST_LINE => return Some(3),
        PRE_LAST_LINE => return Some(4),
        _ => return Some(5),
      }
    }
    return Some(5);
  }
}

#[derive(Default)]
pub struct GameState {
  pub ui_root: Option<Entity>,
  pub fps_display: Option<Entity>,
}

impl SimpleState for GameState {
  fn handle_event(
    &mut self,
    _data: StateData<'_, GameData<'_, '_>>,
    event: StateEvent,
  ) -> SimpleTrans {
    if let StateEvent::Window(event) = event {
      if is_key_down(&event, VirtualKeyCode::Escape) {
        return Trans::Switch(Box::new(MenuState::default()));
      }
    }
    Trans::None
  }

  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    println!("game started");
    self.ui_root = Some(
      data
        .world
        .exec(|mut creator: UiCreator<'_>| creator.create("ui/game.ron", ())),
    );

    let map_sprite_sheet_handle =
      load_sprite_sheet(data.world, "texture/icy.png", "texture/icy.ron");

    let map = TileMap::<ExampleTile, MortonEncoder>::new(
      Vector3::new(TILEMAP_WIDTH, TILEMAP_HEIGHT, 2),
      Vector3::new(128, 128, 1),
      Some(map_sprite_sheet_handle),
    );

    let _map_entity = data
      .world
      .create_entity()
      .with(map)
      .with(Transform::from(Vector3::new(64.0, -64.0, 0.1)))
      .build();
  }

  fn on_stop(&mut self, data: StateData<GameData>) {
    if let Some(root_entity) = self.ui_root {
      data
        .world
        .delete_entity(root_entity)
        .expect("Failed to remove GameState");
    }
  }

  fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    let StateData { world, .. } = state_data;
    if self.fps_display.is_none() {
      world.exec(|finder: UiFinder<'_>| {
        if let Some(entity) = finder.find("fps") {
          self.fps_display = Some(entity);
        }
      });
    }
    let mut ui_text = world.write_storage::<UiText>();
    {
      if let Some(fps_display) = self.fps_display.and_then(|entity| ui_text.get_mut(entity)) {
        if world.read_resource::<Time>().frame_number() % 20 == 0 {
          let fps = world.read_resource::<FpsCounter>().sampled_fps();
          fps_display.text = format!("FPS: {:.*}", 2, fps);
        }
      }
    }
    Trans::None
  }
}

fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
  let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(png_path, ImageFormat::default(), (), &texture_storage)
  };
  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    ron_path,
    SpriteSheetFormat(texture_handle),
    (),
    &sprite_sheet_store,
  )
}
