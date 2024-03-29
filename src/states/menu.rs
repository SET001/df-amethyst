use amethyst::core::transform::Transform;
use amethyst::window::ScreenDimensions;
use amethyst::{
  assets::{AssetStorage, Handle, Loader},
  ecs::prelude::Entity,
  prelude::*,
  renderer::{camera::Camera, Sprite, SpriteRender, SpriteSheet, Texture, Transparent},
  ui::{UiCreator, UiEvent, UiEventType, UiFinder},
};

use amethyst::input::{is_key_down, VirtualKeyCode};
use rand::Rng;

const BUTTON_START: &str = "start";
const BUTTON_EXIT: &str = "exit";

use super::game::GameState;
use super::loader::{Assets, AssetsMap};
use crate::component::{Dimensions, RangedScroller, Scroller, TileRotate};

#[derive(Default)]
pub struct MenuState {
  pub ui_root: Option<Entity>,
  pub button_start: Option<Entity>,
  pub button_exit: Option<Entity>,
}

impl SimpleState for MenuState {
  fn handle_event(&mut self, data: StateData<'_, GameData>) -> SimpleTrans {
    match event {
      StateEvent::Window(event) => {
        if is_key_down(&event, VirtualKeyCode::Escape) {
          Trans::Quit
        } else {
          Trans::None
        }
      }
      StateEvent::Ui(UiEvent {
        event_type: UiEventType::Click,
        target,
      }) => {
        if Some(target) == self.button_start {
          log::info!("[Trans::Switch] Switching to Game!");
          return Trans::Switch(Box::new(GameState::default()));
        }
        if Some(target) == self.button_exit {
          log::info!("[Trans::Switch] Quiting to Game!");
          return Trans::Quit;
        }
        Trans::None
      }
      _ => Trans::None,
    }
  }

  fn on_start(&mut self, data: StateData<'_, GameData>) {
    let mut world = data.world;
    world.register::<Scroller>();
    world.register::<Dimensions>();
    init_camera(&mut world);
    self.ui_root = Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));
    add_sky_scroller(world);
    add_planet_scroller(world);
    add_planet_scroller(world);
    add_planet_scroller(world);
    add_planet_scroller(world);
    add_planet_scroller(world);
    add_earth(world);
  }

  fn on_stop(&mut self, data: StateData<GameData>) {
    data.world.delete_all()
  }

  fn update(&mut self, data: StateData<'_, GameData>) -> SimpleTrans {
    let StateData { world, .. } = state_data;
    if self.button_start.is_none() {
      world.exec(|ui_finder: UiFinder<'_>| {
        self.button_start = ui_finder.find(BUTTON_START);
        self.button_exit = ui_finder.find(BUTTON_EXIT);
      });
    }
    Trans::None
  }
}

fn add_planet_scroller(world: &mut World) {
  let mut rng = rand::thread_rng();
  let screen_dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
  let distance = rng.gen_range(50, 100) as f32 / 100.0;
  let scroller = RangedScroller::new(vec![add_sprite(world, Assets::MOON_SMALL)], distance);
  let dimensions = Dimensions {
    width: screen_dimensions.width(),
    height: screen_dimensions.height(),
  };
  let mut transform = Transform::default();
  transform.set_translation_xyz(
    0.0,
    rng.gen_range(0, screen_dimensions.height() as i32) as f32,
    0.0,
  );

  world
    .create_entity()
    .with(dimensions)
    .with(transform)
    .with(scroller)
    .build();
}

fn add_sky_scroller(world: &mut World) {
  let screen_dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
  let scroller = Scroller::new(vec![(add_sprite(world, Assets::SKY))], 0.2);
  let dimensions = Dimensions {
    width: screen_dimensions.width(),
    height: screen_dimensions.height(),
  };

  let mut transform = Transform::default();
  transform.set_translation_xyz(0.0, screen_dimensions.height(), 0.0);
  world
    .create_entity()
    .with(dimensions)
    .with(transform)
    .with(scroller)
    .build();
}

fn add_earth(world: &mut World) {
  let screen_dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
  let earth = add_sprite(world, Assets::EARTH);
  let mut transform = Transform::default();
  transform.set_translation_xyz(screen_dimensions.width() / 2.0, earth.2 as f32 / 2.0, 100.0);
  world
    .create_entity()
    .with(earth.0)
    .with(transform)
    .with(Transparent)
    .build();
}

fn init_camera(world: &mut World) {
  let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
  let mut transform = Transform::default();
  transform.set_translation_xyz(dimensions.width() / 2.0, dimensions.height() / 2.0, 1000.0);
  let camera = Camera::standard_2d(dimensions.width(), dimensions.height());
  world.create_entity().with(camera).with(transform).build();
}

// let (width, height) = {
//   let dim = world.read_resource::<ScreenDimensions>();
//   (dim.width(), dim.height())
// };

fn add_sprite(world: &mut World, asset: Assets) -> (SpriteRender, u32, u32) {
  let assets = world.read_resource::<AssetsMap>();
  let masset = assets.clone()[asset].clone();
  let sprite_sheet_handle = {
    let textureHandle = masset.0.unwrap();
    let sprite_sheet = load_sprite_sheet(textureHandle, masset.1, masset.2);
    let loader = world.read_resource::<Loader>();
    loader.load_from_data(
      sprite_sheet,
      (),
      &world.read_resource::<AssetStorage<SpriteSheet>>(),
    )
  };
  (
    SpriteRender {
      sprite_sheet: sprite_sheet_handle,
      sprite_number: 0, // First sprite
    },
    masset.1,
    masset.2,
  )
}

pub fn load_sprite_sheet(texture: Handle<Texture>, width: u32, height: u32) -> SpriteSheet {
  let sprite_count = 1; // number of sprites
  let mut sprites = Vec::with_capacity(sprite_count);

  // Here we are loading the 5th sprite on the bottom row.
  let offset_x = 0; // 5th sprite * 10 pixel sprite width
  let offset_y = 0; // Second row (1) * 10 pixel sprite height
  let offsets = [0.0; 2]; // Align the sprite with the middle of the entity.

  let sprite = Sprite::from_pixel_values(
    width, height, width, height, offset_x, offset_y, offsets, false, false,
  );
  sprites.push(sprite);
  SpriteSheet { texture, sprites }
}
