use amethyst::ecs::{Component, VecStorage};

#[derive(Default, Clone)]
pub struct Scroller {
  pub speed: f32,
}

impl Component for Scroller {
  type Storage = VecStorage<Self>;
}
