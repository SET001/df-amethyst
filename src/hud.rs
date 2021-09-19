use amethyst::{
  assets::Handle,
  prelude::*,
  ui::{Anchor, FontAsset, LineMode, UiLabel, UiLabelBuilder, UiText},
  utils::fps_counter::FpsCounter,
};

pub trait Hud {
  fn enable(&mut self, world: &mut World, resources: &mut Resources);
  fn disable(&self, _: &mut World, _: &mut Resources) {}
  fn on_event(&self, _: &mut World, _: &mut Resources) {}
  fn on_update(&self, _: &mut World, _: &mut Resources) {}
}

pub struct DebugHud {
  fps: Option<UiLabel>,
  world_entries: Option<UiLabel>,
  color: [f32; 4],
  font_size: f32,
  font: Handle<FontAsset>,
}

impl DebugHud {
  pub fn new(font: Handle<FontAsset>) -> DebugHud {
    DebugHud {
      fps: None,
      world_entries: None,
      color: [0.34, 0.36, 0.52, 1.0],
      font_size: 30.,
      font,
    }
  }
}

impl Hud for DebugHud {
  fn enable(&mut self, world: &mut World, resources: &mut Resources) {
    let (_, label) = UiLabelBuilder::<(), u32>::new(&"fps counter")
      .with_line_mode(LineMode::Wrap)
      .with_position(100., -50.)
      .with_size(400., 200.)
      .with_anchor(Anchor::TopLeft)
      .with_font(self.font.clone())
      .with_font_size(self.font_size)
      .with_text_color(self.color)
      .build_from_world_and_resources(world, resources);
    self.fps = Some(label);

    let (_, label) = UiLabelBuilder::<(), u32>::new(&"world_entries_count_label")
      .with_line_mode(LineMode::Wrap)
      .with_position(100., -100.)
      .with_size(400., 200.)
      .with_anchor(Anchor::TopLeft)
      .with_font(self.font.clone())
      .with_font_size(self.font_size)
      .with_text_color(self.color)
      .build_from_world_and_resources(world, resources);
    self.world_entries = Some(label);
  }

  fn disable(&self, _: &mut World, _: &mut Resources) {}

  fn on_update(&self, world: &mut World, resources: &mut Resources) {
    if self.fps.is_some() {
      let fps = resources.get::<FpsCounter>().unwrap().sampled_fps();
      if let Some(entry) = world.entry(self.fps.as_ref().unwrap().text_entity) {
        match entry.into_component_mut::<UiText>() {
          Ok(text) => text.text = format!("FPS: {:.*}", 2, fps),
          Err(_) => (),
        };
      }
    }

    if self.world_entries.is_some() {
      let world_len = world.len();
      if let Some(entry) = world.entry(self.world_entries.as_ref().unwrap().text_entity) {
        match entry.into_component_mut::<UiText>() {
          Ok(text) => text.text = format!("Entities: {:.*}", 2, world_len),
          Err(_) => (),
        };
      }
    }
  }
}
