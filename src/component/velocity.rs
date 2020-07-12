use amethyst::ecs::{Component, VecStorage};

#[derive(Default, Clone)]
pub struct Velocity {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Component for Velocity {
  type Storage = VecStorage<Self>;
}
