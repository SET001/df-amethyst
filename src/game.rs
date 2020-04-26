use amethyst::{
  core::Time,
  ecs::prelude::Entity,
  prelude::*,
  ui::{UiCreator, UiFinder, UiText},
  utils::fps_counter::FpsCounter,
};

use crate::menu::MenuState;
use amethyst::input::{is_key_down, VirtualKeyCode};

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
