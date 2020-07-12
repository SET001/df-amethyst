use amethyst::ecs::{Component, VecStorage};

#[derive(Default, Clone)]
pub struct Dimensions {
  pub height: f32,
  pub width: f32,
}

impl Component for Dimensions {
  type Storage = VecStorage<Self>;
}
