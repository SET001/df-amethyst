use amethyst::{assets::Handle, prelude::*, renderer::Texture};

pub struct GameState {
  pub texture_handle: Handle<Texture>,
}

impl SimpleState for GameState {
  fn handle_event(
    &mut self,
    _data: StateData<'_, GameData<'_, '_>>,
    _event: StateEvent,
  ) -> SimpleTrans {
    Trans::None
  }

  fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
    println!("game started");
  }
}
