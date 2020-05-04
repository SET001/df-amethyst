use amethyst::ecs::{Component, VecStorage};

#[derive(Default)]
pub struct Scroller {
  speed: i32,
}

impl Component for Scroller {
  type Storage = VecStorage<Self>;
}
