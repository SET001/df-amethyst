use amethyst::core::transform::Transform;
use amethyst::window::ScreenDimensions;
use amethyst::{
  assets::{AssetStorage, Handle},
  core::math::Vector3,
  ecs::prelude::Entity,
  prelude::*,
  renderer::{
    camera::{Camera, Projection},
    SpriteRender, SpriteSheet, Texture, Transparent,
  },
  ui::{UiCreator, UiEvent, UiEventType, UiFinder, UiText},
};

use amethyst::input::{is_key_down, VirtualKeyCode};

const BUTTON_START: &str = "start";
const BUTTON_EXIT: &str = "exit";

use crate::game::GameState;
use crate::loader::{Assets, AssetsMap, LoadingState};

#[derive(Default)]
pub struct MenuState {
  pub ui_root: Option<Entity>,
  pub button_start: Option<Entity>,
  pub button_exit: Option<Entity>,
}

impl SimpleState for MenuState {
  fn handle_event(&mut self, _: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
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

  fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;
    init_camera(world);
    self.ui_root = Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));
    let assets = world.read_resource::<AssetsMap>();
  }

  fn on_stop(&mut self, data: StateData<GameData>) {
    if let Some(root_entity) = self.ui_root {
      data
        .world
        .delete_entity(root_entity)
        .expect("Failed to remove MainMenu");
    }
  }

  fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
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

fn init_camera(world: &mut World) {
  let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
  let mut transform = Transform::default();
  transform.set_translation_xyz(0.0, 0.0, 10.0);
  let camera = Camera::standard_2d(dimensions.width(), dimensions.height());
  world.create_entity().with(camera).with(transform).build();
}

fn initialize_sprite(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
  let (width, height) = {
    let dim = world.read_resource::<ScreenDimensions>();
    (dim.width(), dim.height())
  };
  let mut sprite_transform = Transform::default();
  sprite_transform.set_translation_xyz(width / 2., height / 2., 0.);

  let sprite_render = SpriteRender {
    sprite_sheet: sprite_sheet_handle,
    sprite_number: 0, // First sprite
  };
  world
    .create_entity()
    .with(sprite_render)
    .with(sprite_transform)
    .with(Transparent)
    .build();
}
