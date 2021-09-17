use amethyst::{
  assets::{AssetStorage, Loader},
  core::{
    math::{Point3, Vector3},
    Time, Transform,
  },
  ecs::{Entity, Read},
  prelude::*,
  renderer::camera::Camera,
  renderer::{formats::texture::ImageFormat, sprite::SpriteSheet, Texture},
  tiles::{
    MortonEncoder,
    // RenderTiles2D,
    Tile,
    TileMap,
  },
  // ui::{UiCreator, UiFinder, UiText},
  utils::fps_counter::FpsCounter,
  window::ScreenDimensions,
};

// use super::menu::MenuState;
// use crate::component::{TileRotate, Velocity};
// use crate::level::{IcyLevel, Level, LevelMap, MapLayers};
// use crate::tilemap::{ExampleTile, TILEMAP_HEIGHT, TILEMAP_WIDTH};
use amethyst::input::{is_key_down, VirtualKeyCode};

#[derive(Default)]
pub struct GameState {
  // pub ui_root: Option<Entity>,
// pub fps_display: Option<Entity>,
// pub map_entity: Option<Entity>,
}

impl SimpleState for GameState {
  // fn handle_event(&mut self, _: StateData<'_, GameData>, event: StateEvent) -> SimpleTrans {
  //   // if let StateEvent::Window(event) = event {
  //   //   if is_key_down(&event, VirtualKeyCode::Escape) {
  //   //     return Trans::Switch(Box::new(MenuState::default()));
  //   //   }
  //   // }
  //   Trans::None
  // }

  // fn on_start(&mut self, data: StateData<'_, GameData>) {
  //   println!("GameState.on_start");
  //   // println!("game started");
  //   // self.ui_root = Some(
  //   //   data
  //   //     .world
  //   //     .exec(|mut creator: UiCreator<'_>| creator.create("ui/game.ron", ())),
  //   // );

  //   // let map_sprite_sheet_handle =
  //   //   load_sprite_sheet(data.world, "texture/icy.png", "texture/icy.ron");

  //   // let tilemap = TileMap::<ExampleTile, MortonEncoder>::new(
  //   //   Vector3::new(TILEMAP_WIDTH, TILEMAP_HEIGHT, 2),
  //   //   Vector3::new(128, 128, 1),
  //   //   Some(map_sprite_sheet_handle),
  //   // );
  //   // let mut world = data.world;
  //   // world.register::<TileRotate>();
  //   // self.map_entity = Some(
  //   //   world
  //   //     .create_entity()
  //   //     .with(tilemap)
  //   //     .with(Transform::from(Vector3::new(0.0, -64.0, 0.0)))
  //   //     .with(Velocity {
  //   //       x: -1.0,
  //   //       y: 0.0,
  //   //       z: 0.0,
  //   //     })
  //   //     .with(TileRotate::default())
  //   //     .build(),
  //   // );

  //   // let mut level = IcyLevel::new(TILEMAP_WIDTH as usize, TILEMAP_HEIGHT as usize);
  //   // level.gen_map();
  //   // world.insert::<MapLayers>(level.map_layers);
  //   // world.setup::<Read<MapLayers>>();

  //   // let aa_entity = Some(
  //   //   world
  //   //     .create_entity()
  //   //     .with(level)
  //   // )
  //   // init_camera(&mut world);
  // }

  // fn on_stop(&mut self, data: StateData<GameData>) {
  //   // data.world.delete_all()
  // }

  // fn update(&mut self, data: &mut StateData<'_, GameData>) -> SimpleTrans {
  //   println!("GameState.update");
  //   // let StateData { world, .. } = state_data;
  //   // if self.fps_display.is_none() {
  //   //   world.exec(|finder: UiFinder<'_>| {
  //   //     if let Some(entity) = finder.find("fps") {
  //   //       self.fps_display = Some(entity);
  //   //     }
  //   //   });
  //   // }
  //   // let mut ui_text = world.write_storage::<UiText>();
  //   // {
  //   //   if let Some(fps_display) = self.fps_display.and_then(|entity| ui_text.get_mut(entity)) {
  //   //     if world.read_resource::<Time>().frame_number() % 20 == 0 {
  //   //       let fps = world.read_resource::<FpsCounter>().sampled_fps();
  //   //       fps_display.text = format!("FPS: {:.*}", 2, fps);
  //   //     }
  //   //   }
  //   // }
  //   Trans::None
  // }
}

fn init_camera(world: &mut World) {
  // let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
  // let mut transform = Transform::default();
  // transform.set_translation_xyz(0.0, 0.0, 10.0);
  // let camera = Camera::standard_2d(dimensions.width(), dimensions.height());
  // world.create_entity().with(camera).with(transform).build();
}

fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str)
// -> SpriteSheetHandle
{
  // let texture_handle = {
  //   let loader = world.read_resource::<Loader>();
  //   let texture_storage = world.read_resource::<AssetStorage<Texture>>();
  //   loader.load(png_path, ImageFormat::default(), (), &texture_storage)
  // };
  // let loader = world.read_resource::<Loader>();
  // let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  // loader.load(
  //   ron_path,
  //   SpriteSheetFormat(texture_handle),
  //   (),
  //   &sprite_sheet_store,
  // )
}
