use amethyst::ecs::{Component, VecStorage};

pub struct TileMap {
  pub layers: u32,
}

impl Component for TileMap {
  type Storage = VecStorage<Self>;
}
