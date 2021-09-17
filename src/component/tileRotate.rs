use amethyst::ecs::{Component, VecStorage};

#[derive(Clone)]
pub enum TileGenerators {
  icy,
}

#[derive(Clone)]
pub struct TileRotate {
  pub rotaionCycle: u32,
  pub tileGenerator: TileGenerators,
}

impl Component for TileRotate {
  type Storage = VecStorage<Self>;
}

impl Default for TileRotate {
  fn default() -> TileRotate {
    TileRotate {
      rotaionCycle: 0,
      tileGenerator: TileGenerators::icy,
    }
  }
}
