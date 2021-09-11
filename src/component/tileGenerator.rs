use amethyst::ecs::{Component, VecStorage};

pub struct TileGenerator {
  pub generator: u32,
}

impl Component for TileGenerator {
  type Storage = VecStorage<Self>;
}
