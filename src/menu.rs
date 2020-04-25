use amethyst::core::transform::Transform;
use amethyst::window::ScreenDimensions;
use amethyst::{
  assets::Handle,
  prelude::*,
  renderer::{
    camera::{Camera, Projection},
    SpriteRender, SpriteSheet,
  },
};
#[derive(Clone)]
pub struct MenuState {
  pub background_texture_handle: Handle<SpriteSheet>,
}

impl SimpleState for MenuState {
  fn handle_event(
    &mut self,
    _data: StateData<'_, GameData<'_, '_>>,
    _event: StateEvent,
  ) -> SimpleTrans {
    Trans::None
  }
  fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
    println!("menu opened");
    let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();
    init_camera(data.world, &dimensions);
    initialize_sprite(&mut data.world, self.background_texture_handle.clone());
  }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
  let mut transform = Transform::default();
  transform.set_translation_xyz(-dimensions.width() * 0.5, -dimensions.height() * 0.5, 0.1);
  world
    .create_entity()
    .with(Camera::standard_2d(
      dimensions.width(),
      -dimensions.height(),
    ))
    .with(transform)
    .build();
}

fn initialize_sprite(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
  let (width, height) = {
    let dim = world.read_resource::<ScreenDimensions>();
    (dim.width(), dim.height())
  };
  // Move the sprite to the middle of the window
  let mut sprite_transform = Transform::default();
  // sprite_transform.set_translation_xyz(width / 2., height / 2., 0.);
  println!("spritesheet {:?}", sprite_sheet_handle);

  let sprite_render = SpriteRender {
    sprite_sheet: sprite_sheet_handle,
    sprite_number: 0, // First sprite
  };
  world
    .create_entity()
    .with(sprite_render)
    .with(sprite_transform)
    .build();
}
