use amethyst::{
  core::{
    math::{Translation3, UnitQuaternion, Vector3},
    transform::Transform,
    Named,
  },
  input::{is_close_requested, is_key_down},
  prelude::*,
  renderer::{
    camera::{ActiveCamera, Camera},
    sprite::SpriteRender,
  },
  tiles::{MortonEncoder, TileMap},
  window::ScreenDimensions,
  winit,
};

use crate::{
  component::{TileRotate, Velocity},
  config::AppConfig,
  hud::{DebugHud, Hud},
  tiles::{IcyTile, SteppeTile},
};

use super::loader::GameAssets;

#[derive(Default)]
pub struct GameState {
  debug_hud: Option<DebugHud>,
}

impl SimpleState for GameState {
  fn on_start(&mut self, data: StateData<'_, GameData>) {
    println!("Starting Game state...");

    let world = data.world;
    let resources = data.resources;
    let (font, map_sprite_sheet, spaceships_sheet) = {
      let assets = resources.get::<GameAssets>().expect("Get game assets");
      (
        assets.square_font.clone(),
        assets.tile_map.clone(),
        assets.spaceships.clone(),
      )
    };

    let app_config = resources
      .get::<AppConfig>()
      .expect("Get game assets")
      .clone();

    let mut hud = DebugHud::new(font);
    hud.enable(world, resources);
    self.debug_hud = Some(hud);

    let (width, height) = {
      let dim = resources
        .get::<ScreenDimensions>()
        .expect("Read ScreenDimensions");
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

    world.push((
      SpriteRender::new(spaceships_sheet.clone(), 0),
      Transform::new(
        Translation3::new(0.0, 0.0, 5.0),
        UnitQuaternion::from_euler_angles(0., 0., -1.5708),
        Vector3::new(1.0, 1.0, 1.0),
      ),
    ));
    let tilemap = TileMap::<IcyTile, MortonEncoder>::new(
      Vector3::new(app_config.tile_map.width, app_config.tile_map.height, 2),
      Vector3::new(app_config.tiles.width, app_config.tiles.height, 1),
      Some(map_sprite_sheet),
    );
    world.push((
      tilemap,
      Transform::from(Vector3::new(0.0, 0.0, 0.0)),
      Velocity {
        x: -1.0,
        y: 0.0,
        z: 0.0,
      },
      TileRotate::default(),
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
